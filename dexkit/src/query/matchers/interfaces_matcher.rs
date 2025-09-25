use crate::gen_flatbuffers::dexkit::schema::{
    InterfacesMatcher as FBInterfacesMatcher, InterfacesMatcherArgs as FBInterfacesMatcherArgs,
    MatchType as FBMatchType,
};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use crate::query::matchers::ClassMatcher;
use crate::query::matchers::base::IntRange;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct InterfacesMatcher {
    interface_matcher: Option<Vec<ClassMatcher>>,
    match_type: MatchType,
    range_matcher: Option<IntRange>,
}

impl Default for InterfacesMatcher {
    fn default() -> Self {
        Self {
            interface_matcher: None,
            match_type: MatchType::Contains,
            range_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBInterfacesMatcher<'a>>> for InterfacesMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBInterfacesMatcher<'a>> {
        let interfaces = self.interface_matcher.as_ref().map(|matchers| {
            let fb_matchers: Vec<WIPOffset<_>> =
                matchers.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&fb_matchers)
        });
        let match_type: FBMatchType = self.match_type.into();
        let interface_count = self.range_matcher.as_ref().map(|r| r.inner_build(fbb));

        FBInterfacesMatcher::create(
            fbb,
            &FBInterfacesMatcherArgs {
                interfaces,
                match_type,
                interface_count,
            },
        )
    }
}

impl InterfacesMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_interface_name_matcher(mut self, matchers: Vec<ClassMatcher>) -> Self {
        self.interface_matcher = Some(matchers);
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

    // extend interface_name_matcher
    pub fn add_interface_name_matchers(mut self, matchers: Vec<ClassMatcher>) -> Self {
        self.interface_matcher
            .get_or_insert_with(Vec::new)
            .extend(matchers);
        self
    }

    pub fn add_interface_name_matcher(mut self, matcher: ClassMatcher) -> Self {
        self.interface_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
        self
    }

    pub fn set_interface_name_strs<S: Into<String>>(mut self, names: Vec<S>) -> Self {
        let matchers: Vec<ClassMatcher> = names
            .into_iter()
            .map(|name| ClassMatcher::create().set_class_name_str(name))
            .collect();
        self.interface_matcher = Some(matchers);
        self
    }

    pub fn add_interface_name_strs<S: Into<String>>(mut self, names: Vec<S>) -> Self {
        let matchers: Vec<ClassMatcher> = names
            .into_iter()
            .map(|name| ClassMatcher::create().set_class_name_str(name))
            .collect();
        self.interface_matcher
            .get_or_insert_with(Vec::new)
            .extend(matchers);
        self
    }

    pub fn add_interface_name_str<S: Into<String>>(mut self, name: S) -> Self {
        let matcher = ClassMatcher::create().set_class_name_str(name);
        self.interface_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
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
