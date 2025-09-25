use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationEncodeValueMeta as FBAnnotationEncodeValueMeta,
    AnnotationEncodeValueType as FBAnnotationEncodeValueType,
};
use crate::query::enums::AnnotationEncodeValueType;

#[derive(Debug, Clone)]
pub struct AnnotationEncodeValue {
    value_type: AnnotationEncodeValueType,
    value: Vec<u8>,
}

impl AnnotationEncodeValue {
    /// ...
    pub(crate) fn with_meta(meta: &FBAnnotationEncodeValueMeta) -> AnnotationEncodeValue {
        let meta_value_tpye = meta.type_();
        let value = match meta_value_tpye {
            FBAnnotationEncodeValueType::ByteValue => match meta.value_as_encode_value_byte() {
                None => Vec::new(),
                Some(v) => v.value().to_le_bytes().to_vec(),
            },

            FBAnnotationEncodeValueType::ShortValue => match meta.value_as_encode_value_short() {
                None => Vec::new(),
                Some(v) => v.value().to_le_bytes().to_vec(),
            },

            FBAnnotationEncodeValueType::CharValue => match meta.value_as_encode_value_char() {
                None => Vec::new(),
                Some(v) => v.value().to_le_bytes().to_vec(),
            },

            FBAnnotationEncodeValueType::IntValue => match meta.value_as_encode_value_int() {
                None => Vec::new(),
                Some(v) => v.value().to_le_bytes().to_vec(),
            },

            FBAnnotationEncodeValueType::LongValue => match meta.value_as_encode_value_long() {
                None => Vec::new(),
                Some(v) => v.value().to_le_bytes().to_vec(),
            },

            FBAnnotationEncodeValueType::FloatValue => match meta.value_as_encode_value_float() {
                None => Vec::new(),
                Some(v) => v.value().to_le_bytes().to_vec(),
            },

            FBAnnotationEncodeValueType::DoubleValue => match meta.value_as_encode_value_double() {
                None => Vec::new(),
                Some(v) => v.value().to_le_bytes().to_vec(),
            },

            FBAnnotationEncodeValueType::StringValue => match meta.value_as_encode_value_string() {
                None => Vec::new(),
                Some(v) => match v.value() {
                    None => Vec::new(),
                    Some(s) => s.as_bytes().to_vec(),
                },
            },

            FBAnnotationEncodeValueType::TypeValue => {
                match meta.value_as_class_meta() {
                    None => Vec::new(),
                    Some(v) => Vec::new(), // todo: v.value().to_le_bytes().to_vec(),
                }
            }

            FBAnnotationEncodeValueType::MethodValue => {
                match meta.value_as_method_meta() {
                    None => Vec::new(),
                    Some(v) => Vec::new(), //todo: v.value().to_le_bytes().to_vec(),
                }
            }

            FBAnnotationEncodeValueType::EnumValue => {
                match meta.value_as_field_meta() {
                    None => Vec::new(),
                    Some(v) => Vec::new(), //todo: v.value().to_le_bytes().to_vec(),
                }
            }

            FBAnnotationEncodeValueType::ArrayValue => {
                match meta.value_as_annotation_encode_array() {
                    None => Vec::new(),
                    Some(v) => Vec::new(), //todo: v.value().to_le_bytes().to_vec(),
                }
            }

            FBAnnotationEncodeValueType::AnnotationValue => match meta.value_as_annotation_meta() {
                None => Vec::new(),
                Some(v) => Vec::new(), //todo: v.value().to_le_bytes().to_vec(),
            },

            FBAnnotationEncodeValueType::NullValue => Vec::new(),
            FBAnnotationEncodeValueType::BoolValue => match meta.value_as_encode_value_boolean() {
                None => Vec::new(),
                Some(v) => Vec::new(), //todo: v.value().to_le_bytes().to_vec(),
            },
            _ => Vec::new(),
        };

        Self {
            value_type: meta_value_tpye.into(),
            value,
        }
    }
}
