use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{
    FBAnnotationEncodeArrayMatcher, FBAnnotationEncodeArrayMatcherArgs,
    FBAnnotationEncodeValueMatcher, FBAnnotationEncodeValueMatcherUnion,
    FBAnnotationEncodeValueMatcherUnionArgs, FBMatchType,
};
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::enums::MatchType;
use crate::query::matchers::{
    AnnotationEncodeValueMatcher, ClassMatcher, FieldMatcher, IntRange, MethodMatcher,
    StringMatcher,
};

pub struct AnnotationEncodeArrayMatcher {
    encode_values_matcher: Option<Vec<AnnotationEncodeValueMatcher>>,
    match_type: MatchType,
    range_matcher: Option<IntRange>,
}

impl Default for AnnotationEncodeArrayMatcher {
    fn default() -> Self {
        Self {
            encode_values_matcher: None,
            match_type: MatchType::Contains,
            range_matcher: None,
        }
    }
}

impl IAnnotationEncodeValue for AnnotationEncodeArrayMatcher {
    fn inner_build_annotation_union(
        &self,
        fbb: &mut FlatBufferBuilder,
    ) -> WIPOffset<UnionWIPOffset> {
        self.inner_build(fbb).as_union_value()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBAnnotationEncodeArrayMatcher<'a>>>
    for AnnotationEncodeArrayMatcher
{
    fn inner_build(
        &self,
        fbb: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBAnnotationEncodeArrayMatcher<'a>> {
        let values = self.encode_values_matcher.as_ref().map(|matchers| {
            let offsets: Vec<_> = matchers
                .iter()
                .map(|m| {
                    let value_type: FBAnnotationEncodeValueMatcher = m.into();
                    let value = m.inner_build(fbb);
                    FBAnnotationEncodeValueMatcherUnion::create(
                        fbb,
                        &FBAnnotationEncodeValueMatcherUnionArgs { value_type, value },
                    )
                })
                .collect();
            fbb.create_vector(&offsets)
        });
        let match_type: FBMatchType = self.match_type.into();
        let value_count = self.range_matcher.as_ref().map(|r| r.inner_build(fbb));

        FBAnnotationEncodeArrayMatcher::create(
            fbb,
            &FBAnnotationEncodeArrayMatcherArgs {
                values,
                match_type,
                value_count,
            },
        )
    }
}

impl AnnotationEncodeArrayMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AnnotationEncodeArrayMatcher {
    pub fn values<I>(mut self, matchers: I) -> Self
    where
        I: IntoIterator<Item = AnnotationEncodeValueMatcher>,
    {
        self.encode_values_matcher = Some(matchers.into_iter().collect());
        self
    }

    pub fn match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = match_type;
        self
    }

    pub fn range(mut self, range: IntRange) -> Self {
        self.range_matcher = Some(range);
        self
    }
}

impl AnnotationEncodeArrayMatcher {
    pub fn add_value(mut self, matcher: AnnotationEncodeValueMatcher) -> Self {
        if let Some(ref mut matchers) = self.encode_values_matcher {
            matchers.push(matcher);
        } else {
            self.encode_values_matcher = Some(vec![matcher]);
        }
        self
    }
}

impl AnnotationEncodeArrayMatcher {
    pub fn byte_value(mut self, value: i8) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::byte(value));
        self
    }

    pub fn short_value(mut self, value: i16) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::short(value));
        self
    }

    pub fn int_value(mut self, value: i32) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::int(value));
        self
    }

    pub fn long_value(mut self, value: i64) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::long(value));
        self
    }

    pub fn float_value(mut self, value: f32) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::float(value));
        self
    }

    pub fn double_value(mut self, value: f64) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::double(value));
        self
    }

    pub fn string_value(mut self, value: StringMatcher) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::string(value));
        self
    }

    pub fn class_value(mut self, value: ClassMatcher) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::class(value));
        self
    }

    pub fn method_value(mut self, value: MethodMatcher) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::method(value));
        self
    }

    pub fn enum_value(mut self, value: FieldMatcher) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::enum_value(value));
        self
    }

    pub fn array_value(mut self, value: AnnotationEncodeArrayMatcher) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::array(value));
        self
    }

    pub fn null_value(mut self) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::null());
        self
    }

    pub fn bool_value(mut self, value: bool) -> Self {
        self = self.add_value(AnnotationEncodeValueMatcher::bool(value));
        self
    }
}

impl AnnotationEncodeArrayMatcher {
    pub fn count(mut self, count: u32) -> Self {
        self.range_matcher = Some(IntRange::exactly(count));
        self
    }

    pub fn count_range(mut self, min: u32, max: u32) -> Self {
        self.range_matcher = Some(IntRange::range(min, max));
        self
    }

    pub fn count_min(mut self, min: u32) -> Self {
        self.range_matcher = Some(IntRange::at_least(min));
        self
    }

    pub fn count_max(mut self, max: u32) -> Self {
        self.range_matcher = Some(IntRange::at_most(max));
        self
    }
}
