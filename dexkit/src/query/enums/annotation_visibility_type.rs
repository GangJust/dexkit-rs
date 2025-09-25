use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationVisibilityType as FBSchemaAnnotationVisibilityType,
    AnnotationVisibilityType as FBAnnotationVisibilityType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationVisibilityType {
    Build,
    Runtime,
    System,
}

impl From<FBAnnotationVisibilityType> for AnnotationVisibilityType {
    fn from(value: FBAnnotationVisibilityType) -> Self {
        match value {
            FBAnnotationVisibilityType::Build => Self::Build,
            FBAnnotationVisibilityType::Runtime => Self::Runtime,
            FBAnnotationVisibilityType::System => Self::System,
            _ => Self::Build,
        }
    }
}

impl From<AnnotationVisibilityType> for FBSchemaAnnotationVisibilityType {
    fn from(value: AnnotationVisibilityType) -> Self {
        match value {
            AnnotationVisibilityType::Build => Self::Build,
            AnnotationVisibilityType::Runtime => Self::Runtime,
            AnnotationVisibilityType::System => Self::System,
        }
    }
}
