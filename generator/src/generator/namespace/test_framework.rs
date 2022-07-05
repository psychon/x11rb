//! The test framework for the automatically generated code.

use std::{collections::HashMap, iter};

use super::{
    helpers::{CaseInfo, DeducibleField},
    NamespaceGenerator,
};
use crate::generator::{
    camel_case_to_lower_snake,
    namespace::helpers::{to_rust_type_name, to_rust_variable_name},
    output::Output,
};

use xcbgen::defs as xcbdefs;

pub(super) struct TestFramework<'a, 'b, 'r> {
    pub(super) generator: &'r NamespaceGenerator<'a, 'b>,
    pub(super) name: &'r str,
    pub(super) variant: TestFrameworkType<'r>,
    pub(super) externals: &'r [xcbdefs::ExternalParam],
}

pub(super) enum TestFrameworkType<'r> {
    Struct {
        fields: &'r [xcbdefs::FieldDef],
        deducibles: &'r HashMap<String, DeducibleField>,
    },
    Switch {
        switch_case: &'r xcbdefs::SwitchField,
        case_info: &'r [CaseInfo],
    },
}

impl<'a, 'b, 'r> TestFramework<'a, 'b, 'r> {
    /// Emit the opening block for the test framework.
    pub(crate) fn open(&self, out: &mut Output) {
        // the module is stored under the snake case name
        let mut module_name = camel_case_to_lower_snake(self.name);
        if module_name == "type" {
            module_name = "type_".to_string();
        }

        outln!(out, "#[cfg(test)]");
        outln!(out, "mod {} {{", module_name);

        // bump the indent
        out.incr_indent();

        // continue with imports
        outln!(out, "use super::{};", to_rust_type_name(self.name));
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::x11_utils::{{GenRandom, Serialize}};");
        outln!(out, "use fastrand::Rng;");
    }

    /// Emit the closing block for the test framework.
    pub(crate) fn close(&self, out: &mut Output) {
        // unbump the indent
        out.decr_indent();
        outln!(out, "}}");
    }

    /// Emit the implementation of `GenRandom`.
    pub(crate) fn gen_random(&self, out: &mut Output) {
        outln!(
            out,
            "impl GenRandom for {} {{",
            to_rust_type_name(self.name)
        );
        out.indented(|out| {
            outln!(out, "fn generate(rng: &Rng) -> Self {{");
            out.indented(|out| match self.variant {
                TestFrameworkType::Struct {
                    fields, deducibles, ..
                } => {
                    self.gen_random_struct(out, fields, deducibles);
                }
                TestFrameworkType::Switch {
                    switch_case,
                    case_info,
                } => self.gen_random_switch(out, switch_case, case_info),
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
        self.emit_generate_values_fn(out);
    }

    /// Emit a test that run `serialize()` and `serialize_into()` for
    /// a type, then tests to ensure that the result is the same.
    pub(crate) fn gen_check_serialize(&self, out: &mut Output) {
        outln!(out, "#[test]");
        outln!(out, "fn check_serialize() {{");
        out.indented(|out| {
            self.emit_rng_declaration(out, 0);
            self.emit_value_declaration(out);

            // serialize to an array and also serialize to a vec
            out!(out, "let left = value.serialize(");
            self.emit_extern_parameters(out, false);
            outln!(out, "let mut right = alloc::vec![];");
            out!(out, "value.serialize_into(&mut right");
            self.emit_extern_parameters(out, true);

            // left and right should be equal
            outln!(out, "assert_eq!(&left[..], right.as_slice());");

            self.close_value_declaration(out);
        });
        outln!(out, "}}");
    }

    /// Emit the `GenRandom` implementation for a structure.
    fn gen_random_struct(
        &self,
        out: &mut Output,
        fields: &[xcbdefs::FieldDef],
        deducibles: &HashMap<String, DeducibleField>,
    ) {
        outln!(out, "Self {{");
        out.indented(|out| {
            // for each non-deducible field, generate a random value
            for field in fields.iter().filter(|field| match field.name() {
                None => false,
                Some(_) => self.generator.field_is_visible(field, deducibles),
            }) {
                outln!(
                    out,
                    "{}: GenRandom::generate(rng),",
                    to_rust_variable_name(field.name().unwrap())
                );
            }
        });
        outln!(out, "}}");
    }

    /// Emit the `GenRandom` implementation for a switch.
    fn gen_random_switch(
        &self,
        out: &mut Output,
        switch_case: &xcbdefs::SwitchField,
        case_info: &[CaseInfo],
    ) {
        // for each case, generate a random value
        match switch_case.kind {
            xcbdefs::SwitchKind::BitCase => {
                // it's going to be a list of cases, GenRandom is implemented for
                // Option<T> so we should be fine here
                outln!(out, "Self {{");
                out.indented(|out| {
                    for (c_info, case) in case_info.iter().zip(switch_case.cases.iter()) {
                        with_field_name(c_info, case, |field_name| {
                            outln!(
                                out,
                                "{}: GenRandom::generate(rng),",
                                to_rust_variable_name(field_name)
                            );
                        });
                    }
                });
                outln!(out, "}}");
            }
            xcbdefs::SwitchKind::Case => {
                outln!(out, "match rng.usize(..{}) {{", case_info.len());
                out.indented(|out| {
                    for (i, (c_info, case)) in
                        case_info.iter().zip(switch_case.cases.iter()).enumerate()
                    {
                        if i == case_info.len() - 1 {
                            out!(out, "_ => ");
                        } else {
                            out!(out, "{} => ", i);
                        }

                        with_field_name(c_info, case, |field_name| {
                            outln!(
                                out,
                                "Self::{}(GenRandom::generate(rng)),",
                                to_rust_type_name(field_name)
                            );
                        });
                    }
                });
                outln!(out, "}}");
            }
        }
    }

    /// Emit the declaration for a `fastrand::Rng`, in the `rng` slot.
    fn emit_rng_declaration(&self, out: &mut Output, factor: u64) {
        // we need to pick a seed to ensure deterministic tests
        // in order to ensure a deterministic build, use a strategy
        // derived from the name of the test framework
        let mut seed = 1u64;
        for b in self.name.bytes() {
            seed = seed.overflowing_mul(b as u64).0;
            if seed == 0 {
                seed = 1;
            }
        }
        seed = seed.saturating_add(factor);

        // emit the declaration
        outln!(out, "let rng = Rng::with_seed({});", seed);
    }

    /// Emit a function for generating an array for the given switch.
    fn emit_generate_values_fn(&self, out: &mut Output) {
        if let TestFrameworkType::Switch {
            switch_case,
            case_info,
        } = self.variant
        {
            outln!(
                out,
                "fn generate_values(rng: &Rng) -> alloc::vec::Vec<{}> {{",
                to_rust_type_name(self.name)
            );
            out.indented(|out| {
                outln!(out, "alloc::vec![");
                out.indented(|out| {
                    match switch_case.kind {
                        xcbdefs::SwitchKind::BitCase => {
                            // testing every case would be extraordinarily expensive,
                            // so just test:
                            // - all are true
                            // - none are true
                            // - one for each is true
                            self.emit_bitcase_value_permutation(
                                out,
                                case_info,
                                switch_case,
                                iter::repeat(false),
                            );
                            self.emit_bitcase_value_permutation(
                                out,
                                case_info,
                                switch_case,
                                iter::repeat(true),
                            );
                            for i in 0..case_info.len() {
                                self.emit_bitcase_value_permutation(
                                    out,
                                    case_info,
                                    switch_case,
                                    (0..case_info.len()).map(|index| index == i),
                                );
                            }
                        }
                        xcbdefs::SwitchKind::Case => {
                            for (c_info, case) in case_info.iter().zip(switch_case.cases.iter()) {
                                with_field_name(c_info, case, |field_name| {
                                    outln!(
                                        out,
                                        "{}::{}(GenRandom::generate(rng)),",
                                        to_rust_type_name(self.name),
                                        to_rust_type_name(field_name)
                                    );
                                });
                            }
                        }
                    }
                });
                outln!(out, "]");
            });
            outln!(out, "}}");
        }
    }

    /// Emit the declaration for a randomly-generated value of type
    /// `Self`, in the `value` slot.
    fn emit_value_declaration(&self, out: &mut Output) {
        match self.variant {
            TestFrameworkType::Struct { .. } => {
                // emit the declaration for the value
                outln!(
                    out,
                    "let value = {}::generate(&rng);",
                    to_rust_type_name(self.name)
                );
            }
            TestFrameworkType::Switch { .. } => {
                outln!(out, "for value in generate_values(&rng) {{");
                out.incr_indent();
            }
        }
    }

    fn emit_bitcase_value_permutation(
        &self,
        out: &mut Output,
        case_info: &[CaseInfo],
        switch_case: &xcbdefs::SwitchField,
        mask: impl IntoIterator<Item = bool>,
    ) {
        outln!(out, "{} {{", to_rust_type_name(self.name));
        out.indented(|out| {
            for ((c_info, case), unmasked) in
                case_info.iter().zip(switch_case.cases.iter()).zip(mask)
            {
                with_field_name(c_info, case, |field_name| {
                    out!(out, "{}: ", to_rust_variable_name(field_name));
                    if unmasked {
                        outln!(out, "Some(GenRandom::generate(rng)),");
                    } else {
                        outln!(out, "None,");
                    }
                });
            }
        });
        outln!(out, "}},");
    }

    fn close_value_declaration(&self, out: &mut Output) {
        if let TestFrameworkType::Switch { .. } = self.variant {
            out.decr_indent();
            outln!(out, "}}");
        }
    }

    fn emit_extern_parameters(&self, out: &mut Output, begin_with_comma: bool) {
        let params = self.generator.ext_params_to_call_args(
            begin_with_comma,
            |_| "GenRandom::generate(&rng)".to_string(),
            self.externals,
        );
        outln!(out, "{});", params);
    }
}

fn with_field_name<R>(
    c_info: &CaseInfo,
    case: &xcbdefs::SwitchCase,
    f: impl FnOnce(&str) -> R,
) -> R {
    let fields = case.fields.borrow();
    let field_name = match c_info {
        CaseInfo::MultiField(field_name, ..) => field_name,
        CaseInfo::SingleField(index) => fields[*index].name().unwrap(),
    };

    f(field_name)
}
