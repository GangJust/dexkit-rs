use crate::gen_flatbuffers::dexkit::schema::StringMatchType as FBStringMatchType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringMatchType {
    Contains,
    StartWith,
    EndWith,
    SimilarRegex,
    Equals,
}

impl Default for StringMatchType {
    fn default() -> Self {
        StringMatchType::Contains
    }
}

impl From<StringMatchType> for FBStringMatchType {
    fn from(value: StringMatchType) -> Self {
        match value {
            StringMatchType::Contains => Self::Contains,
            StringMatchType::StartWith => Self::StartWith,
            StringMatchType::EndWith => Self::EndWith,
            StringMatchType::SimilarRegex => Self::SimilarRegex,
            StringMatchType::Equals => Self::Equal,
        }
    }
}
