use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBParameterMatcher, FBParameterMatcherArgs};
use crate::query::base::BaseQuery;
use crate::query::matchers::{AnnotationsMatcher, ClassMatcher};

pub struct ParameterMatcher {
    annotations_matcher: Option<AnnotationsMatcher>,
    type_matcher: Option<ClassMatcher>,
}

impl Default for ParameterMatcher {
    fn default() -> Self {
        ParameterMatcher {
            annotations_matcher: None,
            type_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBParameterMatcher<'a>>> for ParameterMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBParameterMatcher<'a>> {
        let annotations = self
            .annotations_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let parameter_type = self.type_matcher.as_ref().map(|m| m.inner_build(fbb));

        FBParameterMatcher::create(
            fbb,
            &FBParameterMatcherArgs {
                annotations,
                parameter_type,
            },
        )
    }
}

impl ParameterMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ParameterMatcher {
    pub fn annotations(mut self, matcher: AnnotationsMatcher) -> Self {
        self.annotations_matcher = Some(matcher);
        self
    }

    pub fn parameter_type(mut self, matcher: ClassMatcher) -> Self {
        self.type_matcher = Some(matcher);
        self
    }
}
