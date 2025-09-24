use crate::gen_flatbuffers::dexkit::schema::{
    MatchType as FBMatchType, MethodsMatcher as FBMethodsMatcher,
    MethodsMatcherArgs as FBMethodsMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use crate::query::matchers::MethodMatcher;
use crate::query::matchers::base::IntRange;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct MethodsMatcher {
    methods_matcher: Option<Vec<MethodMatcher>>,
    match_type: MatchType,
    range_matcher: Option<IntRange>,
}

impl Default for MethodsMatcher {
    fn default() -> Self {
        MethodsMatcher {
            methods_matcher: None,
            match_type: MatchType::Contains,
            range_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBMethodsMatcher<'a>>> for MethodsMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBMethodsMatcher<'a>> {
        let methods = self.methods_matcher.as_ref().map(|matchers| {
            let fb_methods: Vec<_> = matchers.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&fb_methods)
        });
        let match_type: FBMatchType = self.match_type.into();
        let method_count = self.range_matcher.as_ref().map(|r| r.inner_build(fbb));

        FBMethodsMatcher::create(
            fbb,
            &FBMethodsMatcherArgs {
                methods,
                match_type,
                method_count,
            },
        )
    }
}

impl MethodsMatcher {
    pub fn create() -> Self {
        MethodsMatcher::default()
    }

    // base
    pub fn set_methods_matcher(mut self, matchers: Vec<MethodMatcher>) -> Self {
        self.methods_matcher = Some(matchers);
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

    // extend methods_matcher
    pub fn add_method_matcher(mut self, matcher: MethodMatcher) -> Self {
        self.methods_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
        self
    }
}
