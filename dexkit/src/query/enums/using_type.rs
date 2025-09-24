use crate::gen_flatbuffers::dexkit::schema::UsingType as FBUsingType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsingType {
    Any,
    Read,
    Write,
}

impl Default for UsingType {
    fn default() -> Self {
        Self::Any
    }
}

impl From<UsingType> for FBUsingType {
    fn from(value: UsingType) -> Self {
        match value {
            UsingType::Any => Self::Any,
            UsingType::Read => Self::Get,
            UsingType::Write => Self::Put,
        }
    }
}
