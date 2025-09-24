use crate::gen_flatbuffers::dexkit::schema::{
    ParameterMatcher as FBParameterMatcher, ParameterMatcherArgs as FBParameterMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::{AnnotationsMatcher, ClassMatcher};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

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
