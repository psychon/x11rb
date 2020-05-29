use crate::{defs, ResolveError};

/// Resolve references to types.
///
/// This function looks at named references to other types in the module and adds a reference to
/// the actual type.
#[inline]
pub(super) fn resolve(module: &defs::Module) -> Result<(), ResolveError> {
    TypeResolver::new(module).resolve()
}

struct TypeResolver<'a> {
    module: &'a defs::Module,
}

impl<'a> TypeResolver<'a> {
    #[inline]
    fn new(module: &'a defs::Module) -> Self {
        TypeResolver { module }
    }

    fn resolve(&mut self) -> Result<(), ResolveError> {
        let namespaces = self.module.namespaces.borrow();

        // Resolve types
        for ns in namespaces.values() {
            for request_def in ns.request_defs.borrow().values() {
                self.resolve_request_def(request_def, ns)?;
            }
            for event_def in ns.event_defs.borrow().values() {
                self.resolve_event_def(event_def, ns)?;
            }
            for error_def in ns.error_defs.borrow().values() {
                self.resolve_error_def(error_def, ns)?;
            }
            for type_def in ns.type_defs.borrow().values() {
                self.resolve_type_def(type_def, ns)?;
            }
        }

        Ok(())
    }

    fn resolve_request_def(
        &mut self,
        request_def: &defs::RequestDef,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        let fields = request_def.fields.borrow();
        Self::check_repeated_fields(&fields)?;
        for field in fields.iter() {
            self.resolve_field(field, ns)?;
        }
        if let Some(ref reply) = request_def.reply {
            self.resolve_reply_def(reply, ns)?;
        }
        Ok(())
    }

    fn resolve_reply_def(
        &mut self,
        reply_def: &defs::ReplyDef,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        let fields = reply_def.fields.borrow();
        Self::check_repeated_fields(&fields)?;
        for field in fields.iter() {
            self.resolve_field(field, ns)?;
        }
        Ok(())
    }

    fn resolve_event_def(
        &mut self,
        event_def: &defs::EventDef,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        match event_def {
            defs::EventDef::Full(event_full_def) => {
                let fields = event_full_def.fields.borrow();
                Self::check_repeated_fields(&fields)?;
                for field in fields.iter() {
                    self.resolve_field(field, ns)?;
                }
                Ok(())
            }
            defs::EventDef::Copy(event_copy_def) => {
                self.resolve_named_event(&event_copy_def.ref_, ns)?;
                Ok(())
            }
        }
    }

    fn resolve_error_def(
        &mut self,
        error_def: &defs::ErrorDef,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        match error_def {
            defs::ErrorDef::Full(error_full_def) => {
                let fields = error_full_def.fields.borrow();
                Self::check_repeated_fields(&fields)?;
                for field in fields.iter() {
                    self.resolve_field(field, ns)?;
                }
                Ok(())
            }
            defs::ErrorDef::Copy(error_copy_def) => {
                self.resolve_named_error(&error_copy_def.ref_, ns)?;
                Ok(())
            }
        }
    }

    fn resolve_type_def(
        &mut self,
        type_def: &defs::TypeDef,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        match type_def {
            defs::TypeDef::Struct(struct_def) => {
                let fields = struct_def.fields.borrow();
                Self::check_repeated_fields(&fields)?;
                for field in fields.iter() {
                    self.resolve_field(field, ns)?;
                }
                Ok(())
            }
            defs::TypeDef::Union(union_def) => {
                Self::check_repeated_fields(&union_def.fields)?;
                for field in union_def.fields.iter() {
                    self.resolve_field(field, ns)?;
                }
                Ok(())
            }
            defs::TypeDef::EventStruct(event_struct_def) => {
                for allowed in event_struct_def.alloweds.iter() {
                    self.resolve_event_struct_allowed(event_struct_def, allowed)?;
                }
                Ok(())
            }
            defs::TypeDef::Xid(_) => Ok(()),
            defs::TypeDef::XidUnion(xid_union_def) => {
                for type_ in xid_union_def.types.iter() {
                    self.resolve_named_type(type_, ns, false)?;
                }
                Ok(())
            }
            defs::TypeDef::Enum(enum_def) => {
                for (i, enum_item) in enum_def.items.iter().enumerate() {
                    if enum_def.items[..i]
                        .iter()
                        .any(|enum_item2| enum_item2.name == enum_item.name)
                    {
                        return Err(ResolveError::RepeatedEnumValueName(enum_item.name.clone()));
                    }
                }
                Ok(())
            }
            defs::TypeDef::Alias(type_alias_def) => {
                self.resolve_named_type(&type_alias_def.old_name, ns, false)?;
                Ok(())
            }
        }
    }

    fn check_repeated_fields(fields: &[defs::FieldDef]) -> Result<(), ResolveError> {
        for (i, field) in fields.iter().enumerate() {
            if let Some(field_name) = field.name() {
                if fields[..i]
                    .iter()
                    .any(|field2| field2.name().as_ref() == Some(&field_name))
                {
                    return Err(ResolveError::RepeatedFieldName(field_name.into()));
                }
            }
        }
        Ok(())
    }

    fn resolve_field(
        &mut self,
        field: &defs::FieldDef,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        match field {
            defs::FieldDef::Pad(_) => Ok(()),
            defs::FieldDef::Normal(normal_field) => {
                self.resolve_field_value_type(&normal_field.type_, ns)?;
                Ok(())
            }
            defs::FieldDef::List(list_field) => {
                self.resolve_field_value_type(&list_field.element_type, ns)?;
                if let Some(ref length_expr) = list_field.length_expr {
                    self.resolve_expr(length_expr, ns)?;
                }
                Ok(())
            }
            defs::FieldDef::Switch(switch_field) => {
                self.resolve_expr(&switch_field.expr, ns)?;
                for case in switch_field.cases.iter() {
                    for case_expr in case.exprs.iter() {
                        self.resolve_expr(case_expr, ns)?;
                    }
                    for case_field in case.fields.borrow().iter() {
                        self.resolve_field(case_field, ns)?;
                    }
                }
                Ok(())
            }
            defs::FieldDef::Fd(_) => Ok(()),
            defs::FieldDef::FdList(fd_list_field) => {
                self.resolve_expr(&fd_list_field.length_expr, ns)?;
                Ok(())
            }
            defs::FieldDef::Expr(expr_field) => {
                self.resolve_field_value_type(&expr_field.type_, ns)?;
                self.resolve_expr(&expr_field.expr, ns)?;
                Ok(())
            }
            defs::FieldDef::VirtualLen(_) => {
                unreachable!("virtual_len field not expected at this point");
            }
        }
    }

    fn resolve_field_value_type(
        &mut self,
        type_: &defs::FieldValueType,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        self.resolve_named_type(&type_.type_, ns, false)?;
        match type_.value_set {
            defs::FieldValueSet::None => Ok(()),
            defs::FieldValueSet::Enum(ref enum_) => {
                self.resolve_named_type(enum_, ns, true)?;
                Ok(())
            }
            defs::FieldValueSet::AltEnum(ref altenum) => {
                self.resolve_named_type(altenum, ns, true)?;
                Ok(())
            }
            defs::FieldValueSet::Mask(ref mask) => {
                self.resolve_named_type(mask, ns, true)?;
                Ok(())
            }
            defs::FieldValueSet::AltMask(ref altmask) => {
                self.resolve_named_type(altmask, ns, true)?;
                Ok(())
            }
        }
    }

    fn resolve_event_struct_allowed(
        &mut self,
        event_struct_def: &defs::EventStructDef,
        allowed: &defs::EventStructAllowed,
    ) -> Result<(), ResolveError> {
        let mut resolved = allowed.resolved.borrow_mut();

        let module = event_struct_def
            .namespace
            .upgrade()
            .unwrap()
            .module
            .upgrade()
            .unwrap();

        let ext_ns = module
            .get_namespace_by_ext_name(&allowed.extension)
            .ok_or_else(|| ResolveError::UnknownExtNameInEventStruct(allowed.extension.clone()))?;

        for number in allowed.opcode_min..=allowed.opcode_max {
            let event_def = ext_ns
                .get_event_by_number(number, allowed.xge)
                .ok_or_else(|| {
                    ResolveError::UnknownEventNumberEventStruct(
                        allowed.extension.clone(),
                        number,
                        allowed.xge,
                    )
                })?;

            resolved.push(event_def.as_event_ref());
        }

        Ok(())
    }

    fn resolve_expr(
        &mut self,
        expr: &defs::Expression,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        match expr {
            defs::Expression::BinaryOp(bin_op_expr) => {
                self.resolve_expr(&bin_op_expr.lhs, ns)?;
                self.resolve_expr(&bin_op_expr.rhs, ns)?;
                Ok(())
            }
            defs::Expression::UnaryOp(unary_op_expr) => {
                self.resolve_expr(&unary_op_expr.rhs, ns)?;
                Ok(())
            }
            defs::Expression::FieldRef(_) => Ok(()),
            defs::Expression::ParamRef(param_ref_expr) => {
                self.resolve_named_type(&param_ref_expr.type_, ns, false)?;
                Ok(())
            }
            defs::Expression::EnumRef(enum_ref_expr) => {
                self.resolve_named_type(&enum_ref_expr.enum_, ns, true)?;
                Ok(())
            }
            defs::Expression::PopCount(expr) => {
                self.resolve_expr(expr, ns)?;
                Ok(())
            }
            defs::Expression::SumOf(sum_of_expr) => {
                self.resolve_expr(&sum_of_expr.operand, ns)?;
                Ok(())
            }
            defs::Expression::ListElementRef => Ok(()),
            defs::Expression::Value(_) => Ok(()),
            defs::Expression::Bit(_) => Ok(()),
        }
    }

    fn resolve_named_event(
        &mut self,
        named_event: &defs::NamedEventRef,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        let (header, name) = Self::parse_type_name(named_event.name())?;
        if let Some(header) = header {
            if header == ns.header {
                let event = ns
                    .get_event(&name)
                    .ok_or_else(|| ResolveError::UnknownEventName(named_event.name().into()))?;
                named_event.set_resolved(event);
            } else {
                let imported_ns = ns
                    .get_import(&header)
                    .ok_or_else(|| ResolveError::UnknownEventName(named_event.name().into()))?;
                let event = imported_ns
                    .get_event(&name)
                    .ok_or_else(|| ResolveError::UnknownEventName(named_event.name().into()))?;
                named_event.set_resolved(event);
            }
            Ok(())
        } else {
            let event = if let Some(event) = ns.get_event(&name) {
                event
            } else {
                let mut event = None;
                for imported_ns in ns.imports.borrow().values().map(defs::Import::ns) {
                    if let Some(event_in_import) = imported_ns.get_event(&name) {
                        if event.is_some() {
                            return Err(ResolveError::AmbiguousEventName(
                                named_event.name().into(),
                            ));
                        }
                        event = Some(event_in_import);
                    }
                }
                event.ok_or_else(|| ResolveError::UnknownEventName(named_event.name().into()))?
            };
            named_event.set_resolved(event);
            Ok(())
        }
    }

    fn resolve_named_error(
        &mut self,
        named_error: &defs::NamedErrorRef,
        ns: &defs::Namespace,
    ) -> Result<(), ResolveError> {
        let (header, name) = Self::parse_type_name(named_error.name())?;
        if let Some(header) = header {
            if header == ns.header {
                let error = ns
                    .get_error(&name)
                    .ok_or_else(|| ResolveError::UnknownErrorName(named_error.name().into()))?;
                named_error.set_resolved(error);
            } else {
                let imported_ns = ns
                    .get_import(&header)
                    .ok_or_else(|| ResolveError::UnknownErrorName(named_error.name().into()))?;
                let error = imported_ns
                    .get_error(&name)
                    .ok_or_else(|| ResolveError::UnknownErrorName(named_error.name().into()))?;
                named_error.set_resolved(error);
            }
            Ok(())
        } else {
            let error = if let Some(error) = ns.get_error(&name) {
                error
            } else {
                let mut error = None;
                for imported_ns in ns.imports.borrow().values().map(defs::Import::ns) {
                    if let Some(error_in_import) = imported_ns.get_error(&name) {
                        if error.is_some() {
                            return Err(ResolveError::AmbiguousErrorName(
                                named_error.name().into(),
                            ));
                        }
                        error = Some(error_in_import);
                    }
                }
                error.ok_or_else(|| ResolveError::UnknownErrorName(named_error.name().into()))?
            };
            named_error.set_resolved(error);
            Ok(())
        }
    }

    fn resolve_named_type(
        &mut self,
        named_type: &defs::NamedTypeRef,
        ns: &defs::Namespace,
        can_be_enum: bool,
    ) -> Result<(), ResolveError> {
        let (header, name) = Self::parse_type_name(named_type.name())?;
        if let Some(header) = header {
            if header == ns.header {
                let type_ = ns
                    .get_type(&name)
                    .ok_or_else(|| ResolveError::UnknownTypeName(named_type.name().into()))?;
                named_type.set_resolved(type_);
            } else {
                let imported_ns = ns
                    .get_import(&header)
                    .ok_or_else(|| ResolveError::UnknownTypeName(named_type.name().into()))?;
                let type_ = imported_ns
                    .get_type(&name)
                    .ok_or_else(|| ResolveError::UnknownTypeName(named_type.name().into()))?;
                named_type.set_resolved(type_);
            }
        } else {
            let type_ = match name {
                "CARD8" => defs::TypeRef::BuiltIn(defs::BuiltInType::Card8),
                "CARD16" => defs::TypeRef::BuiltIn(defs::BuiltInType::Card16),
                "CARD32" => defs::TypeRef::BuiltIn(defs::BuiltInType::Card32),
                "CARD64" => defs::TypeRef::BuiltIn(defs::BuiltInType::Card64),
                "INT8" => defs::TypeRef::BuiltIn(defs::BuiltInType::Int8),
                "INT16" => defs::TypeRef::BuiltIn(defs::BuiltInType::Int16),
                "INT32" => defs::TypeRef::BuiltIn(defs::BuiltInType::Int32),
                "INT64" => defs::TypeRef::BuiltIn(defs::BuiltInType::Int64),
                "BYTE" => defs::TypeRef::BuiltIn(defs::BuiltInType::Byte),
                "BOOL" => defs::TypeRef::BuiltIn(defs::BuiltInType::Bool),
                "char" => defs::TypeRef::BuiltIn(defs::BuiltInType::Char),
                "float" => defs::TypeRef::BuiltIn(defs::BuiltInType::Float),
                "double" => defs::TypeRef::BuiltIn(defs::BuiltInType::Double),
                "void" => defs::TypeRef::BuiltIn(defs::BuiltInType::Void),
                _ => {
                    if let Some(type_) = ns.get_type(&name) {
                        type_
                    } else {
                        let mut type_ = None;
                        for imported_ns in ns.imports.borrow().values().map(defs::Import::ns) {
                            if let Some(type_in_import) = imported_ns.get_type(&name) {
                                if type_.is_some() {
                                    return Err(ResolveError::AmbiguousTypeName(
                                        named_type.name().into(),
                                    ));
                                }
                                type_ = Some(type_in_import);
                            }
                        }
                        type_.ok_or_else(|| {
                            ResolveError::UnknownTypeName(named_type.name().into())
                        })?
                    }
                }
            };
            named_type.set_resolved(type_);
        }

        if !can_be_enum {
            if let defs::TypeRef::Enum(enum_def) = named_type.get_resolved() {
                let enum_def = enum_def.upgrade().unwrap();
                return Err(ResolveError::InvalidUseOfEnum(enum_def.name.clone()));
            }
        }

        Ok(())
    }

    fn parse_type_name(name: &str) -> Result<(Option<&str>, &str), ResolveError> {
        if name.is_empty() {
            return Err(ResolveError::InvalidTypeName(name.into()));
        }

        let mut splitted = name.split(':');
        let split1 = splitted.next().unwrap();
        if let Some(split2) = splitted.next() {
            if splitted.next().is_some() {
                return Err(ResolveError::InvalidTypeName(name.into()));
            }
            Ok((Some(split1), split2))
        } else {
            Ok((None, name))
        }
    }
}
