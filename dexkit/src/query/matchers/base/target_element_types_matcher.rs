use crate::gen_flatbuffers::dexkit::schema::{
    MatchType as FBMatchType, TargetElementType as FBTargetElementType,
    TargetElementTypesMatcher as FBTargetElementTypesMatcher,
    TargetElementTypesMatcherArgs as FBTargetElementTypesMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::enums::MatchType;
use crate::query::enums::TargetElementType;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct TargetElementTypesMatcher {
    types: Option<Vec<TargetElementType>>,
    match_type: MatchType,
}

impl Default for TargetElementTypesMatcher {
    fn default() -> Self {
        TargetElementTypesMatcher {
            types: None,
            match_type: MatchType::Contains,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBTargetElementTypesMatcher<'a>>> for TargetElementTypesMatcher {
    fn inner_build(
        &self,
        fbb: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBTargetElementTypesMatcher<'a>> {
        let types = self.types.as_ref().map(|types| {
            let fb_types: Vec<FBTargetElementType> =
                types.iter().copied().map(|t| t.into()).collect();
            fbb.create_vector(&fb_types)
        });
        let match_type: FBMatchType = self.match_type.into();

        let root = FBTargetElementTypesMatcher::create(
            fbb,
            &FBTargetElementTypesMatcherArgs { types, match_type },
        );
        root
    }
}

impl TargetElementTypesMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_types(mut self, types: Vec<TargetElementType>) -> Self {
        self.types = Some(types);
        self
    }

    pub fn set_match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = match_type;
        self
    }

    // extend types
    pub fn add_target_element_type(mut self, element_type: TargetElementType) -> Self {
        if let Some(types) = &mut self.types {
            types.push(element_type);
        } else {
            self.types = Some(vec![element_type]);
        }
        self
    }
}
