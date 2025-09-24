use crate::gen_flatbuffers::dexkit::schema::RetentionPolicyType as FBRetentionPolicyType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionPolicyType {
    Source,
    Class,
    Runtime,
}

impl From<RetentionPolicyType> for FBRetentionPolicyType {
    fn from(value: RetentionPolicyType) -> Self {
        match value {
            RetentionPolicyType::Source => Self::Source,
            RetentionPolicyType::Class => Self::Class,
            RetentionPolicyType::Runtime => Self::Runtime,
        }
    }
}
