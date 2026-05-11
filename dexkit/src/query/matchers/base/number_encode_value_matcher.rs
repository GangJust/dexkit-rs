use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::FBNumber;
use crate::query::base::{BaseQuery, INumberEncodeValue};
use crate::query::enums::NumberEncodeValueType;
use crate::query::matchers::{
    EncodeValueByte, EncodeValueDouble, EncodeValueFloat, EncodeValueInt, EncodeValueLong,
    EncodeValueShort,
};

pub struct NumberEncodeValueMatcher {
    value: Option<Box<dyn INumberEncodeValue>>,
    value_type: Option<NumberEncodeValueType>,
}

impl Default for NumberEncodeValueMatcher {
    fn default() -> Self {
        NumberEncodeValueMatcher {
            value: None,
            value_type: None,
        }
    }
}

// marker..
impl<'a> BaseQuery<'a, Option<WIPOffset<UnionWIPOffset>>> for NumberEncodeValueMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> Option<WIPOffset<UnionWIPOffset>> {
        if let Some(value) = &self.value {
            Some(value.inner_build_number_union(fbb))
        } else {
            None
        }
    }
}

// marker..
impl From<&NumberEncodeValueMatcher> for FBNumber {
    fn from(matcher: &NumberEncodeValueMatcher) -> Self {
        if let Some(value_type) = matcher.value_type {
            value_type.into()
        } else {
            FBNumber::NONE
        }
    }
}

impl NumberEncodeValueMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NumberEncodeValueMatcher {
    pub fn byte(value: i8) -> Self {
        Self {
            value: Some(Box::new(EncodeValueByte(value))),
            value_type: Some(NumberEncodeValueType::ByteValue),
        }
    }

    pub fn short(value: i16) -> Self {
        Self {
            value: Some(Box::new(EncodeValueShort(value))),
            value_type: Some(NumberEncodeValueType::ShortValue),
        }
    }

    pub fn int(value: i32) -> Self {
        Self {
            value: Some(Box::new(EncodeValueInt(value))),
            value_type: Some(NumberEncodeValueType::IntValue),
        }
    }

    pub fn long(value: i64) -> Self {
        Self {
            value: Some(Box::new(EncodeValueLong(value))),
            value_type: Some(NumberEncodeValueType::LongValue),
        }
    }

    pub fn float(value: f32) -> Self {
        Self {
            value: Some(Box::new(EncodeValueFloat(value))),
            value_type: Some(NumberEncodeValueType::FloatValue),
        }
    }

    pub fn double(value: f64) -> Self {
        Self {
            value: Some(Box::new(EncodeValueDouble(value))),
            value_type: Some(NumberEncodeValueType::DoubleValue),
        }
    }
}

impl NumberEncodeValueMatcher {
    pub(crate) fn set_value(mut self, value: Box<dyn INumberEncodeValue>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn value_type(mut self, value_type: NumberEncodeValueType) -> Self {
        self.value_type = Some(value_type);
        self
    }
}
