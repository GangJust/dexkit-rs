use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBMatchType, FBMethodsMatcher, FBMethodsMatcherArgs};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use crate::query::matchers::{IntRange, MethodMatcher};

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
    pub fn new() -> Self {
        MethodsMatcher::default()
    }

    // base
    pub fn methods(mut self, matchers: Vec<MethodMatcher>) -> Self {
        self.methods_matcher = Some(matchers);
        self
    }

    pub fn match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = match_type;
        self
    }

    pub fn range(mut self, range: IntRange) -> Self {
        self.range_matcher = Some(range);
        self
    }

    // extend methods_matcher
    pub fn method(mut self, matcher: MethodMatcher) -> Self {
        self.methods_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
        self
    }

    pub fn method_names<S>(self, method_name: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        let matchers: Vec<MethodMatcher> = method_name
            .into_iter()
            .map(|name| MethodMatcher::new().name_contains(name))
            .collect();
        self.methods(matchers)
    }

    pub fn method_name<S>(self, method_name: S) -> Self
    where
        S: Into<String>,
    {
        self.method(MethodMatcher::new().name_contains(method_name))
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
