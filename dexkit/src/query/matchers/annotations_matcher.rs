use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationsMatcher as FBAnnotationsMatcher, AnnotationsMatcherArgs as FBAnnotationsMatcherArgs,
    MatchType as FBMatchType,
};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use crate::query::matchers::AnnotationMatcher;
use crate::query::matchers::base::IntRange;
use flatbuffers::WIPOffset;

pub struct AnnotationsMatcher {
    annotations_matcher: Option<Vec<AnnotationMatcher>>,
    match_type: MatchType,
    range_matcher: Option<IntRange>,
}

impl Default for AnnotationsMatcher {
    fn default() -> Self {
        Self {
            annotations_matcher: None,
            match_type: MatchType::Contains,
            range_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBAnnotationsMatcher<'a>>> for AnnotationsMatcher {
    fn inner_build(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBAnnotationsMatcher<'a>> {
        let annotations = self.annotations_matcher.as_ref().map(|matchers| {
            let offsets: Vec<_> = matchers.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&offsets)
        });
        let match_type: FBMatchType = self.match_type.into();
        let annotation_count = self.range_matcher.as_ref().map(|r| r.inner_build(fbb));

        FBAnnotationsMatcher::create(
            fbb,
            &FBAnnotationsMatcherArgs {
                annotations,
                match_type,
                annotation_count,
            },
        )
    }
}

impl AnnotationsMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // bese
    pub fn set_annotations_matcher(mut self, matchers: Vec<AnnotationMatcher>) -> Self {
        self.annotations_matcher = Some(matchers);
        self
    }

    pub fn add_annotation_matcher(mut self, matcher: AnnotationMatcher) -> Self {
        self.annotations_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
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

    // extend annotations_matcher
    pub fn add_annotation_type_class_name_strs<S: Into<String>>(
        mut self,
        annotations: Vec<S>,
    ) -> Self {
        if self.annotations_matcher.is_none() {
            self.set_annotations_matcher(
                annotations
                    .into_iter()
                    .map(|s| AnnotationMatcher::create().set_type_class_name(s))
                    .collect(),
            )
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|mut am| {
                am.extend(
                    annotations
                        .into_iter()
                        .map(|s| AnnotationMatcher::create().set_type_class_name(s)),
                );
                am
            });
            self
        }
    }
}
