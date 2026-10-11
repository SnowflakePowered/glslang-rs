#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;

pub use bindings::*;

impl crate::bindings::glslang_limits_s {
    pub const fn default() -> Self {
        crate::bindings::glslang_limits_t {
            non_inductive_for_loops: true,
            while_loops: true,
            do_while_loops: true,
            general_uniform_indexing: true,
            general_attribute_matrix_vector_indexing: true,
            general_varying_indexing: true,
            general_sampler_indexing: true,
            general_variable_indexing: true,
            general_constant_matrix_vector_indexing: true,
        }
    }
}

impl Default for crate::bindings::glslang_limits_s {
    fn default() -> Self {
        crate::bindings::glslang_limits_s::default()
    }
}
