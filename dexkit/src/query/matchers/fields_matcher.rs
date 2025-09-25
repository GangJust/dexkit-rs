use crate::gen_flatbuffers::dexkit::schema::{
    FieldsMatcher as FBFieldsMatcher, FieldsMatcherArgs as FBFieldsMatcherArgs,
    MatchType as FBMatchType,
};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use crate::query::matchers::FieldMatcher;
use crate::query::matchers::base::IntRange;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct FieldsMatcher {
    fields_matcher: Option<Vec<FieldMatcher>>,
    match_type: MatchType,
    range_matcher: Option<IntRange>,
}

impl Default for FieldsMatcher {
    fn default() -> Self {
        Self {
            fields_matcher: None,
            match_type: MatchType::Contains,
            range_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBFieldsMatcher<'a>>> for FieldsMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBFieldsMatcher<'a>> {
        let fields = self.fields_matcher.as_ref().map(|matchers| {
            let fb_field_matchers: Vec<
                WIPOffset<crate::gen_flatbuffers::dexkit::schema::FieldMatcher>,
            > = matchers.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&fb_field_matchers)
        });
        let match_type: FBMatchType = self.match_type.into();
        let field_count = self.range_matcher.as_ref().map(|r| r.inner_build(fbb));

        FBFieldsMatcher::create(
            fbb,
            &FBFieldsMatcherArgs {
                fields,
                match_type,
                field_count,
            },
        )
    }
}

impl FieldsMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_fields_matcher(mut self, matchers: Vec<FieldMatcher>) -> Self {
        self.fields_matcher = Some(matchers);
        self
    }

    pub fn add_field_matcher(mut self, matcher: FieldMatcher) -> Self {
        self.fields_matcher
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

    // extend fields_matcher
    pub fn set_fields(mut self, matchers: Vec<FieldMatcher>) -> Self {
        self.fields_matcher = Some(matchers);
        self
    }

    pub fn set_field_name_strs(mut self, field_names: Vec<String>) -> Self {
        let matchers: Vec<FieldMatcher> = field_names
            .into_iter()
            .map(|name| FieldMatcher::create().set_field_name_str(name))
            .collect();
        self.fields_matcher = Some(matchers);
        self
    }

    pub fn add_field_names_strs<S: Into<String>>(mut self, field_names: Vec<S>) -> Self {
        let matchers: Vec<FieldMatcher> = field_names
            .into_iter()
            .map(|name| FieldMatcher::create().set_field_name_str(name))
            .collect();
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(matchers);
        } else {
            self.fields_matcher
                .as_mut()
                .unwrap()
                .extend(matchers.into_iter());
        }
        self
    }

    pub fn add_field_name_str<S: Into<String>>(mut self, field_name: S) -> Self {
        self.fields_matcher
            .get_or_insert_with(Vec::new)
            .push(FieldMatcher::create().set_field_name_str(field_name));
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
