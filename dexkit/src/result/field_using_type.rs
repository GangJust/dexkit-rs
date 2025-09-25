use crate::gen_flatbuffers::dexkit::schema::UsingType as FBUsingType;

#[derive(Debug, Clone)]
pub enum FieldUsingType {
    /// Read field
    Read,

    /// Write field
    Write,
}

impl From<FBUsingType> for FieldUsingType {
    fn from(value: FBUsingType) -> Self {
        match value {
            FBUsingType::Get => FieldUsingType::Read,
            FBUsingType::Put => FieldUsingType::Write,
            _ => panic!("Unknown using type: {:?}", value), // Should not happen
        }
    }
}

impl FieldUsingType {
    /// Check if the field usage type is read
    pub fn is_read(&self) -> bool {
        matches!(self, FieldUsingType::Read)
    }

    /// Check if the field usage type is write
    pub fn is_write(&self) -> bool {
        matches!(self, FieldUsingType::Write)
    }
}
