use crate::gen_flatbuffers::dexkit::schema::{
    AccessFlagsMatcher as FBAccessFlagsMatcher, AccessFlagsMatcherArgs as FBAccessFlagsMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct AccessFlagsMatcher {
    modifiers: u32,
    match_type: MatchType,
}

impl Default for AccessFlagsMatcher {
    fn default() -> Self {
        Self {
            modifiers: 0,
            match_type: MatchType::default(),
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBAccessFlagsMatcher<'a>>> for AccessFlagsMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBAccessFlagsMatcher<'a>> {
        let match_type: crate::gen_flatbuffers::dexkit::schema::MatchType = self.match_type.into();
        let flags = self.modifiers;

        if flags == 0 {
            panic!("AccessFlagsMatcher requires at least one modifier to be set.");
        }

        let root =
            FBAccessFlagsMatcher::create(fbb, &FBAccessFlagsMatcherArgs { flags, match_type });
        root
    }
}

impl AccessFlagsMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_modifiers(mut self, modifiers: u32) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn set_match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = match_type;
        self
    }

    // extend modifiers
    pub fn or_modifiers(mut self, modifiers: u32) -> Self {
        self.modifiers |= modifiers;
        self
    }
}
