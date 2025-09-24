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
use crate::query::matchers::base::AnnotationEncodeValueMatcher;
use crate::query::matchers::base::IntRange;
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
    pub fn set_encode_values_matcher(mut self, matcher: Vec<AnnotationEncodeValueMatcher>) -> Self {
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
}
