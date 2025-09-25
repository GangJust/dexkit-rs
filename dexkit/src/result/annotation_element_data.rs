use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationElementMeta as FBAnnotationElementMeta,
    AnnotationEncodeValueMeta as FBAnnotationEncodeValueMeta,
};
use crate::{DexkitBridge, result::AnnotationEncodeValue};

#[derive(Debug, Clone)]
pub struct AnnotationElementData<'a> {
    bridge: &'a DexkitBridge,
    name: String,
    value: AnnotationEncodeValue,
}

impl<'a> AnnotationElementData<'a> {
    pub(crate) fn with_meta(
        bridge: &'a DexkitBridge,
        meta: &FBAnnotationElementMeta,
    ) -> AnnotationElementData<'a> {
        let name = meta.name().unwrap_or("").to_string();
        let value = AnnotationEncodeValue::with_meta(&meta.value().unwrap());

        Self {
            bridge,
            name,
            value,
        }
    }
}
