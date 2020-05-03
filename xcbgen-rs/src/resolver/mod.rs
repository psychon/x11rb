use std::rc::Rc;

use crate::defs;

mod align_checker;
mod extra_fields_inserter;
mod field_ref_resolver;
mod nesting_checker;
mod param_ref_gatherer;
mod type_resolver;

/// An error that occurred while resolving a module
#[derive(Debug)]
pub enum ResolveError {
    /// A namespace was imported that was not loaded into the module.
    ///
    /// The argument is the name of the missing namespace.
    UnknownImport(String),

    /// Some complex type contains two fields with the same name.
    ///
    /// The argument is the name of the duplicate field.
    RepeatedFieldName(String),

    /// Some enum contains two values with the same name.
    ///
    /// The argument is the name of the duplicate value.
    RepeatedEnumValueName(String),

    /// The given type name is invalid.
    InvalidTypeName(String),

    /// An event was copied, but the source of the copy was not found.
    UnknownEventName(String),

    /// An event was copied, but the source of the copy is ambiguous.
    ///
    /// The source for the copy is a plain name (without specifying the namespace that the event
    /// can be found in). There is no event with that name in the current namespace and there are
    /// multiple possibilities in other namespaces, making the reference ambiguous.
    AmbiguousEventName(String),

    /// An error was copied, but the source of the copy was not found.
    UnknownErrorName(String),

    /// An error was copied, but the source of the copy is ambiguous.
    ///
    /// The source for the copy is a plain name (without specifying the namespace that the event
    /// can be found in). There is no event with that name in the current namespace and there are
    /// multiple possibilities in other namespaces, making the reference ambiguous.
    AmbiguousErrorName(String),

    /// The type of some field was not found.
    ///
    /// The argument is the type that was not found.
    UnknownTypeName(String),

    /// The type of some field is ambiguous.
    ///
    /// The type is specified without specifying the namespace that it can be found in. In the
    /// imported namespaces, there are multiple types with this name.
    AmbiguousTypeName(String),

    /// An enum type was used in a place where no enum is allowed.
    ///
    /// As an example for this error, an enum cannot be used in a type alias (`<typedef>`).
    InvalidUseOfEnum(String),

    /// An unknown extension name was used in an `<eventstruct>`.
    UnknownExtNameInEventStruct(String),

    /// An `<eventstruct>` refers to an unknown event.
    ///
    /// The tuple contains the name of the extension, the event opcode that was not found, and
    /// whether XGE was allowed.
    UnknownEventNumberEventStruct(String, u16, bool),

    /// Some type (indirectly) includes itself.
    InfiniteStructNesting,

    /// Some required align is invalid.
    ///
    /// The tuple contains the alignment and offset that are incorrect.
    InvalidRequiredStartAlign(u32, u32),

    /// The given request contains an unaligned field.
    UnaligneableRequest(String),

    /// The given reply contains an unaligned field.
    UnaligneableReply(String),

    /// The given event contains an unaligned field.
    UnaligneableEvent(String),

    /// The given error contains an unaligned field.
    UnaligneableError(String),

    /// The given struct contains an unaligned field.
    UnaligneableStruct(String),

    /// The given union contains an unaligned field.
    UnaligneableUnion(String),

    /// The given list contains an unaligned field.
    UnaligneableList(String),

    /// The given switch contains an unaligned field.
    UnaligneableSwitch(String),

    /// The given switch-case contains an unaligned field.
    ///
    /// The tuple contains the name of the switch and of the case.
    UnaligneableSwitchCase(String, Option<String>),

    /// A parameter was referenced as different types.
    DiscrepantParamRefTypes(String),

    /// A field was referenced that is not allowed to be referenced.
    InvalidFieldRef(String),

    /// A field reference was not found / does not exist.
    UnknownNameInFieldRef(String),
}

/// Resolve a module
///
/// This function verifies that the module's XML is well-formed. Then it synthesises some implicit
/// fields, e.g. `response_type` in events and also resolves references to other fields.
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
