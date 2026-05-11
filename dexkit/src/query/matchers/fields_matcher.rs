use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBFieldsMatcher, FBFieldsMatcherArgs, FBMatchType};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use crate::query::matchers::{FieldMatcher, IntRange};

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
                WIPOffset<crate::gen_flatbuffers::dexkit::fb::FBFieldMatcher>,
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
    pub fn new() -> Self {
        Self::default()
    }
}

impl FieldsMatcher {
    pub fn fields<I>(mut self, matchers: I) -> Self
    where
        I: IntoIterator<Item = FieldMatcher>,
    {
        self.fields_matcher = Some(matchers.into_iter().collect());
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
}

impl FieldsMatcher {
    pub fn add_field(mut self, matcher: FieldMatcher) -> Self {
        self.fields_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
        self
    }
}

impl FieldsMatcher {
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
