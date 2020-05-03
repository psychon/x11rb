//! Parser for protocol XML descriptions.
//!
//! This module contains the [`Parser`] that allows parsing a [`roxmltree::Node`] into a
//! [`defs::Module`].

use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use once_cell::unsync::OnceCell;

use crate::defs;

/// An error that occurred while parsing an error.
#[derive(Debug)]
pub enum ParseError {
    /// The XML tree is in some way malformed.
    ///
    /// Possible errors include missing or duplicate tags, or tags that were not understood, among
    /// others.
    InvalidXml,

    /// The module already contains a namespace with this header name.
    RepeatedHeaderName,

    /// Some type was specified multiple times.
    ///
    /// An example for this error is two requests with the same name.
    RepeatedTypeName,

    /// A `<pad>` field is invalid.
    ///
    /// A `<pad>` must contain either a `bytes` or `align` attribute. This property was violated.
    InvalidPad,

    /// A `<pad align= >` has an invalid value (i.e., `align` is not
    /// a power of two).
    InvalidPadAlign,

    /// A `<field>` contains an invalid set of properties.
    ///
    /// At most one of the attributes `enum`, `altenum`, `mask`, or `altmask` may be present.
    /// However, some field had more than this.
    InvalidFieldValueSet,

    /// A `<required_start_align>` has an invalid value (i.e., `align` is not a power of two
    /// or `offset` is equal or greater than `align`).
    InvalidRequiredStartAlign,
}

/// A `Parser` that adds namespaces to a module.
///
/// One instance of this struct can be used to parse multiple namespaces, one after another.
pub struct Parser {
    module: Rc<defs::Module>,
}

impl Parser {
    /// Create a new parser that adds its information to the given module
    pub fn new(module: Rc<defs::Module>) -> Self {
        Self { module }
    }

    /// Parse an XML tree.
    ///
    /// This function parses the XML tree that is described by the given `node`. This `node` must
    /// describe a complete namespace, which means that it should correspond to the root tag of an
    /// XML document.
    ///
    /// The resulting namespace is added to the module that this parser was constructed for. A
    /// reference to the namespace is also returned.
    pub fn parse_namespace(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<Rc<defs::Namespace>, ParseError> {
        if !node.is_element() || !node.has_tag_name("xcb") {
            return Err(ParseError::InvalidXml);
        }

        let header = get_attr(node, "header")?;

        let ext_info = node
            .attribute("extension-xname")
            .map::<Result<defs::ExtInfo, ParseError>, _>(|xname| {
                let name = get_attr(node, "extension-name")?;
                let multiword = try_get_attr_parsed(node, "extension-multiword")?.unwrap_or(false);
                let major_version = get_attr_parsed(node, "major-version")?;
                let minor_version = get_attr_parsed(node, "minor-version")?;
                Ok(defs::ExtInfo {
                    xname: xname.into(),
                    name: name.into(),
                    multiword,
                    major_version,
                    minor_version,
                })
            })
            .transpose()?;

        let namespace = defs::Namespace::new(&self.module, header.into(), ext_info);
        if !self.module.insert_namespace(namespace.clone()) {
            return Err(ParseError::RepeatedHeaderName);
        }

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.tag_name().namespace().is_some() {
                return Err(ParseError::InvalidXml);
            }

            match child_node.tag_name().name() {
                "import" => {
                    let import_name = get_text(child_node)?;
                    namespace.add_import(import_name.into());
                }
                "request" => self.parse_request_def(child_node, &namespace)?,
                "event" => self.parse_event_full_def(child_node, &namespace)?,
                "eventcopy" => self.parse_event_copy_def(child_node, &namespace)?,
                "error" => self.parse_error_full_def(child_node, &namespace)?,
                "errorcopy" => self.parse_error_copy_def(child_node, &namespace)?,
                "struct" => self.parse_struct_def(child_node, &namespace)?,
                "union" => self.parse_union_def(child_node, &namespace)?,
                "eventstruct" => self.parse_event_struct_def(child_node, &namespace)?,
                "xidtype" => self.parse_xid_type_def(child_node, &namespace)?,
                "xidunion" => self.parse_xid_union_def(child_node, &namespace)?,
                "enum" => self.parse_enum_def(child_node, &namespace)?,
                "typedef" => self.parse_type_alias_def(child_node, &namespace)?,
                _ => return Err(ParseError::InvalidXml),
            }
        }

        Ok(namespace)
    }

    fn parse_request_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;
        let opcode = get_attr_parsed(node, "opcode")?;
        let combine_adjacent = try_get_attr_parsed(node, "combine-adjacent")?.unwrap_or(false);

        let mut required_start_align = None;
        let mut fields = Vec::new();
        let mut reply = None;
        let mut doc = None;

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.has_tag_name("required_start_align") {
                if required_start_align.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                required_start_align = Some(self.parse_required_start_align(child_node)?);
            } else if let Some(field) = self.try_parse_field_def(child_node)? {
                fields.push(field);
            } else if child_node.has_tag_name("reply") {
                if reply.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                reply = Some(self.parse_reply_def(child_node)?);
            } else if child_node.has_tag_name("doc") {
                if doc.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                doc = Some(self.parse_doc(child_node)?);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let request = Rc::new(defs::RequestDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            opcode,
            combine_adjacent,
            required_start_align,
            fields: RefCell::new(fields),
            reply,
            doc,
        });
        if let Some(ref reply) = request.reply {
            reply.request.set(Rc::downgrade(&request)).unwrap();
        }
        if !ns.insert_request_def(name.into(), request) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_reply_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<Rc<defs::ReplyDef>, ParseError> {
        assert!(node.is_element());

        let mut required_start_align = None;
        let mut fields = Vec::new();
        let mut doc = None;

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.has_tag_name("required_start_align") {
                if required_start_align.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                required_start_align = Some(self.parse_required_start_align(child_node)?);
            } else if let Some(field) = self.try_parse_field_def(child_node)? {
                fields.push(field);
            } else if child_node.has_tag_name("doc") {
                if doc.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                doc = Some(self.parse_doc(child_node)?);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        Ok(Rc::new(defs::ReplyDef {
            request: OnceCell::new(),
            required_start_align,
            fields: RefCell::new(fields),
            doc,
        }))
    }

    fn parse_event_full_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;
        let number = get_attr_parsed(node, "number")?;
        let no_sequence_number = try_get_attr_parsed(node, "no-sequence-number")?.unwrap_or(false);
        let xge = try_get_attr_parsed(node, "xge")?.unwrap_or(false);

        let mut required_start_align = None;
        let mut fields = Vec::new();
        let mut doc = None;

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.has_tag_name("required_start_align") {
                if required_start_align.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                required_start_align = Some(self.parse_required_start_align(child_node)?);
            } else if let Some(field) = self.try_parse_field_def(child_node)? {
                fields.push(field);
            } else if child_node.has_tag_name("doc") {
                if doc.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                doc = Some(self.parse_doc(child_node)?);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let event_full = Rc::new(defs::EventFullDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            number,
            no_sequence_number,
            xge,
            required_start_align,
            fields: RefCell::new(fields),
            doc,
        });
        if !ns.insert_event_def(name.into(), defs::EventDef::Full(event_full)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_event_copy_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;
        let number = get_attr_parsed(node, "number")?;
        let ref_ = get_attr(node, "ref")?;

        let event_copy = Rc::new(defs::EventCopyDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            number,
            ref_: defs::NamedEventRef::unresolved(ref_.into()),
        });
        if !ns.insert_event_def(name.into(), defs::EventDef::Copy(event_copy)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_error_full_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;
        let number = get_attr_parsed(node, "number")?;

        let mut required_start_align = None;
        let mut fields = Vec::new();

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.has_tag_name("required_start_align") {
                if required_start_align.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                required_start_align = Some(self.parse_required_start_align(child_node)?);
            } else if let Some(field) = self.try_parse_field_def(child_node)? {
                fields.push(field);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let error_full = Rc::new(defs::ErrorFullDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            number,
            required_start_align,
            fields: RefCell::new(fields),
        });
        if !ns.insert_error_def(name.into(), defs::ErrorDef::Full(error_full)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_error_copy_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;
        let number = get_attr_parsed(node, "number")?;
        let ref_ = get_attr(node, "ref")?;

        let error_copy = Rc::new(defs::ErrorCopyDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            number,
            ref_: defs::NamedErrorRef::unresolved(ref_.into()),
        });
        if !ns.insert_error_def(name.into(), defs::ErrorDef::Copy(error_copy)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_struct_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;

        let mut fields = Vec::new();

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if let Some(field) = self.try_parse_field_def(child_node)? {
                fields.push(field);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let struct_ = Rc::new(defs::StructDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            alignment: OnceCell::new(),
            fields: RefCell::new(fields),
            external_params: RefCell::new(Vec::new()),
        });
        if !ns.insert_type_def(name.into(), defs::TypeDef::Struct(struct_)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_union_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;

        let mut fields = Vec::new();

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if let Some(field) = self.try_parse_field_def(child_node)? {
                fields.push(field);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let union = Rc::new(defs::UnionDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            alignment: OnceCell::new(),
            fields,
        });
        if !ns.insert_type_def(name.into(), defs::TypeDef::Union(union)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_event_struct_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;

        let mut alloweds = Vec::new();

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.has_tag_name("allowed") {
                alloweds.push(self.parse_event_struct_allowed(child_node)?);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let event_struct = Rc::new(defs::EventStructDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            alloweds,
        });
        if !ns.insert_type_def(name.into(), defs::TypeDef::EventStruct(event_struct)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_event_struct_allowed(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<defs::EventStructAllowed, ParseError> {
        assert!(node.is_element());

        let extension = get_attr(node, "extension")?;
        let xge = get_attr_parsed(node, "xge")?;
        let opcode_min = get_attr_parsed(node, "opcode-min")?;
        let opcode_max = get_attr_parsed(node, "opcode-max")?;

        Ok(defs::EventStructAllowed {
            extension: extension.into(),
            xge,
            opcode_min,
            opcode_max,
            resolved: RefCell::new(Vec::new()),
        })
    }

    fn parse_xid_type_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;

        let xid_type = Rc::new(defs::XidTypeDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
        });
        if !ns.insert_type_def(name.into(), defs::TypeDef::Xid(xid_type)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_xid_union_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;
        let mut types = Vec::new();

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.has_tag_name("type") {
                let type_name = get_text(child_node)?;
                types.push(defs::NamedTypeRef::unresolved(type_name.into()));
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let xid_union = Rc::new(defs::XidUnionDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            types,
        });
        if !ns.insert_type_def(name.into(), defs::TypeDef::XidUnion(xid_union)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_enum_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;
        let mut items = Vec::new();
        let mut doc = None;

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.has_tag_name("item") {
                items.push(self.parse_enum_item(child_node)?);
            } else if child_node.has_tag_name("doc") {
                if doc.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                doc = Some(self.parse_doc(child_node)?);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let enum_ = Rc::new(defs::EnumDef {
            namespace: Rc::downgrade(ns),
            name: name.into(),
            items,
            doc,
        });
        if !ns.insert_type_def(name.into(), defs::TypeDef::Enum(enum_)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn parse_enum_item(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<defs::EnumItem, ParseError> {
        assert!(node.is_element());

        let name = get_attr(node, "name")?;

        let mut value = None;

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if child_node.has_tag_name("value") {
                if value.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                let v = get_text_parsed(child_node)?;
                value = Some(defs::EnumValue::Value(v));
            } else if child_node.has_tag_name("bit") {
                if value.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                let bit = get_text_parsed(child_node)?;
                if bit >= 32 {
                    return Err(ParseError::InvalidXml);
                }
                value = Some(defs::EnumValue::Bit(bit));
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        let value = value.ok_or_else(|| ParseError::InvalidXml)?;

        Ok(defs::EnumItem {
            name: name.into(),
            value,
        })
    }

    fn parse_type_alias_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
        ns: &Rc<defs::Namespace>,
    ) -> Result<(), ParseError> {
        assert!(node.is_element());

        let new_name = get_attr(node, "newname")?;
        let old_name = get_attr(node, "oldname")?;
        let old_name = defs::NamedTypeRef::unresolved(old_name.into());

        let type_alias = Rc::new(defs::TypeAliasDef {
            namespace: Rc::downgrade(ns),
            old_name,
            new_name: new_name.into(),
        });
        if !ns.insert_type_def(new_name.into(), defs::TypeDef::Alias(type_alias)) {
            Err(ParseError::RepeatedTypeName)
        } else {
            Ok(())
        }
    }

    fn try_parse_field_def(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<Option<defs::FieldDef>, ParseError> {
        assert!(node.is_element());

        if node.tag_name().namespace().is_some() {
            return Ok(None);
        }

        match node.tag_name().name() {
            "pad" => {
                let bytes = try_get_attr_parsed(node, "bytes")?;
                let align = try_get_attr_parsed::<u16>(node, "align")?;
                let serialize = try_get_attr_parsed(node, "serialize")?.unwrap_or(false);

                let pad_kind = match (bytes, align) {
                    (Some(bytes), None) => defs::PadKind::Bytes(bytes),
                    (None, Some(align)) => {
                        if !align.is_power_of_two() {
                            return Err(ParseError::InvalidPadAlign);
                        }
                        defs::PadKind::Align(align)
                    }
                    _ => return Err(ParseError::InvalidPad),
                };
                Ok(Some(defs::FieldDef::Pad(defs::PadField {
                    kind: pad_kind,
                    serialize,
                })))
            }
            "field" => {
                let name = get_attr(node, "name")?;
                let type_ = self.parse_field_value_type(node)?;

                Ok(Some(defs::FieldDef::Normal(defs::NormalField {
                    name: name.into(),
                    type_,
                })))
            }
            "list" => {
                let name = get_attr(node, "name")?;
                let element_type = self.parse_field_value_type(node)?;
                let length_expr = node
                    .first_element_child()
                    .map(|expr_node| self.parse_expression(expr_node))
                    .transpose()?;

                if element_type.type_.name == "fd" {
                    Ok(Some(defs::FieldDef::FdList(defs::FdListField {
                        name: name.into(),
                        length_expr: length_expr.ok_or_else(|| ParseError::InvalidXml)?,
                    })))
                } else {
                    Ok(Some(defs::FieldDef::List(defs::ListField {
                        name: name.into(),
                        element_type,
                        length_expr,
                    })))
                }
            }
            "switch" => {
                let name = get_attr(node, "name")?;

                let mut expr = None;
                let mut required_start_align = None;
                let mut cases = Vec::new();
                let mut kind = None;

                for child_node in node.children() {
                    if !child_node.is_element() {
                        continue;
                    }

                    if let Some(exp) = self.try_parse_expression(child_node)? {
                        if expr.is_some() {
                            return Err(ParseError::InvalidXml);
                        }
                        expr = Some(exp);
                    } else if child_node.has_tag_name("required_start_align") {
                        if required_start_align.is_some() {
                            return Err(ParseError::InvalidXml);
                        }
                        required_start_align = Some(self.parse_required_start_align(child_node)?);
                    } else if child_node.has_tag_name("case") {
                        if kind.is_none() {
                            kind = Some(defs::SwitchKind::Case);
                        } else if kind != Some(defs::SwitchKind::Case) {
                            return Err(ParseError::InvalidXml);
                        }
                        let case = self.try_parse_switch_case(child_node)?;
                        cases.push(case);
                    } else if child_node.has_tag_name("bitcase") {
                        if kind.is_none() {
                            kind = Some(defs::SwitchKind::BitCase);
                        } else if kind != Some(defs::SwitchKind::BitCase) {
                            return Err(ParseError::InvalidXml);
                        }
                        let case = self.try_parse_switch_case(child_node)?;
                        cases.push(case);
                    } else {
                        return Err(ParseError::InvalidXml);
                    }
                }

                let expr = expr.ok_or_else(|| ParseError::InvalidXml)?;
                let kind = kind.ok_or_else(|| ParseError::InvalidXml)?;

                Ok(Some(defs::FieldDef::Switch(defs::SwitchField {
                    name: name.into(),
                    expr,
                    required_start_align,
                    alignment: OnceCell::new(),
                    kind,
                    cases,
                    external_params: RefCell::new(Vec::new()),
                })))
            }
            "fd" => {
                let name = get_attr(node, "name")?;
                Ok(Some(defs::FieldDef::Fd(defs::FdField {
                    name: name.into(),
                })))
            }
            "exprfield" => {
                let name = get_attr(node, "name")?;
                let type_ = self.parse_field_value_type(node)?;
                let expr = self.parse_expression(
                    node.first_element_child()
                        .ok_or_else(|| ParseError::InvalidXml)?,
                )?;

                Ok(Some(defs::FieldDef::Expr(defs::ExprField {
                    name: name.into(),
                    type_,
                    expr,
                })))
            }
            _ => Ok(None),
        }
    }

    fn parse_required_start_align(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<defs::Alignment, ParseError> {
        assert!(node.is_element());

        let align = get_attr_parsed::<u32>(node, "align")?;
        let offset = try_get_attr_parsed(node, "offset")?.unwrap_or(0);
        if !align.is_power_of_two() || offset >= align {
            Err(ParseError::InvalidRequiredStartAlign)
        } else {
            Ok(defs::Alignment::new(align, offset))
        }
    }

    fn try_parse_switch_case(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<defs::SwitchCase, ParseError> {
        assert!(node.is_element());

        let name = node.attribute("name");

        let mut exprs = Vec::new();
        let mut required_start_align = None;
        let mut fields = Vec::new();

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }

            if let Some(expr) = self.try_parse_expression(child_node)? {
                exprs.push(expr);
            } else if child_node.has_tag_name("required_start_align") {
                if required_start_align.is_some() {
                    return Err(ParseError::InvalidXml);
                }
                required_start_align = Some(self.parse_required_start_align(child_node)?);
            } else if let Some(field) = self.try_parse_field_def(child_node)? {
                fields.push(field);
            } else {
                return Err(ParseError::InvalidXml);
            }
        }

        Ok(defs::SwitchCase {
            name: name.map(String::from),
            exprs,
            required_start_align,
            alignment: OnceCell::new(),
            fields: RefCell::new(fields),
            external_params: RefCell::new(Vec::new()),
        })
    }

    fn parse_field_value_type(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<defs::FieldValueType, ParseError> {
        assert!(node.is_element());

        let type_ = get_attr(node, "type")?;
        let enum_ = node.attribute("enum");
        let altenum = node.attribute("altenum");
        let mask = node.attribute("mask");
        let altmask = node.attribute("altmask");

        let value_set = match (enum_, altenum, mask, altmask) {
            (None, None, None, None) => defs::FieldValueSet::None,
            (Some(enum_), None, None, None) => {
                defs::FieldValueSet::Enum(defs::NamedTypeRef::unresolved(enum_.into()))
            }
            (None, Some(altenum), None, None) => {
                defs::FieldValueSet::AltEnum(defs::NamedTypeRef::unresolved(altenum.into()))
            }
            (None, None, Some(mask), None) => {
                defs::FieldValueSet::Mask(defs::NamedTypeRef::unresolved(mask.into()))
            }
            (None, None, None, Some(altmask)) => {
                defs::FieldValueSet::AltMask(defs::NamedTypeRef::unresolved(altmask.into()))
            }
            _ => return Err(ParseError::InvalidFieldValueSet),
        };

        Ok(defs::FieldValueType {
            type_: defs::NamedTypeRef::unresolved(type_.into()),
            value_set,
        })
    }

    fn parse_expression(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<defs::Expression, ParseError> {
        self.try_parse_expression(node)
            .transpose()
            .ok_or_else(|| ParseError::InvalidXml)?
    }

    fn try_parse_expression(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<Option<defs::Expression>, ParseError> {
        assert!(node.is_element());

        if node.tag_name().namespace().is_some() {
            return Ok(None);
        }

        match node.tag_name().name() {
            "op" => {
                let operator = match get_attr(node, "op")? {
                    "+" => defs::BinaryOperator::Add,
                    "-" => defs::BinaryOperator::Sub,
                    "*" => defs::BinaryOperator::Mul,
                    "/" => defs::BinaryOperator::Div,
                    "&" => defs::BinaryOperator::And,
                    "<<" => defs::BinaryOperator::Shl,
                    _ => return Err(ParseError::InvalidXml),
                };

                let operands = self.parse_expression_list(node)?;
                if operands.len() != 2 {
                    return Err(ParseError::InvalidXml);
                }
                let mut operand_iter = operands.into_iter();
                let lhs = Box::new(operand_iter.next().unwrap());
                let rhs = Box::new(operand_iter.next().unwrap());

                Ok(Some(defs::Expression::BinaryOp(defs::BinaryOpExpr {
                    operator,
                    lhs,
                    rhs,
                })))
            }
            "unop" => {
                let operator = match get_attr(node, "op")? {
                    "~" => defs::UnaryOperator::Not,
                    _ => return Err(ParseError::InvalidXml),
                };

                let operands = self.parse_expression_list(node)?;
                if operands.len() != 1 {
                    return Err(ParseError::InvalidXml);
                }
                let mut operand_iter = operands.into_iter();
                let rhs = Box::new(operand_iter.next().unwrap());

                Ok(Some(defs::Expression::UnaryOp(defs::UnaryOpExpr {
                    operator,
                    rhs,
                })))
            }
            "fieldref" => {
                let name = get_text(node)?;
                Ok(Some(defs::Expression::FieldRef(defs::FieldRefExpr {
                    field_name: name.into(),
                    resolved: OnceCell::new(),
                })))
            }
            "paramref" => {
                let name = get_text(node)?;
                let type_ = get_attr(node, "type")?;
                Ok(Some(defs::Expression::ParamRef(defs::ParamRefExpr {
                    field_name: name.into(),
                    type_: defs::NamedTypeRef::unresolved(type_.into()),
                })))
            }
            "enumref" => {
                let ref_ = get_attr(node, "ref")?;
                let variant = get_text(node)?;
                Ok(Some(defs::Expression::EnumRef(defs::EnumRefExpr {
                    enum_: defs::NamedTypeRef::unresolved(ref_.into()),
                    variant: variant.into(),
                })))
            }
            "popcount" => {
                let operands = self.parse_expression_list(node)?;
                if operands.len() != 1 {
                    return Err(ParseError::InvalidXml);
                }
                let mut operand_iter = operands.into_iter();
                let operand = Box::new(operand_iter.next().unwrap());

                Ok(Some(defs::Expression::PopCount(operand)))
            }
            "sumof" => {
                let ref_ = get_attr(node, "ref")?;

                let operands = self.parse_expression_list(node)?;
                if operands.len() > 1 {
                    return Err(ParseError::InvalidXml);
                }
                let mut operand_iter = operands.into_iter();
                let operand = operand_iter.next().map(Box::new);

                Ok(Some(defs::Expression::SumOf(defs::SumOfExpr {
                    field_name: ref_.into(),
                    resolved_field: OnceCell::new(),
                    operand,
                })))
            }
            "listelement-ref" => Ok(Some(defs::Expression::ListElementRef)),
            "value" => {
                let value = get_text_parsed(node)?;
                Ok(Some(defs::Expression::Value(value)))
            }
            "bit" => {
                let bit = get_text_parsed(node)?;
                if bit >= 32 {
                    return Err(ParseError::InvalidXml);
                }
                Ok(Some(defs::Expression::Bit(bit)))
            }
            _ => Ok(None),
        }
    }

    fn parse_expression_list(
        &mut self,
        node: roxmltree::Node<'_, '_>,
    ) -> Result<Vec<defs::Expression>, ParseError> {
        let mut exprs = Vec::new();
        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }
            exprs.push(self.parse_expression(child_node)?);
        }
        Ok(exprs)
    }

    fn parse_doc(&mut self, node: roxmltree::Node<'_, '_>) -> Result<defs::Doc, ParseError> {
        assert!(node.is_element());

        let mut brief = None;
        let mut description = None;
        let mut example = None;
        let mut fields = Vec::new();
        let mut errors = Vec::new();
        let mut sees = Vec::new();

        for child_node in node.children() {
            if !child_node.is_element() {
                continue;
            }
            if child_node.tag_name().namespace().is_some() {
                return Err(ParseError::InvalidXml);
            }

            match child_node.tag_name().name() {
                "brief" => {
                    if brief.is_some() {
                        return Err(ParseError::InvalidXml);
                    }
                    brief = Some(get_text(child_node)?.into());
                }
                "description" => {
                    if description.is_some() {
                        return Err(ParseError::InvalidXml);
                    }
                    description = Some(get_text(child_node)?.into());
                }
                "example" => {
                    if example.is_some() {
                        return Err(ParseError::InvalidXml);
                    }
                    example = Some(get_text(child_node)?.into());
                }
                "field" => {
                    let name = get_attr(child_node, "name")?;
                    let doc = try_get_text(child_node)?;
                    fields.push(defs::FieldDoc {
                        name: name.into(),
                        doc: doc.map(Into::into),
                    });
                }
                "error" => {
                    let type_ = get_attr(child_node, "type")?;
                    let doc = try_get_text(child_node)?;
                    errors.push(defs::ErrorDoc {
                        type_: type_.into(),
                        doc: doc.map(Into::into),
                    });
                }
                "see" => {
                    let type_ = get_attr(child_node, "type")?;
                    let name = get_attr(child_node, "name")?;
                    sees.push(defs::SeeDoc {
                        type_: type_.into(),
                        name: name.into(),
                    });
                }
                _ => return Err(ParseError::InvalidXml),
            }
        }

        Ok(defs::Doc {
            brief,
            description,
            example,
            fields,
            errors,
            sees,
        })
    }
}

fn get_attr<'a>(node: roxmltree::Node<'a, '_>, name: &str) -> Result<&'a str, ParseError> {
    node.attribute(name).ok_or(ParseError::InvalidXml)
}

fn get_attr_parsed<T: FromStr>(node: roxmltree::Node<'_, '_>, name: &str) -> Result<T, ParseError> {
    try_get_attr_parsed(node, name)?.ok_or(ParseError::InvalidXml)
}

fn try_get_attr_parsed<T: FromStr>(
    node: roxmltree::Node<'_, '_>,
    name: &str,
) -> Result<Option<T>, ParseError> {
    node.attribute(name)
        .map(FromStr::from_str)
        .transpose()
        .map_err(|_| ParseError::InvalidXml)
}

fn get_text<'a>(node: roxmltree::Node<'a, '_>) -> Result<&'a str, ParseError> {
    try_get_text(node)?.ok_or(ParseError::InvalidXml)
}

fn try_get_text<'a>(node: roxmltree::Node<'a, '_>) -> Result<Option<&'a str>, ParseError> {
    let mut text = None;
    for child_node in node.children() {
        if child_node.is_comment() {
            continue;
        }
        if !child_node.is_text() {
            return Err(ParseError::InvalidXml);
        }
        if text.is_some() {
            return Err(ParseError::InvalidXml);
        }
        text = Some(child_node.text().unwrap());
    }
    Ok(text)
}

fn get_text_parsed<T: FromStr>(node: roxmltree::Node<'_, '_>) -> Result<T, ParseError> {
    FromStr::from_str(get_text(node)?).map_err(|_| ParseError::InvalidXml)
}
