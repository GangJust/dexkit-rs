use crate::gen_flatbuffers::dexkit::schema::AnnotationVisibilityType as FBSchemaAnnotationVisibilityType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationVisibilityType {
    Build,
    Runtime,
    System,
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