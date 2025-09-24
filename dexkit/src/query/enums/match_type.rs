use crate::gen_flatbuffers::dexkit::schema::MatchType as FBMatchType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    Contains,
    Equals,
}

impl Default for MatchType {
    fn default() -> Self {
        Self::Contains
    }
}

impl From<MatchType> for FBMatchType {
    fn from(value: MatchType) -> Self {
        match value {
            MatchType::Contains => FBMatchType::Contains,
            MatchType::Equals => FBMatchType::Equal,
        }
    }
}
