use crate::gen_flatbuffers::dexkit::schema::{
    ParametersMatcher as FBParametersMatcher, ParametersMatcherArgs as FBParametersMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::ParameterMatcher;
use crate::query::matchers::base::IntRange;
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

impl ParametersMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_params_matcher(mut self, params: Vec<Option<ParameterMatcher>>) -> Self {
        self.params_matcher = Some(params);
        self
    }

    pub fn set_range_matcher(mut self, range: IntRange) -> Self {
        self.range_matcher = Some(range);
        self
    }

    // extend params_matcher
    pub fn add_param_matchers(mut self, params: Vec<Option<ParameterMatcher>>) -> Self {
        if let Some(ref mut existing_params) = self.params_matcher {
            existing_params.extend(params);
        } else {
            self.params_matcher = Some(params);
        }
        self
    }

    pub fn add_param_matcher(mut self, param: Option<ParameterMatcher>) -> Self {
        if let Some(ref mut params) = self.params_matcher {
            params.push(param);
        } else {
            self.params_matcher = Some(vec![param]);
        }
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
