use std::rc::Rc;

use crate::defs;

mod align_checker;
mod extra_fields_inserter;
mod field_ref_resolver;
mod nesting_checker;
mod param_ref_gatherer;
mod type_resolver;

#[derive(Debug)]
pub enum ResolveError {
    UnknownImport(String),
    RepeatedFieldName(String),
    RepeatedEnumValueName(String),
    InvalidTypeName(String),
    UnknownRequestName(String),
    UnknownEventName(String),
    AmbiguousEventName(String),
    UnknownErrorName(String),
    AmbiguousErrorName(String),
    UnknownTypeName(String),
    AmbiguousTypeName(String),
    InvalidUseOfEnum(String),
    UnknownExtNameInEventStruct(String),
    UnknownEventNumberEventStruct(String, u16, bool),
    InfiniteStructNesting,
    InvalidRequiredStartAlign(u32, u32),
    UnaligneableRequest(String),
    UnaligneableReply(String),
    UnaligneableEvent(String),
    UnaligneableError(String),
    UnaligneableStruct(String),
    UnaligneableUnion(String),
    UnaligneableList(String),
    UnaligneableSwitch(String),
    UnaligneableSwitchCase(String, Option<String>),
    UnalignedField(Option<String>),
    UnalignedSwitchCase(String, Option<String>),
    UnalignedSwitchCaseEnd(String, Option<String>),
    DiscrepantParamRefTypes(String),
    InvalidFieldRef(String),
    UnknownNameInFieldRef(String),
}

pub fn resolve(module: &defs::Module) -> Result<(), ResolveError> {
    // Resolve imports
    resolve_imports(module)?;

    // Resolve types
    type_resolver::resolve(module)?;

    // Check nesting
    nesting_checker::check(module)?;

    // Insert extra fields
    extra_fields_inserter::run(module);

    // TODO: check that complex types have at least one field
    // TODO: check that unions fields have fixed size
    // TODO: check that types are not zero sized
    // TODO: check that switches have at least one case
    // align_checker might panic if those invariants are not met

    // Check alignment
    align_checker::check(module)?;

    // Gather `paramref` expressions in structs
    param_ref_gatherer::gather(module)?;

    // Resolve `fieldref` expressions in structs
    field_ref_resolver::resolve(module)?;

    Ok(())
}

fn resolve_imports(module: &defs::Module) -> Result<(), ResolveError> {
    let namespaces = module.namespaces.borrow();
    for ns in namespaces.values() {
        for import in ns.imports.borrow().values() {
            if let Some(imported_ns) = namespaces.get(&import.name) {
                import.ns.set(Rc::downgrade(imported_ns)).unwrap();
            } else {
                return Err(ResolveError::UnknownImport(import.name.clone()));
            }
        }
    }
    Ok(())
}
