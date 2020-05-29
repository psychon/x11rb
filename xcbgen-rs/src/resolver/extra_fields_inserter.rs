use crate::defs;

/// Create synthesised fields in the module.
///
/// This function creates virtual fields in structures that are not part of the XML. This includes
/// generic fields in errors, events, requests, and replies that are present in all such types.
/// This function also creates virtual length fields for lists without a length.  Because these
/// added fields are so generic, they are not explicitly present in the XML description.
pub(super) fn run(module: &defs::Module) {
    for ns in module.namespaces.borrow().values() {
        for request_def in ns.request_defs.borrow().values() {
            run_in_request(request_def, module);
        }
        for event_def in ns.event_defs.borrow().values() {
            if let defs::EventDef::Full(event_full_def) = event_def {
                run_in_event(event_full_def, module);
            }
        }
        for error_def in ns.error_defs.borrow().values() {
            if let defs::ErrorDef::Full(error_full_def) = error_def {
                run_in_error(error_full_def, module);
            }
        }
        for type_def in ns.type_defs.borrow().values() {
            if let defs::TypeDef::Struct(struct_def) = type_def {
                run_in_struct(struct_def, module);
            }
        }
    }
}

fn run_in_request(request_def: &defs::RequestDef, module: &defs::Module) {
    let mut fields = request_def.fields.borrow_mut();

    let major_opcode_field = defs::FieldDef::Normal(defs::NormalField {
        name: "major_opcode".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card8),
    });
    let length_field = defs::FieldDef::Normal(defs::NormalField {
        name: "length".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card16),
    });

    if request_def.namespace.upgrade().unwrap().ext_info.is_some() {
        let minor_opcode_field = defs::FieldDef::Normal(defs::NormalField {
            name: "minor_opcode".into(),
            type_: make_builtin_field_value_type(defs::BuiltInType::Card8),
        });
        fields.insert(0, major_opcode_field);
        fields.insert(1, minor_opcode_field);
        fields.insert(2, length_field);
    } else {
        fields.insert(0, major_opcode_field);
        if fields.get(1).and_then(|field| field.size()) != Some(1) {
            fields.insert(
                1,
                defs::FieldDef::Pad(defs::PadField {
                    kind: defs::PadKind::Bytes(1),
                    serialize: false,
                }),
            );
        }
        fields.insert(2, length_field);
    }
    run_in_field_list(&mut fields, module);
    if let Some(ref reply_def) = request_def.reply {
        run_in_reply(reply_def, module);
    }
}

fn run_in_reply(reply_def: &defs::ReplyDef, module: &defs::Module) {
    let mut fields = reply_def.fields.borrow_mut();

    let response_type_field = defs::FieldDef::Normal(defs::NormalField {
        name: "response_type".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card8),
    });
    let sequence_field = defs::FieldDef::Normal(defs::NormalField {
        name: "sequence".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card16),
    });
    let length_field = defs::FieldDef::Normal(defs::NormalField {
        name: "length".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card32),
    });

    fields.insert(0, response_type_field);
    if fields.get(1).and_then(|field| field.size()) != Some(1) {
        fields.insert(
            1,
            defs::FieldDef::Pad(defs::PadField {
                kind: defs::PadKind::Bytes(1),
                serialize: false,
            }),
        );
    }
    fields.insert(2, sequence_field);
    fields.insert(3, length_field);

    run_in_field_list(&mut fields, module);
}

fn run_in_event(event_def: &defs::EventFullDef, module: &defs::Module) {
    let mut fields = event_def.fields.borrow_mut();

    let response_type_field = defs::FieldDef::Normal(defs::NormalField {
        name: "response_type".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card8),
    });
    let sequence_field = defs::FieldDef::Normal(defs::NormalField {
        name: "sequence".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card16),
    });

    if event_def.xge {
        let extension_field = defs::FieldDef::Normal(defs::NormalField {
            name: "extension".into(),
            type_: make_builtin_field_value_type(defs::BuiltInType::Card8),
        });
        let length_field = defs::FieldDef::Normal(defs::NormalField {
            name: "length".into(),
            type_: make_builtin_field_value_type(defs::BuiltInType::Card32),
        });
        let event_type_field = defs::FieldDef::Normal(defs::NormalField {
            name: "event_type".into(),
            type_: make_builtin_field_value_type(defs::BuiltInType::Card16),
        });
        fields.insert(0, response_type_field);
        fields.insert(1, extension_field);
        fields.insert(2, sequence_field);
        fields.insert(3, length_field);
        fields.insert(4, event_type_field);
    } else {
        fields.insert(0, response_type_field);
        if !event_def.no_sequence_number {
            if fields.get(1).and_then(|field| field.size()) != Some(1) {
                fields.insert(
                    1,
                    defs::FieldDef::Pad(defs::PadField {
                        kind: defs::PadKind::Bytes(1),
                        serialize: false,
                    }),
                );
            }
            fields.insert(2, sequence_field);
        }
    }

    run_in_field_list(&mut fields, module);
}

fn run_in_error(error_def: &defs::ErrorFullDef, module: &defs::Module) {
    let mut fields = error_def.fields.borrow_mut();

    let response_type_field = defs::FieldDef::Normal(defs::NormalField {
        name: "response_type".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card8),
    });
    let error_code_field = defs::FieldDef::Normal(defs::NormalField {
        name: "error_code".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card8),
    });
    let sequence_field = defs::FieldDef::Normal(defs::NormalField {
        name: "sequence".into(),
        type_: make_builtin_field_value_type(defs::BuiltInType::Card16),
    });

    fields.insert(0, response_type_field);
    fields.insert(1, error_code_field);
    fields.insert(2, sequence_field);
    run_in_field_list(&mut fields, module);
}

fn run_in_struct(struct_def: &defs::StructDef, module: &defs::Module) {
    run_in_field_list(&mut struct_def.fields.borrow_mut(), module);
}

fn run_in_field_list(fields: &mut Vec<defs::FieldDef>, module: &defs::Module) {
    let mut i: usize = 0;
    while i < fields.len() {
        match fields[i] {
            defs::FieldDef::Pad(_) => {}
            defs::FieldDef::Normal(_) => {}
            defs::FieldDef::List(ref list_field) => {
                if list_field.length_expr.is_none() {
                    i += 1;
                    let list_name = list_field.name.clone();
                    let len_field_name = make_len_field_name(&list_name);
                    fields.insert(
                        i,
                        defs::FieldDef::VirtualLen(defs::VirtualLenField {
                            name: len_field_name,
                            type_: make_builtin_field_value_type(defs::BuiltInType::Card32),
                            list_name,
                        }),
                    );
                }
            }
            defs::FieldDef::Switch(ref switch_field) => {
                for switch_case in switch_field.cases.iter() {
                    run_in_field_list(&mut switch_case.fields.borrow_mut(), module);
                }
            }
            defs::FieldDef::Fd(_) => {}
            defs::FieldDef::FdList(_) => {}
            defs::FieldDef::Expr(_) => {}
            defs::FieldDef::VirtualLen(_) => {}
        }
        i += 1;
    }
}

fn make_builtin_field_value_type(builtin_type: defs::BuiltInType) -> defs::FieldValueType {
    let type_name = match builtin_type {
        defs::BuiltInType::Card8 => "CARD8",
        defs::BuiltInType::Card16 => "CARD16",
        defs::BuiltInType::Card32 => "CARD32",
        defs::BuiltInType::Card64 => "CARD64",
        defs::BuiltInType::Int8 => "INT8",
        defs::BuiltInType::Int16 => "INT16",
        defs::BuiltInType::Int32 => "INT32",
        defs::BuiltInType::Int64 => "INT64",
        defs::BuiltInType::Byte => "BYTE",
        defs::BuiltInType::Bool => "BOOL",
        defs::BuiltInType::Char => "char",
        defs::BuiltInType::Float => "float",
        defs::BuiltInType::Double => "double",
        defs::BuiltInType::Void => "void",
    };
    defs::FieldValueType {
        type_: defs::NamedTypeRef::resolved(type_name.into(), defs::TypeRef::BuiltIn(builtin_type)),
        value_set: defs::FieldValueSet::None,
    }
}

fn make_len_field_name(list_name: &str) -> String {
    let mut len_field_name = String::with_capacity(list_name.len() + 4);
    len_field_name.push_str(list_name);
    len_field_name.push_str("_len");
    len_field_name
}
