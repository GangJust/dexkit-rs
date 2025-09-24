use crate::gen_flatbuffers::dexkit::schema::Number as FBNumber;
use crate::query::base::{BaseQuery, INumberEncodeValue};
use crate::query::enums::NumberEncodeValueType;
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

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
