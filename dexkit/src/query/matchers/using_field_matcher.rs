use crate::gen_flatbuffers::dexkit::schema::{
    UsingFieldMatcher as FBUsingFieldMatcher, UsingFieldMatcherArgs as FBUsingFieldMatcherArgs,
    UsingType as FBUsingType,
};
use crate::query::base::BaseQuery;
use crate::query::enums::UsingType;
use crate::query::matchers::FieldMatcher;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct UsingFieldMatcher {
    matcher: Option<FieldMatcher>,
    using_type: UsingType,
}

impl Default for UsingFieldMatcher {
    fn default() -> Self {
        UsingFieldMatcher {
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
