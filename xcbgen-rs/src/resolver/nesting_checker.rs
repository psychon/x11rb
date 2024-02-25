use std::collections::HashSet;
use std::rc::Rc;

use crate::{defs, ResolveError};

/// Check that there is no infinite struct nesting, i.e. no structs containing themselves (even
/// indirectly).
#[inline]
pub(super) fn check(module: &defs::Module) -> Result<(), ResolveError> {
    NestingChecker::new().check_module(module)
}

struct NestingChecker {
    stack: Vec<NestingStackItem>,
    /// Pointers converted to `usize`.
    checked: HashSet<usize>,
}

impl NestingChecker {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            checked: HashSet::default(),
        }
    }

    fn push(&mut self, item: NestingStackItem) -> Result<(), ResolveError> {
        if self
            .stack
            .iter()
            .any(|stack_item| stack_item.same_as(&item))
        {
            return Err(ResolveError::InfiniteStructNesting);
        }
        self.stack.push(item);
        Ok(())
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn check_module(&mut self, module: &defs::Module) -> Result<(), ResolveError> {
        for ns in module.namespaces.borrow().values() {
            for type_def in ns.type_defs.borrow().values() {
                self.check_type_def(type_def)?;
            }
        }
        Ok(())
    }

    fn check_type_def(&mut self, type_def: &defs::TypeDef) -> Result<(), ResolveError> {
        match type_def {
            defs::TypeDef::Struct(struct_def) => self.check_struct(struct_def),
            defs::TypeDef::Union(union_def) => self.check_union(union_def),
            _ => Ok(()),
        }
    }

    fn check_struct(&mut self, struct_def: &Rc<defs::StructDef>) -> Result<(), ResolveError> {
        self.push(NestingStackItem::Struct(struct_def.clone()))?;
        let struct_def_ptr: *const defs::StructDef = &**struct_def;
        if self.checked.insert(struct_def_ptr as usize) {
            // Not checked yet
            for field in struct_def.fields.borrow().iter() {
                self.check_field(field)?;
            }
        }
        self.pop();
        Ok(())
    }

    fn check_union(&mut self, union_def: &Rc<defs::UnionDef>) -> Result<(), ResolveError> {
        self.push(NestingStackItem::Union(union_def.clone()))?;
        let union_def_ptr: *const defs::UnionDef = &**union_def;
        if self.checked.insert(union_def_ptr as usize) {
            // Not checked yet
            for field in union_def.fields.iter() {
                self.check_field(field)?;
            }
        }
        self.pop();
        Ok(())
    }

    fn check_field(&mut self, field: &defs::FieldDef) -> Result<(), ResolveError> {
        match field {
            defs::FieldDef::Pad(_) => Ok(()),
            defs::FieldDef::Normal(normal_field) => {
                self.check_field_value_type(&normal_field.type_)?;
                Ok(())
            }
            defs::FieldDef::List(list_field) => {
                self.check_field_value_type(&list_field.element_type)?;
                Ok(())
            }
            defs::FieldDef::Switch(switch_field) => {
                for case in switch_field.cases.iter() {
                    for case_field in case.fields.borrow().iter() {
                        self.check_field(case_field)?;
                    }
                }
                Ok(())
            }
            defs::FieldDef::Fd(_) => Ok(()),
            defs::FieldDef::FdList(_) => Ok(()),
            defs::FieldDef::Expr(_) => Ok(()),
            defs::FieldDef::VirtualLen(_) => Ok(()),
        }
    }

    fn check_field_value_type(&mut self, type_: &defs::FieldValueType) -> Result<(), ResolveError> {
        self.check_type_ref(type_.type_.get_resolved())
    }

    fn check_type_ref(&mut self, type_: &defs::TypeRef) -> Result<(), ResolveError> {
        match type_ {
            defs::TypeRef::Struct(struct_def) => self.check_struct(&struct_def.upgrade().unwrap()),
            defs::TypeRef::Union(union_def) => self.check_union(&union_def.upgrade().unwrap()),
            defs::TypeRef::Alias(type_alias_def) => {
                let type_alias_def = type_alias_def.upgrade().unwrap();
                self.check_type_ref(type_alias_def.old_name.get_resolved())
            }
            _ => Ok(()),
        }
    }
}

enum NestingStackItem {
    Struct(Rc<defs::StructDef>),
    Union(Rc<defs::UnionDef>),
}

impl NestingStackItem {
    fn same_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Struct(this), Self::Struct(other)) => Rc::ptr_eq(this, other),
            (Self::Union(this), Self::Union(other)) => Rc::ptr_eq(this, other),
            _ => false,
        }
    }
}
