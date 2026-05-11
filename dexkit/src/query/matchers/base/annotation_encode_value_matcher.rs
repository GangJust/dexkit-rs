use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::FBAnnotationEncodeValueMatcher;
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::enums::AnnotationEncodeValueType;
use crate::query::matchers::{
    AnnotationEncodeArrayMatcher, AnnotationMatcher, ClassMatcher, EncodeValueBoolean,
    EncodeValueByte, EncodeValueChar, EncodeValueDouble, EncodeValueFloat, EncodeValueInt,
    EncodeValueLong, EncodeValueNull, EncodeValueShort, FieldMatcher, MethodMatcher,
    StringMatcher,
};

pub struct AnnotationEncodeValueMatcher {
    value: Option<Box<dyn IAnnotationEncodeValue>>,
    value_type: Option<AnnotationEncodeValueType>,
}

impl Default for AnnotationEncodeValueMatcher {
    fn default() -> Self {
        Self {
            value: None,
            value_type: None,
        }
    }
}

impl<'a> BaseQuery<'a, Option<WIPOffset<UnionWIPOffset>>> for AnnotationEncodeValueMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> Option<WIPOffset<UnionWIPOffset>> {
        if let Some(value) = &self.value {
            Some(value.inner_build_annotation_union(fbb))
        } else {
            None
        }
    }
}

impl From<&AnnotationEncodeValueMatcher> for FBAnnotationEncodeValueMatcher {
    fn from(matcher: &AnnotationEncodeValueMatcher) -> Self {
        if let Some(value_type) = matcher.value_type {
            value_type.into()
        } else {
            FBAnnotationEncodeValueMatcher::NONE
        }
    }
}

impl AnnotationEncodeValueMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AnnotationEncodeValueMatcher {
    pub fn byte(byte: i8) -> Self {
        Self {
            value: Some(Box::new(EncodeValueByte(byte))),
            value_type: Some(AnnotationEncodeValueType::ByteValue),
        }
    }

    pub fn short(short: i16) -> Self {
        Self {
            value: Some(Box::new(EncodeValueShort(short))),
            value_type: Some(AnnotationEncodeValueType::ShortValue),
        }
    }

    pub fn char(value: char) -> Self {
        Self {
            value: Some(Box::new(EncodeValueChar(value))),
            value_type: Some(AnnotationEncodeValueType::CharValue),
        }
    }

    pub fn int(int: i32) -> Self {
        Self {
            value: Some(Box::new(EncodeValueInt(int))),
            value_type: Some(AnnotationEncodeValueType::IntValue),
        }
    }

    pub fn long(long: i64) -> Self {
        Self {
            value: Some(Box::new(EncodeValueLong(long))),
            value_type: Some(AnnotationEncodeValueType::LongValue),
        }
    }

    pub fn float(float: f32) -> Self {
        Self {
            value: Some(Box::new(EncodeValueFloat(float))),
            value_type: Some(AnnotationEncodeValueType::FloatValue),
        }
    }

    pub fn double(double: f64) -> Self {
        Self {
            value: Some(Box::new(EncodeValueDouble(double))),
            value_type: Some(AnnotationEncodeValueType::DoubleValue),
        }
    }

    pub fn string(value: StringMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::StringValue),
        }
    }

    pub fn class(value: ClassMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::TypeValue),
        }
    }

    pub fn method(value: MethodMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::MethodValue),
        }
    }

    pub fn enum_value(value: FieldMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::EnumValue),
        }
    }

    pub fn array(value: AnnotationEncodeArrayMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::ArrayValue),
        }
    }

    pub fn annotation(value: AnnotationMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::AnnotationValue),
        }
    }

    pub fn null() -> Self {
        Self {
            value: Some(Box::new(EncodeValueNull)),
            value_type: Some(AnnotationEncodeValueType::NullValue),
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            value: Some(Box::new(EncodeValueBoolean(value))),
            value_type: Some(AnnotationEncodeValueType::BoolValue),
        }
    }
}

impl AnnotationEncodeValueMatcher {
    pub(crate) fn set_value(mut self, value: Box<dyn IAnnotationEncodeValue>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn value_type(mut self, value_type: AnnotationEncodeValueType) -> Self {
        self.value_type = Some(value_type);
        self
    }
}
