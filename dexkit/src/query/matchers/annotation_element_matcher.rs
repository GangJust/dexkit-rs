use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationElementMatcher as FBAnnotationElementMatcher,
    AnnotationElementMatcherArgs as FBAnnotationElementMatcherArgs,
    AnnotationEncodeValueMatcher as FBAnnotationEncodeValueMatcher,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::base::AnnotationEncodeValueMatcher;
use crate::query::matchers::base::StringMatcher;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct AnnotationElementMatcher {
    name_matcher: Option<StringMatcher>,
    value_matcher: Option<AnnotationEncodeValueMatcher>,
}

impl Default for AnnotationElementMatcher {
    fn default() -> Self {
        AnnotationElementMatcher {
            name_matcher: None,
            value_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBAnnotationElementMatcher<'a>>> for AnnotationElementMatcher {
    fn inner_build(
        &self,
        fbb: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBAnnotationElementMatcher<'a>> {
        let name = self.name_matcher.as_ref().map(|m| m.inner_build(fbb));
        let value_type: FBAnnotationEncodeValueMatcher = if let Some(matcher) = &self.value_matcher
        {
            matcher.into()
        } else {
            FBAnnotationEncodeValueMatcher::NONE
        };
        let value = self
            .value_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb))
            .unwrap_or(None);

        FBAnnotationElementMatcher::create(
            fbb,
            &FBAnnotationElementMatcherArgs {
                name,
                value_type,
                value,
            },
        )
    }
}

impl AnnotationElementMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_name_matcher(mut self, matcher: StringMatcher) -> Self {
        self.name_matcher = Some(matcher);
        self
    }

    pub fn set_value_matcher(mut self, matcher: AnnotationEncodeValueMatcher) -> Self {
        self.value_matcher = Some(matcher);
        self
    }
}
