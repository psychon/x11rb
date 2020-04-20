use crate::{defs, ResolveError};

pub(super) fn check(module: &defs::Module) -> Result<(), ResolveError> {
    for ns in module.namespaces.borrow().values() {
        for request_def in ns.request_defs.borrow().values() {
            check_alignment_in_request_def(request_def)?;
        }
        for event_def in ns.event_defs.borrow().values() {
            check_alignment_in_event_def(event_def)?;
        }
        for error_def in ns.error_defs.borrow().values() {
            check_alignment_in_error_def(error_def)?;
        }
        for type_def in ns.type_defs.borrow().values() {
            check_alignment_in_type_def(type_def)?;
        }
    }
    Ok(())
}

fn check_alignment_in_request_def(request_def: &defs::RequestDef) -> Result<(), ResolveError> {
    let req_start_align = if let Some(required_start_align) = request_def.required_start_align {
        check_required_start_align(required_start_align)?;
        // there cannot be an offset here
        if required_start_align.offset != 0 {
            return Err(ResolveError::InvalidRequiredStartAlign(
                required_start_align.align,
                required_start_align.offset,
            ));
        }
        required_start_align
    } else {
        defs::Alignment {
            align: 4,
            offset: 0,
        }
    };

    let fields = request_def.fields.borrow();
    let mut field_aligns = Vec::with_capacity(fields.len());
    for field in fields.iter() {
        field_aligns.push(resolve_field_alignment(field)?);
    }
    check_struct_fields_alignment(req_start_align, &field_aligns)
        .ok_or_else(|| ResolveError::UnaligneableRequest(request_def.name.clone()))?;

    if let Some(ref reply) = request_def.reply {
        check_alignment_in_reply_def(reply, &request_def.name)?;
    }

    Ok(())
}

fn check_alignment_in_reply_def(
    reply_def: &defs::ReplyDef,
    request_name: &str,
) -> Result<(), ResolveError> {
    let req_start_align = if let Some(required_start_align) = reply_def.required_start_align {
        check_required_start_align(required_start_align)?;
        // there cannot be an offset here
        if required_start_align.offset != 0 {
            return Err(ResolveError::InvalidRequiredStartAlign(
                required_start_align.align,
                required_start_align.offset,
            ));
        }
        required_start_align
    } else {
        defs::Alignment {
            align: 4,
            offset: 0,
        }
    };

    let fields = reply_def.fields.borrow();
    let mut field_aligns = Vec::with_capacity(fields.len());
    for field in fields.iter() {
        field_aligns.push(resolve_field_alignment(field)?);
    }
    check_struct_fields_alignment(req_start_align, &field_aligns)
        .ok_or_else(|| ResolveError::UnaligneableReply(request_name.into()))?;

    Ok(())
}

fn check_alignment_in_event_def(event_def: &defs::EventDef) -> Result<(), ResolveError> {
    match event_def {
        defs::EventDef::Full(event_full_def) => {
            let req_start_align =
                if let Some(required_start_align) = event_full_def.required_start_align {
                    check_required_start_align(required_start_align)?;
                    // there cannot be an offset here
                    if required_start_align.offset != 0 {
                        return Err(ResolveError::InvalidRequiredStartAlign(
                            required_start_align.align,
                            required_start_align.offset,
                        ));
                    }
                    required_start_align
                } else {
                    defs::Alignment {
                        align: 4,
                        offset: 0,
                    }
                };

            let fields = event_full_def.fields.borrow();
            let mut field_aligns = Vec::with_capacity(fields.len());
            for field in fields.iter() {
                field_aligns.push(resolve_field_alignment(field)?);
            }
            check_struct_fields_alignment(req_start_align, &field_aligns)
                .ok_or_else(|| ResolveError::UnaligneableEvent(event_full_def.name.clone()))?;

            Ok(())
        }
        defs::EventDef::Copy(_) => Ok(()),
    }
}

fn check_alignment_in_error_def(error_def: &defs::ErrorDef) -> Result<(), ResolveError> {
    match error_def {
        defs::ErrorDef::Full(error_full_def) => {
            let req_start_align =
                if let Some(required_start_align) = error_full_def.required_start_align {
                    check_required_start_align(required_start_align)?;
                    // there cannot be an offset here
                    if required_start_align.offset != 0 {
                        return Err(ResolveError::InvalidRequiredStartAlign(
                            required_start_align.align,
                            required_start_align.offset,
                        ));
                    }
                    required_start_align
                } else {
                    defs::Alignment {
                        align: 4,
                        offset: 0,
                    }
                };

            let fields = error_full_def.fields.borrow();
            let mut field_aligns = Vec::with_capacity(fields.len());
            for field in fields.iter() {
                field_aligns.push(resolve_field_alignment(field)?);
            }
            check_struct_fields_alignment(req_start_align, &field_aligns)
                .ok_or_else(|| ResolveError::UnaligneableError(error_full_def.name.clone()))?;

            Ok(())
        }
        defs::ErrorDef::Copy(_) => Ok(()),
    }
}

fn check_alignment_in_type_def(type_def: &defs::TypeDef) -> Result<(), ResolveError> {
    match type_def {
        defs::TypeDef::Struct(struct_def) => resolve_struct_alignment(struct_def),
        defs::TypeDef::Union(union_def) => resolve_union_alignment(union_def),
        defs::TypeDef::EventStruct(_) => {
            // TODO: check the required start align fields of allowed events
            Ok(())
        }
        defs::TypeDef::Xid(_) => Ok(()),
        defs::TypeDef::XidUnion(_) => Ok(()),
        defs::TypeDef::Enum(_) => Ok(()),
        defs::TypeDef::Alias(_) => Ok(()),
    }
}

fn resolve_struct_alignment(struct_def: &defs::StructDef) -> Result<(), ResolveError> {
    struct_def.alignment.get_or_try_init(|| {
        let fields = struct_def.fields.borrow();
        let mut field_aligns = Vec::with_capacity(fields.len());
        for field in fields.iter() {
            field_aligns.push(resolve_field_alignment(field)?);
        }

        let struct_align = find_struct_fields_alignment(&field_aligns)
            .ok_or_else(|| ResolveError::UnaligneableStruct(struct_def.name.clone()))?;

        Ok(struct_align)
    })?;
    Ok(())
}

fn resolve_union_alignment(union_def: &defs::UnionDef) -> Result<(), ResolveError> {
    union_def.alignment.get_or_try_init(|| {
        let fields = &union_def.fields;
        let mut field_aligns = Vec::with_capacity(fields.len());
        for field in fields.iter() {
            field_aligns.push(resolve_field_alignment(field)?);
        }

        let mut union_align = find_union_fields_alignment(&field_aligns)
            .ok_or_else(|| ResolveError::UnaligneableUnion(union_def.name.clone()))?;

        // Unlike case switches, unions always have the size of its largest field.
        union_align.body = defs::AlignBody::Size(defs::VariableSize {
            base: union_def.size(),
            incr: 0,
        });
        assert_eq!(union_align.internal_align, 1);

        Ok(union_align)
    })?;
    Ok(())
}

fn resolve_switch_alignment(switch: &defs::SwitchField) -> Result<(), ResolveError> {
    let mut case_aligns = Vec::with_capacity(switch.cases.len());
    for case in switch.cases.iter() {
        let fields = case.fields.borrow();
        let mut field_aligns = Vec::with_capacity(fields.len());
        for field in fields.iter() {
            field_aligns.push(resolve_field_alignment(field)?);
        }
        let case_align = if let Some(req_start_align) = case.required_start_align {
            check_required_start_align(req_start_align)?;
            check_struct_fields_alignment(req_start_align, &field_aligns).ok_or_else(|| {
                ResolveError::UnaligneableSwitchCase(switch.name.clone(), case.name.clone())
            })?
        } else {
            find_struct_fields_alignment(&field_aligns).ok_or_else(|| {
                ResolveError::UnaligneableSwitchCase(switch.name.clone(), case.name.clone())
            })?
        };
        case_aligns.push(case_align);
        case.alignment.set(case_align).unwrap();
    }

    if switch.kind == defs::SwitchKind::Case {
        let switch_align = if let Some(req_start_align) = switch.required_start_align {
            check_required_start_align(req_start_align)?;
            check_union_fields_alignment(req_start_align, &case_aligns)
                .ok_or_else(|| ResolveError::UnaligneableSwitch(switch.name.clone()))?
        } else {
            find_union_fields_alignment(&case_aligns)
                .ok_or_else(|| ResolveError::UnaligneableSwitch(switch.name.clone()))?
        };

        switch.alignment.set(switch_align).unwrap();
    } else {
        let switch_align = if let Some(req_start_align) = switch.required_start_align {
            check_required_start_align(req_start_align)?;
            check_bitcases_alignment(req_start_align, &case_aligns)
                .ok_or_else(|| ResolveError::UnaligneableSwitch(switch.name.clone()))?
        } else {
            find_bitcases_alignment(&case_aligns)
                .ok_or_else(|| ResolveError::UnaligneableSwitch(switch.name.clone()))?
        };

        switch.alignment.set(switch_align).unwrap();
    }

    Ok(())
}

fn find_struct_fields_alignment(
    field_aligns: &[defs::ComplexAlignment],
) -> Option<defs::ComplexAlignment> {
    for &align in [1, 2, 4, 8].iter() {
        for offset in 0..align {
            let start_align = defs::Alignment { align, offset };
            if let Some(alignment) = check_struct_fields_alignment(start_align, field_aligns) {
                return Some(alignment);
            }
        }
    }
    None
}

fn check_struct_fields_alignment(
    start_align: defs::Alignment,
    field_aligns: &[defs::ComplexAlignment],
) -> Option<defs::ComplexAlignment> {
    let mut global_alignment = defs::ComplexAlignment {
        begin: start_align,
        body: defs::AlignBody::Size(defs::VariableSize::zero()),
        internal_align: 1,
    };

    for &field_align in field_aligns.iter() {
        global_alignment = global_alignment.append(field_align)?;
    }

    if start_align.align < global_alignment.internal_align {
        None
    } else {
        Some(global_alignment)
    }
}

fn check_union_fields_alignment(
    start_align: defs::Alignment,
    field_aligns: &[defs::ComplexAlignment],
) -> Option<defs::ComplexAlignment> {
    let mut global_alignment = defs::ComplexAlignment {
        begin: start_align,
        body: defs::AlignBody::Size(defs::VariableSize::zero()),
        internal_align: 1,
    };
    global_alignment = global_alignment.append(field_aligns[0])?;

    for &field_align in field_aligns[1..].iter() {
        global_alignment = global_alignment.union_append(field_align)?;
    }

    if start_align.align < global_alignment.internal_align {
        None
    } else {
        Some(global_alignment)
    }
}

fn find_union_fields_alignment(
    field_aligns: &[defs::ComplexAlignment],
) -> Option<defs::ComplexAlignment> {
    for &align in [1, 2, 4, 8].iter() {
        for offset in 0..align {
            let start_align = defs::Alignment { align, offset };
            if let Some(alignment) = check_union_fields_alignment(start_align, field_aligns) {
                return Some(alignment);
            }
        }
    }
    None
}

fn check_bitcases_alignment(
    start_align: defs::Alignment,
    case_aligns: &[defs::ComplexAlignment],
) -> Option<defs::ComplexAlignment> {
    let mut global_alignment = defs::ComplexAlignment {
        begin: start_align,
        body: defs::AlignBody::Size(defs::VariableSize::zero()),
        internal_align: 1,
    };

    for &field_align in case_aligns.iter() {
        global_alignment = global_alignment.bitcase_append(field_align)?;
    }

    if start_align.align < global_alignment.internal_align {
        None
    } else {
        Some(global_alignment)
    }
}

fn find_bitcases_alignment(
    case_aligns: &[defs::ComplexAlignment],
) -> Option<defs::ComplexAlignment> {
    for &align in [1, 2, 4, 8].iter() {
        for offset in 0..align {
            let start_align = defs::Alignment { align, offset };
            if let Some(alignment) = check_bitcases_alignment(start_align, case_aligns) {
                return Some(alignment);
            }
        }
    }
    None
}

fn resolve_field_alignment(field: &defs::FieldDef) -> Result<defs::ComplexAlignment, ResolveError> {
    match field {
        defs::FieldDef::Pad(pad_field) => match pad_field.kind {
            defs::PadKind::Bytes(bytes) => {
                Ok(defs::ComplexAlignment::fixed_size(u32::from(bytes), 1))
            }
            defs::PadKind::Align(align) => Ok(defs::ComplexAlignment {
                begin: defs::Alignment {
                    align: 1,
                    offset: 0,
                },
                body: defs::AlignBody::EndAlign(defs::Alignment {
                    align: u32::from(align),
                    offset: 0,
                }),
                internal_align: u32::from(align),
            }),
        },
        defs::FieldDef::Normal(normal_field) => {
            get_type_alignment(normal_field.type_.type_.def.get().unwrap())
        }
        defs::FieldDef::List(list_field) => {
            let element_alignment =
                get_type_alignment(list_field.element_type.type_.def.get().unwrap())?;
            if let Some(list_length) = list_field.length() {
                Ok(element_alignment
                    .repeat_n(list_length)
                    .ok_or_else(|| ResolveError::UnaligneableList(list_field.name.clone()))?)
            } else {
                Ok(element_alignment
                    .zero_one_or_many()
                    .ok_or_else(|| ResolveError::UnaligneableList(list_field.name.clone()))?)
            }
        }
        defs::FieldDef::Switch(switch_field) => {
            resolve_switch_alignment(switch_field)?;
            Ok(*switch_field.alignment.get().unwrap())
        }
        // FDs are not sent serialized in the structure,
        // so they do not impose any alignment.
        defs::FieldDef::Fd(_) => Ok(defs::ComplexAlignment::zero_sized()),
        defs::FieldDef::FdList(_) => Ok(defs::ComplexAlignment::zero_sized()),
        defs::FieldDef::Expr(expr_field) => {
            get_type_alignment(expr_field.type_.type_.def.get().unwrap())
        }
        // This is a virtual field (not serialized), so it
        // does not impose any alignment
        defs::FieldDef::VirtualLen(_) => Ok(defs::ComplexAlignment::zero_sized()),
    }
}

fn get_type_alignment(type_: &defs::TypeRef) -> Result<defs::ComplexAlignment, ResolveError> {
    match type_.get_original_type() {
        defs::TypeRef::BuiltIn(builtin_type) => match builtin_type {
            defs::BuiltInType::Card8 => Ok(defs::ComplexAlignment::fixed_size(1, 1)),
            defs::BuiltInType::Card16 => Ok(defs::ComplexAlignment::fixed_size(2, 2)),
            defs::BuiltInType::Card32 => Ok(defs::ComplexAlignment::fixed_size(4, 4)),
            defs::BuiltInType::Card64 => Ok(defs::ComplexAlignment::fixed_size(8, 8)),
            defs::BuiltInType::Int8 => Ok(defs::ComplexAlignment::fixed_size(1, 1)),
            defs::BuiltInType::Int16 => Ok(defs::ComplexAlignment::fixed_size(2, 2)),
            defs::BuiltInType::Int32 => Ok(defs::ComplexAlignment::fixed_size(4, 4)),
            defs::BuiltInType::Int64 => Ok(defs::ComplexAlignment::fixed_size(8, 8)),
            defs::BuiltInType::Byte => Ok(defs::ComplexAlignment::fixed_size(1, 1)),
            defs::BuiltInType::Bool => Ok(defs::ComplexAlignment::fixed_size(1, 1)),
            defs::BuiltInType::Char => Ok(defs::ComplexAlignment::fixed_size(1, 1)),
            defs::BuiltInType::Float => Ok(defs::ComplexAlignment::fixed_size(4, 4)),
            defs::BuiltInType::Double => Ok(defs::ComplexAlignment::fixed_size(8, 8)),
            defs::BuiltInType::Void => Ok(defs::ComplexAlignment::fixed_size(1, 1)),
        },
        defs::TypeRef::Struct(struct_def) => {
            let struct_def = struct_def.upgrade().unwrap();
            resolve_struct_alignment(&struct_def)?;
            let struct_alignment = *struct_def.alignment.get().unwrap();
            Ok(struct_alignment)
        }
        defs::TypeRef::Union(union_def) => {
            let union_def = union_def.upgrade().unwrap();
            resolve_union_alignment(&union_def)?;
            let union_alignment = *union_def.alignment.get().unwrap();
            Ok(union_alignment)
        }
        defs::TypeRef::EventStruct(_) => {
            // TODO: check for xge events
            Ok(defs::ComplexAlignment::fixed_size(32, 4))
        }
        defs::TypeRef::Xid(_) => Ok(defs::ComplexAlignment::fixed_size(4, 4)),
        defs::TypeRef::XidUnion(_) => Ok(defs::ComplexAlignment::fixed_size(4, 4)),
        // `type_resolver` should have rejected this
        defs::TypeRef::Enum(_) => unreachable!(),
        // should not be returned by `TypeRef::get_original_type`
        defs::TypeRef::Alias(_) => unreachable!(),
    }
}

fn check_required_start_align(required_start_align: defs::Alignment) -> Result<(), ResolveError> {
    if !required_start_align.align.is_power_of_two()
        || required_start_align.offset >= required_start_align.align
    {
        Err(ResolveError::InvalidRequiredStartAlign(
            required_start_align.align,
            required_start_align.offset,
        ))
    } else {
        Ok(())
    }
}
