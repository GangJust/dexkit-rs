use crate::gen_flatbuffers::dexkit::schema::OpCodeMatchType as FBOpCodeMatchType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCodeMatchType {
    Contains,
    StartsWith,
    EndsWith,
    Equals,
}

impl From<OpCodeMatchType> for FBOpCodeMatchType {
    fn from(value: OpCodeMatchType) -> Self {
        match value {
            OpCodeMatchType::Contains => Self::Contains,
            OpCodeMatchType::StartsWith => Self::StartWith,
            OpCodeMatchType::EndsWith => Self::EndWith,
            OpCodeMatchType::Equals => Self::Equal,
        }
    }
}
