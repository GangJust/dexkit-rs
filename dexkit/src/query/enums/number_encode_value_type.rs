use crate::gen_flatbuffers::dexkit::schema::{
    Number as FBNumber,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberEncodeValueType {
    ByteValue,
    ShortValue,
    IntValue,
    LongValue,
    FloatValue,
    DoubleValue,
}

impl From<NumberEncodeValueType> for FBNumber {
    fn from(value: NumberEncodeValueType) -> Self {
        match value {
            NumberEncodeValueType::ByteValue => FBNumber::EncodeValueByte,
            NumberEncodeValueType::ShortValue => FBNumber::EncodeValueShort,
            NumberEncodeValueType::IntValue => FBNumber::EncodeValueInt,
            NumberEncodeValueType::LongValue => FBNumber::EncodeValueLong,
            NumberEncodeValueType::FloatValue => FBNumber::EncodeValueFloat,
            NumberEncodeValueType::DoubleValue => FBNumber::EncodeValueDouble,
        }
    }
}
