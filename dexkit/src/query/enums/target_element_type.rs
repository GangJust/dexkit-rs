use crate::gen_flatbuffers::dexkit::schema::TargetElementType as FBTargetElementType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetElementType {
    Type,
    Field,
    Method,
    Parameter,
    Constructor,
    LocalVariable,
    AnnotationType,
    Package,
    TypeParameter,
    TypeUse,
    //TODO add MODULE
}

impl From<TargetElementType> for FBTargetElementType {
    fn from(value: TargetElementType) -> Self {
        match value {
            TargetElementType::Type => Self::Type,
            TargetElementType::Field => Self::Field,
            TargetElementType::Method => Self::Method,
            TargetElementType::Parameter => Self::Parameter,
            TargetElementType::Constructor => Self::Constructor,
            TargetElementType::LocalVariable => Self::LocalVariable,
            TargetElementType::AnnotationType => Self::AnnotationType,
            TargetElementType::Package => Self::Package,
            TargetElementType::TypeParameter => Self::TypeParameter,
            TargetElementType::TypeUse => Self::TypeUse,
        }
    }
}
