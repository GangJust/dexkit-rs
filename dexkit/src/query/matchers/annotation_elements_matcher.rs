use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationElementsMatcher as FBAnnotationElementsMatcher,
    AnnotationElementsMatcherArgs as FBAnnotationElementsMatcherArgs, MatchType as FBMatchType,
};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use crate::query::matchers::base::IntRange;
use crate::query::matchers::AnnotationElementMatcher;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct AnnotationElementsMatcher {
    elements_matcher: Option<Vec<AnnotationElementMatcher>>,
    match_type: MatchType,
    range_matcher: Option<IntRange>,
}

impl Default for AnnotationElementsMatcher {
    fn default() -> Self {
        AnnotationElementsMatcher {
            elements_matcher: None,
            match_type: MatchType::Contains,
            range_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBAnnotationElementsMatcher<'a>>> for AnnotationElementsMatcher {
    fn inner_build(
        &self,
        fbb: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBAnnotationElementsMatcher<'a>> {
        let elements = self.elements_matcher.as_ref().map(|matchers| {
            let offsets: Vec<_> = matchers.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&offsets)
        });
        let match_type: FBMatchType = self.match_type.into();
        let element_count = self.range_matcher.as_ref().map(|r| r.inner_build(fbb));

        FBAnnotationElementsMatcher::create(
            fbb,
            &FBAnnotationElementsMatcherArgs {
                elements,
                match_type,
                element_count,
            },
        )
    }
}

impl AnnotationElementsMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_elements_matcher(mut self, matcher: Vec<AnnotationElementMatcher>) -> Self {
        self.elements_matcher = Some(matcher);
        self
    }

    pub fn set_match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = match_type;
        self
    }

    pub fn set_range_matcher(mut self, range: IntRange) -> Self {
        self.range_matcher = Some(range);
        self
    }
}
