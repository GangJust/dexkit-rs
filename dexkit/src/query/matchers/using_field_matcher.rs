use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{
    FBUsingFieldMatcher, FBUsingFieldMatcherArgs, FBUsingType,
};
use crate::query::base::BaseQuery;
use crate::query::enums::UsingType;
use crate::query::matchers::FieldMatcher;

pub struct UsingFieldMatcher {
    matcher: Option<FieldMatcher>,
    using_type: UsingType,
}

impl Default for UsingFieldMatcher {
    fn default() -> Self {
        Self {
            matcher: None,
            using_type: UsingType::Any,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBUsingFieldMatcher<'a>>> for UsingFieldMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBUsingFieldMatcher<'a>> {
        let field = self.matcher.as_ref().map(|m| m.inner_build(fbb));
        let using_type: FBUsingType = self.using_type.into();

        FBUsingFieldMatcher::create(fbb, &FBUsingFieldMatcherArgs { field, using_type })
    }
}

impl UsingFieldMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl UsingFieldMatcher {
    pub fn matcher(mut self, matcher: FieldMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }

    pub fn using_type(mut self, using_type: UsingType) -> Self {
        self.using_type = using_type;
        self
    }
}
