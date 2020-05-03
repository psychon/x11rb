use std::cell::RefCell;

use crate::{defs, ResolveError};

/// Resolve references to other fields in the module.
pub(super) fn resolve(module: &defs::Module) -> Result<(), ResolveError> {
    for ns in module.namespaces.borrow().values() {
        for request_def in ns.request_defs.borrow().values() {
            resolve_field_refs_in_request(request_def)?;
        }
        for event_def in ns.event_defs.borrow().values() {
            if let defs::EventDef::Full(event_full_def) = event_def {
                resolve_field_refs_in_event(event_full_def)?;
            }
        }
        for error_def in ns.error_defs.borrow().values() {
            if let defs::ErrorDef::Full(error_full_def) = error_def {
                resolve_field_refs_in_error(error_full_def)?;
            }
        }
        for type_def in ns.type_defs.borrow().values() {
            if let defs::TypeDef::Struct(struct_def) = type_def {
                resolve_field_refs_in_struct(struct_def)?;
            }
        }
    }
    Ok(())
}

enum FieldRefResolveScope<'p, 'f, 'g> {
    Normal(NormalFieldRefResolveScope<'p, 'f, 'g>),
    ExprField(ExprFieldFieldRefResolveScope<'p, 'f, 'g>),
    SumOf(SumOfFieldRefResolveScope<'p, 'f, 'g>),
}

struct NormalFieldRefResolveScope<'p, 'f, 'g> {
    parent: Option<&'p NormalFieldRefResolveScope<'p, 'f, 'g>>,
    all_fields: &'f [defs::FieldDef],
    prev_fields: &'f [defs::FieldDef],
    ext_param_gatherer: &'g RefCell<ExternalParamGatherer>,
}

struct ExprFieldFieldRefResolveScope<'p, 'f, 'g> {
    parent: &'p NormalFieldRefResolveScope<'p, 'f, 'g>,
    all_fields: &'f [defs::FieldDef],
}

struct SumOfFieldRefResolveScope<'p, 'f, 'g> {
    parent: &'p NormalFieldRefResolveScope<'p, 'f, 'g>,
    struct_fields: &'f [defs::FieldDef],
}

impl FieldRefResolveScope<'_, '_, '_> {
    fn as_normal(&self) -> Option<&NormalFieldRefResolveScope<'_, '_, '_>> {
        match self {
            FieldRefResolveScope::Normal(normal_scope) => Some(normal_scope),
            _ => None,
        }
    }

    fn resolve_field_ref(&self, name: &str) -> Result<defs::ResolvedFieldRef, ResolveError> {
        match self {
            FieldRefResolveScope::Normal(normal_scope) => normal_scope.resolve_field_ref(name),
            FieldRefResolveScope::ExprField(expr_field_scope) => {
                for field in expr_field_scope.all_fields.iter() {
                    if field.name() == Some(name) {
                        let field_type = field
                            .value_type()
                            .ok_or_else(|| ResolveError::InvalidFieldRef(name.into()))?;
                        return Ok(defs::ResolvedFieldRef {
                            ref_kind: defs::FieldRefKind::LocalField,
                            field_type: field_type.type_.def.get().unwrap().clone(),
                        });
                    }
                }
                expr_field_scope.parent.resolve_field_ref(name)
            }
            FieldRefResolveScope::SumOf(sum_of_scope) => {
                for field in sum_of_scope.struct_fields.iter() {
                    if field.name() == Some(name) {
                        let field_type = field
                            .value_type()
                            .ok_or_else(|| ResolveError::InvalidFieldRef(name.into()))?;
                        return Ok(defs::ResolvedFieldRef {
                            ref_kind: defs::FieldRefKind::SumOfRef,
                            field_type: field_type.type_.def.get().unwrap().clone(),
                        });
                    }
                }
                sum_of_scope.parent.resolve_field_ref(name)
            }
        }
    }

    fn add_param_ref(&self, name: &str, type_: &defs::TypeRef) -> Result<(), ResolveError> {
        match self {
            FieldRefResolveScope::Normal(normal_scope) => normal_scope.add_param_ref(name, type_),
            FieldRefResolveScope::ExprField(expr_field_scope) => {
                expr_field_scope.parent.add_param_ref(name, type_)
            }
            FieldRefResolveScope::SumOf(sum_of_scope) => {
                sum_of_scope.parent.add_param_ref(name, type_)
            }
        }
    }
}

impl NormalFieldRefResolveScope<'_, '_, '_> {
    fn resolve_field_ref(&self, name: &str) -> Result<defs::ResolvedFieldRef, ResolveError> {
        let mut current_scope = self;
        let mut depth: usize = 0;

        let resolved = 'outer: loop {
            for field in current_scope.prev_fields {
                if field.name() == Some(name) {
                    let field_type = field
                        .value_type()
                        .ok_or_else(|| ResolveError::InvalidFieldRef(name.into()))?;
                    if depth == 0 {
                        break 'outer Some(defs::ResolvedFieldRef {
                            ref_kind: defs::FieldRefKind::LocalField,
                            field_type: field_type.type_.def.get().unwrap().clone(),
                        });
                    } else {
                        break 'outer Some(defs::ResolvedFieldRef {
                            ref_kind: defs::FieldRefKind::ExtParam,
                            field_type: field_type.type_.def.get().unwrap().clone(),
                        });
                    }
                }
            }
            if let Some(parent_scope) = current_scope.parent {
                current_scope = parent_scope;
                depth += 1;
            } else {
                break None;
            }
        };

        let resolved = resolved.ok_or_else(|| ResolveError::UnknownNameInFieldRef(name.into()))?;

        let mut current_scope = self;
        for _ in 0..depth {
            current_scope
                .ext_param_gatherer
                .borrow_mut()
                .add(name, &resolved.field_type)?;
            current_scope = current_scope.parent.unwrap();
        }

        Ok(resolved)
    }

    fn add_param_ref(&self, name: &str, type_: &defs::TypeRef) -> Result<(), ResolveError> {
        let mut current_scope = self;
        loop {
            current_scope
                .ext_param_gatherer
                .borrow_mut()
                .add(name, type_)?;
            if let Some(parent_scope) = current_scope.parent {
                current_scope = parent_scope;
            } else {
                break;
            }
        }
        Ok(())
    }
}

struct ExternalParamGatherer {
    vec: Vec<defs::ExternalParam>,
}

impl ExternalParamGatherer {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }

    fn add(&mut self, name: &str, type_: &defs::TypeRef) -> Result<(), ResolveError> {
        for ext_param in self.vec.iter() {
            if ext_param.name == *name {
                if !ext_param.type_.same_as(type_) {
                    return Err(ResolveError::DiscrepantParamRefTypes(name.into()));
                } else {
                    return Ok(());
                }
            }
        }
        self.vec.push(defs::ExternalParam {
            name: name.into(),
            type_: type_.clone(),
        });
        Ok(())
    }
}

fn resolve_field_refs_in_request(request_def: &defs::RequestDef) -> Result<(), ResolveError> {
    let ext_fields_gatherer = RefCell::new(ExternalParamGatherer::new());
    resolve_field_refs_in_fields(&request_def.fields.borrow(), None, &ext_fields_gatherer)?;
    assert!(ext_fields_gatherer.into_inner().vec.is_empty());

    if let Some(ref reply_def) = request_def.reply {
        resolve_field_refs_in_reply(reply_def)?;
    }
    Ok(())
}

fn resolve_field_refs_in_reply(reply_def: &defs::ReplyDef) -> Result<(), ResolveError> {
    let ext_fields_gatherer = RefCell::new(ExternalParamGatherer::new());
    resolve_field_refs_in_fields(&reply_def.fields.borrow(), None, &ext_fields_gatherer)?;
    assert!(ext_fields_gatherer.into_inner().vec.is_empty());
    Ok(())
}

fn resolve_field_refs_in_event(event_def: &defs::EventFullDef) -> Result<(), ResolveError> {
    let ext_fields_gatherer = RefCell::new(ExternalParamGatherer::new());
    resolve_field_refs_in_fields(&event_def.fields.borrow(), None, &ext_fields_gatherer)?;
    assert!(ext_fields_gatherer.into_inner().vec.is_empty());
    Ok(())
}

fn resolve_field_refs_in_error(error_def: &defs::ErrorFullDef) -> Result<(), ResolveError> {
    let ext_fields_gatherer = RefCell::new(ExternalParamGatherer::new());
    resolve_field_refs_in_fields(&error_def.fields.borrow(), None, &ext_fields_gatherer)?;
    assert!(ext_fields_gatherer.into_inner().vec.is_empty());
    Ok(())
}

fn resolve_field_refs_in_struct(struct_def: &defs::StructDef) -> Result<(), ResolveError> {
    let ext_fields_gatherer = RefCell::new(ExternalParamGatherer::new());
    resolve_field_refs_in_fields(&struct_def.fields.borrow(), None, &ext_fields_gatherer)?;
    // FIXME: Not needed
    struct_def
        .external_params
        .replace(ext_fields_gatherer.into_inner().vec);
    Ok(())
}

fn resolve_field_refs_in_fields(
    fields: &[defs::FieldDef],
    parent_scope: Option<&FieldRefResolveScope<'_, '_, '_>>,
    ext_param_gatherer: &RefCell<ExternalParamGatherer>,
) -> Result<(), ResolveError> {
    for (i, field) in fields.iter().enumerate() {
        let scope = FieldRefResolveScope::Normal(NormalFieldRefResolveScope {
            parent: parent_scope.map(|p| p.as_normal().unwrap()),
            prev_fields: &fields[..i],
            all_fields: fields,
            ext_param_gatherer,
        });
        resolve_field_refs_in_field(field, &scope)?;
    }
    Ok(())
}

fn resolve_field_refs_in_field(
    field: &defs::FieldDef,
    scope: &FieldRefResolveScope<'_, '_, '_>,
) -> Result<(), ResolveError> {
    match field {
        defs::FieldDef::Pad(_) => Ok(()),
        defs::FieldDef::Normal(normal_field) => {
            resolve_field_refs_in_type(normal_field.type_.type_.def.get().unwrap(), scope)?;
            Ok(())
        }
        defs::FieldDef::List(list_field) => {
            resolve_field_refs_in_type(list_field.element_type.type_.def.get().unwrap(), scope)?;
            if let Some(ref length_expr) = list_field.length_expr {
                resolve_field_refs_in_expr(length_expr, scope)?;
            }
            Ok(())
        }
        defs::FieldDef::Switch(switch_field) => {
            let switch_ext_fields_gatherer = RefCell::new(ExternalParamGatherer::new());
            let switch_scope = FieldRefResolveScope::Normal(NormalFieldRefResolveScope {
                parent: Some(scope.as_normal().unwrap()),
                prev_fields: &[],
                all_fields: &[],
                ext_param_gatherer: &switch_ext_fields_gatherer,
            });
            resolve_field_refs_in_expr(&switch_field.expr, &switch_scope)?;
            for switch_case in switch_field.cases.iter() {
                let case_ext_fields_gatherer = RefCell::new(ExternalParamGatherer::new());
                for case_expr in switch_case.exprs.iter() {
                    resolve_field_refs_in_expr(
                        case_expr,
                        &FieldRefResolveScope::Normal(NormalFieldRefResolveScope {
                            parent: Some(switch_scope.as_normal().unwrap()),
                            prev_fields: &[],
                            all_fields: &[],
                            ext_param_gatherer: &case_ext_fields_gatherer,
                        }),
                    )?;
                }
                resolve_field_refs_in_fields(
                    &switch_case.fields.borrow(),
                    Some(&switch_scope),
                    &case_ext_fields_gatherer,
                )?;
                switch_case
                    .external_params
                    .replace(case_ext_fields_gatherer.into_inner().vec);
            }
            switch_field
                .external_params
                .replace(switch_ext_fields_gatherer.into_inner().vec);
            Ok(())
        }
        defs::FieldDef::Fd(_) => Ok(()),
        defs::FieldDef::FdList(fd_list_field) => {
            resolve_field_refs_in_expr(&fd_list_field.length_expr, scope)?;
            Ok(())
        }
        defs::FieldDef::Expr(expr_field) => {
            let scope = scope.as_normal().unwrap();
            let scope2 = FieldRefResolveScope::ExprField(ExprFieldFieldRefResolveScope {
                parent: scope,
                all_fields: scope.all_fields,
            });
            resolve_field_refs_in_expr(&expr_field.expr, &scope2)?;
            Ok(())
        }
        defs::FieldDef::VirtualLen(_) => Ok(()),
    }
}

fn resolve_field_refs_in_type(
    type_: &defs::TypeRef,
    scope: &FieldRefResolveScope<'_, '_, '_>,
) -> Result<(), ResolveError> {
    match type_ {
        defs::TypeRef::Struct(struct_def) => {
            let struct_def = struct_def.upgrade().unwrap();
            for struct_ext_param in struct_def.external_params.borrow().iter() {
                let resolved = scope.resolve_field_ref(&struct_ext_param.name)?;
                if !resolved.field_type.same_as(&struct_ext_param.type_) {
                    return Err(ResolveError::DiscrepantParamRefTypes(
                        struct_ext_param.name.clone(),
                    ));
                }
            }
            Ok(())
        }
        defs::TypeRef::Alias(type_alias_def) => {
            let type_alias_def = type_alias_def.upgrade().unwrap();
            resolve_field_refs_in_type(type_alias_def.old_name.def.get().unwrap(), scope)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn resolve_field_refs_in_expr(
    expr: &defs::Expression,
    scope: &FieldRefResolveScope<'_, '_, '_>,
) -> Result<(), ResolveError> {
    match expr {
        defs::Expression::BinaryOp(bin_op_expr) => {
            resolve_field_refs_in_expr(&bin_op_expr.lhs, scope)?;
            resolve_field_refs_in_expr(&bin_op_expr.rhs, scope)?;
            Ok(())
        }
        defs::Expression::UnaryOp(unary_op_expr) => {
            resolve_field_refs_in_expr(&unary_op_expr.rhs, scope)?;
            Ok(())
        }
        defs::Expression::FieldRef(field_ref_expr) => {
            let resolved = scope.resolve_field_ref(&field_ref_expr.field_name)?;
            field_ref_expr.resolved.set(resolved).unwrap();
            Ok(())
        }
        defs::Expression::ParamRef(param_ref_expr) => {
            scope.add_param_ref(
                &param_ref_expr.field_name,
                param_ref_expr.type_.def.get().unwrap(),
            )?;
            Ok(())
        }
        defs::Expression::EnumRef(_) => Ok(()),
        defs::Expression::PopCount(operand_expr) => {
            resolve_field_refs_in_expr(operand_expr, scope)?;
            Ok(())
        }
        defs::Expression::SumOf(sum_of_expr) => {
            let resolved = scope.resolve_field_ref(&sum_of_expr.field_name)?;
            sum_of_expr.resolved_field.set(resolved).unwrap();

            if let Some(ref operand_expr) = sum_of_expr.operand {
                let resolved = sum_of_expr.resolved_field.get().unwrap();
                match resolved.field_type {
                    defs::TypeRef::Struct(ref struct_def) => {
                        let struct_def = struct_def.upgrade().unwrap();
                        let struct_fields = struct_def.fields.borrow();
                        let sum_of_scope = FieldRefResolveScope::SumOf(SumOfFieldRefResolveScope {
                            parent: scope.as_normal().unwrap(),
                            struct_fields: &struct_fields,
                        });
                        resolve_field_refs_in_expr(operand_expr, &sum_of_scope)?;
                    }
                    _ => {
                        let sum_of_scope = FieldRefResolveScope::SumOf(SumOfFieldRefResolveScope {
                            parent: scope.as_normal().unwrap(),
                            struct_fields: &[],
                        });
                        resolve_field_refs_in_expr(operand_expr, &sum_of_scope)?;
                    }
                }
            }
            Ok(())
        }
        defs::Expression::ListElementRef => Ok(()),
        defs::Expression::Value(_) => Ok(()),
        defs::Expression::Bit(_) => Ok(()),
    }
}
