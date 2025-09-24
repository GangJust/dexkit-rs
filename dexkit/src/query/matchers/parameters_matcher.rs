use crate::gen_flatbuffers::dexkit::schema::{
    ParametersMatcher as FBParametersMatcher, ParametersMatcherArgs as FBParametersMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::base::IntRange;
use crate::query::matchers::ParameterMatcher;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct ParametersMatcher {
    params_matcher: Option<Vec<Option<ParameterMatcher>>>,
    range_matcher: Option<IntRange>,
}

impl Default for ParametersMatcher {
    fn default() -> Self {
        Self {
            params_matcher: None,
            range_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBParametersMatcher<'a>>> for ParametersMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBParametersMatcher<'a>> {
        let parameters = self.params_matcher.as_ref().map(|params| {
            let fb_params: Vec<_> = params
                .iter()
                .map(|param_opt| {
                    param_opt
                        .as_ref()
                        .map(|param| param.inner_build(fbb))
                        .unwrap_or_else(|| ParameterMatcher::default().inner_build(fbb))
                })
                .collect();
            fbb.create_vector(&fb_params)
        });
        let parameter_count = self
            .range_matcher
            .as_ref()
            .map(|range| range.inner_build(fbb));

        FBParametersMatcher::create(
            fbb,
            &FBParametersMatcherArgs {
                parameters,
                parameter_count,
            },
        )
    }
}
