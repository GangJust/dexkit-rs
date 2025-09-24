use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationEncodeValueMatcher as FBAnnotationEncodeValueMatcher,
    // AnnotationEncodeValueType as FBAnnotationEncodeValueType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationEncodeValueType {
    ByteValue,
    ShortValue,
    CharValue,
    IntValue,
    LongValue,
    FloatValue,
    DoubleValue,
    StringValue,
    TypeValue,
    MethodValue,
    EnumValue,
    ArrayValue,
    AnnotationValue,
    NullValue,
    BoolValue,
}

/*impl From<AnnotationEncodeValueType> for FBAnnotationEncodeValueType {
    fn from(value: AnnotationEncodeValueType) -> Self {
        match value {
            AnnotationEncodeValueType::ByteValue => Self::ByteValue,
            AnnotationEncodeValueType::ShortValue => Self::ShortValue,
            AnnotationEncodeValueType::CharValue => Self::CharValue,
            AnnotationEncodeValueType::IntValue => Self::IntValue,
            AnnotationEncodeValueType::LongValue => Self::LongValue,
            AnnotationEncodeValueType::FloatValue => Self::FloatValue,
            AnnotationEncodeValueType::DoubleValue => Self::DoubleValue,
            AnnotationEncodeValueType::StringValue => Self::StringValue,
            AnnotationEncodeValueType::TypeValue => Self::TypeValue,
            AnnotationEncodeValueType::MethodValue => Self::MethodValue,
            AnnotationEncodeValueType::EnumValue => Self::EnumValue,
            AnnotationEncodeValueType::ArrayValue => Self::ArrayValue,
            AnnotationEncodeValueType::AnnotationValue => Self::AnnotationValue,
            AnnotationEncodeValueType::NullValue => Self::NullValue,
            AnnotationEncodeValueType::BoolValue => Self::BoolValue,
        }
    }
}*/

impl From<AnnotationEncodeValueType> for FBAnnotationEncodeValueMatcher {
    fn from(value: AnnotationEncodeValueType) -> Self {
        match value {
            AnnotationEncodeValueType::ByteValue => Self::EncodeValueByte,
            AnnotationEncodeValueType::ShortValue => Self::EncodeValueShort,
            AnnotationEncodeValueType::CharValue => Self::EncodeValueChar,
            AnnotationEncodeValueType::IntValue => Self::EncodeValueInt,
            AnnotationEncodeValueType::LongValue => Self::EncodeValueLong,
            AnnotationEncodeValueType::FloatValue => Self::EncodeValueFloat,
            AnnotationEncodeValueType::DoubleValue => Self::EncodeValueDouble,
            AnnotationEncodeValueType::StringValue => Self::StringMatcher,
            AnnotationEncodeValueType::TypeValue => Self::ClassMatcher,
            AnnotationEncodeValueType::MethodValue => Self::MethodMatcher,
            AnnotationEncodeValueType::EnumValue => Self::FieldMatcher,
            AnnotationEncodeValueType::ArrayValue => Self::AnnotationEncodeArrayMatcher,
            AnnotationEncodeValueType::AnnotationValue => Self::AnnotationMatcher,
            AnnotationEncodeValueType::NullValue => Self::EncodeValueNull,
            AnnotationEncodeValueType::BoolValue => Self::EncodeValueBoolean,
        }
    }
}
