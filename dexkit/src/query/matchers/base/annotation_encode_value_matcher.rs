use crate::gen_flatbuffers::dexkit::schema::AnnotationEncodeValueMatcher as FBAnnotationEncodeValueMatcher;
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::enums::AnnotationEncodeValueType;
use crate::query::matchers::base::StringMatcher;
use crate::query::matchers::{
    ClassMatcher, EncodeValueByte, EncodeValueDouble, EncodeValueFloat, EncodeValueInt,
    EncodeValueLong, EncodeValueShort, MethodMatcher,
};
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub struct AnnotationEncodeValueMatcher {
    value: Option<Box<dyn IAnnotationEncodeValue>>,
    value_type: Option<AnnotationEncodeValueType>,
}

impl Default for AnnotationEncodeValueMatcher {
    fn default() -> Self {
        AnnotationEncodeValueMatcher {
            value: None,
            value_type: None,
        }
    }
}

// marker..
impl<'a> BaseQuery<'a, Option<WIPOffset<UnionWIPOffset>>> for AnnotationEncodeValueMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> Option<WIPOffset<UnionWIPOffset>> {
        if let Some(value) = &self.value {
            Some(value.inner_build_annotation_union(fbb))
        } else {
            None
        }
    }
}

// marker..
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
    // create
    pub fn create() -> Self {
        Self::default()
    }

    pub fn create_number_byte(byte: i8) -> Self {
        Self {
            value: Some(Box::new(EncodeValueByte(byte))),
            value_type: Some(AnnotationEncodeValueType::ByteValue),
        }
    }

    pub fn create_number_short(short: i16) -> Self {
        Self {
            value: Some(Box::new(EncodeValueShort(short))),
            value_type: Some(AnnotationEncodeValueType::ShortValue),
        }
    }

    pub fn create_number_int(int: i32) -> Self {
        Self {
            value: Some(Box::new(EncodeValueInt(int))),
            value_type: Some(AnnotationEncodeValueType::IntValue),
        }
    }

    pub fn create_number_long(long: i64) -> Self {
        Self {
            value: Some(Box::new(EncodeValueLong(long))),
            value_type: Some(AnnotationEncodeValueType::LongValue),
        }
    }

    pub fn create_number_float(float: f32) -> Self {
        Self {
            value: Some(Box::new(EncodeValueFloat(float))),
            value_type: Some(AnnotationEncodeValueType::FloatValue),
        }
    }

    pub fn create_number_double(double: f64) -> Self {
        Self {
            value: Some(Box::new(EncodeValueDouble(double))),
            value_type: Some(AnnotationEncodeValueType::DoubleValue),
        }
    }

    pub fn create_string(value: StringMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::StringValue),
        }
    }

    pub fn create_class(value: ClassMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::TypeValue),
        }
    }

    pub fn create_method(value: MethodMatcher) -> Self {
        Self {
            value: Some(Box::new(value)),
            value_type: Some(AnnotationEncodeValueType::MethodValue),
        }
    }

    // base
    pub fn set_value(mut self, value: Box<dyn IAnnotationEncodeValue>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn set_value_type(mut self, value_type: AnnotationEncodeValueType) -> Self {
        self.value_type = Some(value_type);
        self
    }

    // extend value
    pub fn create_string_str<S: Into<String>>(value: S) -> Self {
        Self {
            value: Some(Box::new(StringMatcher::create_string_str(value))),
            value_type: Some(AnnotationEncodeValueType::StringValue),
        }
    }
}
