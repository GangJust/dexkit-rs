#[allow(warnings)]
mod gen_flatbuffers {
    include!(concat!(env!("OUT_DIR"), "/flatbuffers/mod.rs"));
}

#[allow(unused)]
pub mod errors {
    mod errors;
    pub use errors::*;
}

#[allow(unused)]
pub mod query {
    pub mod base {
        mod base_query;
        pub use base_query::*;
        mod i_annotation_encode_value;
        pub use i_annotation_encode_value::*;
        mod i_number_encode_value;
        pub use i_number_encode_value::*;
        mod i_query;
        pub use i_query::*;
    }

    pub mod enums {
        mod annotation_encode_value_type;
        pub use annotation_encode_value_type::*;
        mod annotation_visibility_type;
        pub use annotation_visibility_type::*;
        mod match_type;
        pub use match_type::*;
        mod number_encode_value_type;
        pub use number_encode_value_type::*;
        mod op_code_match_type;
        pub use op_code_match_type::*;
        mod retention_policy_type;
        pub use retention_policy_type::*;
        mod string_match_type;
        pub use string_match_type::*;
        mod target_element_type;
        pub use target_element_type::*;
        mod using_type;

        pub use using_type::*;
    }

    pub mod matchers {
        pub mod base {
            mod access_flags_matcher;
            pub use access_flags_matcher::*;
            mod annotation_encode_value_matcher;
            pub use annotation_encode_value_matcher::*;
            mod int_range;
            pub use int_range::*;
            mod op_codes_matcher;
            pub use op_codes_matcher::*;
            mod string_matcher;
            pub use string_matcher::*;
            mod target_element_types_matcher;
            pub use target_element_types_matcher::*;
            mod number_encode_value_matcher;
            pub use number_encode_value_matcher::*;
        }

        mod annotation_element_matcher;
        pub use annotation_element_matcher::*;
        mod annotation_elements_matcher;
        pub use annotation_elements_matcher::*;
        mod annotation_encode_array_matcher;
        pub use annotation_encode_array_matcher::*;
        mod annotation_matcher;
        pub use annotation_matcher::*;
        mod annotations_matcher;
        pub use annotations_matcher::*;
        mod class_matcher;
        pub use class_matcher::*;
        mod encode_value;
        pub use encode_value::*;
        mod field_matcher;
        pub use field_matcher::*;
        mod fields_matcher;
        pub use fields_matcher::*;
        mod interfaces_matcher;
        pub use interfaces_matcher::*;
        mod method_matcher;
        pub use method_matcher::*;
        mod methods_matcher;
        pub use methods_matcher::*;
        mod parameters_matcher;
        pub use parameters_matcher::*;
        mod parameter_matcher;
        pub use parameter_matcher::*;
        mod using_field_matcher;
        pub use using_field_matcher::*;
        mod string_matchers_group;
        pub use string_matchers_group::*;
    }

    mod batch_find_class_using_strings;
    pub use batch_find_class_using_strings::*;
    mod batch_find_method_using_strings;
    pub use batch_find_method_using_strings::*;
    mod find_class;
    pub use find_class::*;
    mod find_field;
    pub use find_field::*;
    mod find_method;
    pub use find_method::*;
}

#[allow(unused)]
pub mod result {
    pub mod base {
        mod base_data;
        pub use base_data::*;
    }

    mod data_collections;
    pub use data_collections::*;

    mod class_data;
    pub use class_data::*;
    mod field_data;
    pub use field_data::*;
    mod method_data;
    pub use method_data::*;
    mod annotation_data;
    pub use annotation_data::*;
    mod annotation_element_data;
    pub use annotation_element_data::*;
    mod annotation_encode_array_data;
    pub use annotation_encode_array_data::*;
    mod annotation_encode_value;
    pub use annotation_encode_value::*;
    mod using_field_data;
    pub use using_field_data::*;
    mod field_using_type;
    pub use field_using_type::*;
}

pub mod wrap {
    mod dex_class;
    pub use dex_class::*;
    mod dex_field;
    pub use dex_field::*;
    mod dex_method;
    pub use dex_method::*;
}

#[allow(unused)]
pub mod uitls {
    mod modifier;
    pub use modifier::*;
    mod dex_signature;
    pub use dex_signature::*;
    mod mutf8;
    pub use mutf8::*;
    mod string_unicode_encode_decode;
    pub use string_unicode_encode_decode::*;
    mod op_codes;
    pub use op_codes::*;
}

mod dexkit_bridge;
pub use dexkit_bridge::*;
