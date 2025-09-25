use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationEncodeArrayMatcher as FBAnnotationEncodeArrayMatcher,
    AnnotationEncodeArrayMatcherArgs as FBAnnotationEncodeArrayMatcherArgs,
    AnnotationEncodeValueMatcher as FBAnnotationEncodeValueMatcher,
    AnnotationEncodeValueMatcherWrapper,
    AnnotationEncodeValueMatcherWrapperArgs as FBAnnotationEncodeValueMatcherWrapperArgs,
    MatchType as FBMatchType,
};
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::enums::MatchType;
use crate::query::matchers::base::IntRange;
use crate::query::matchers::base::{AnnotationEncodeValueMatcher, StringMatcher};
use crate::query::matchers::{ClassMatcher, FieldMatcher, MethodMatcher};
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub struct AnnotationEncodeArrayMatcher {
    encode_values_matcher: Option<Vec<AnnotationEncodeValueMatcher>>,
    match_type: MatchType,
    range_matcher: Option<IntRange>,
}

impl Default for AnnotationEncodeArrayMatcher {
    fn default() -> Self {
        AnnotationEncodeArrayMatcher {
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
                    AnnotationEncodeValueMatcherWrapper::create(
                        fbb,
                        &FBAnnotationEncodeValueMatcherWrapperArgs { value_type, value },
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
    pub fn create() -> Self {
        AnnotationEncodeArrayMatcher::default()
    }

    // base
    pub fn set_values_matcher(mut self, matcher: Vec<AnnotationEncodeValueMatcher>) -> Self {
        self.encode_values_matcher = Some(matcher);
        self
    }

    pub fn set_match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = match_type;
        self
    }

    pub fn set_range_matcher(mut self, matcher: IntRange) -> Self {
        self.range_matcher = Some(matcher);
        self
    }

    // extend encode_values_matcher
    pub fn add_value_matcher(mut self, matcher: AnnotationEncodeValueMatcher) -> Self {
        if let Some(ref mut matchers) = self.encode_values_matcher {
            matchers.push(matcher);
        } else {
            self.encode_values_matcher = Some(vec![matcher]);
        }
        self
    }

    pub fn add_byte_value(mut self, value: i8) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_number_byte(value));
        self
    }

    pub fn add_short_value(mut self, value: i16) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_number_short(value));
        self
    }

    pub fn add_int_value(mut self, value: i32) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_number_int(value));
        self
    }

    pub fn add_long_value(mut self, value: i64) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_number_long(value));
        self
    }

    pub fn add_float_value(mut self, value: f32) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_number_float(value));
        self
    }

    pub fn add_double_value(mut self, value: f64) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_number_double(value));
        self
    }

    pub fn add_string_value(mut self, value: StringMatcher) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_string(value));
        self
    }

    pub fn add_string_value_str<S: Into<String>>(mut self, value: S) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_string_str(value));
        self
    }

    pub fn add_class_value(mut self, value: ClassMatcher) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_class(value));
        self
    }

    pub fn add_method_value(mut self, value: MethodMatcher) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_method(value));
        self
    }

    pub fn addd_enum_value(mut self, value: FieldMatcher) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_enum(value));
        self
    }

    pub fn add_annotation_value(mut self, value: AnnotationEncodeArrayMatcher) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_array(value));
        self
    }

    pub fn add_null_value(mut self) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_null());
        self
    }

    pub fn add_bool_value(mut self, value: bool) -> Self {
        self = self.add_value_matcher(AnnotationEncodeValueMatcher::create_bool(value));
        self
    }

    // extend range_matcher
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
