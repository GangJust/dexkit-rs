use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationElementMeta as FBAnnotationElementMeta,
    AnnotationEncodeValueMeta as FBAnnotationEncodeValueMeta,
};
use crate::{DexkitBridge, result::AnnotationEncodeValue};

#[derive(Debug, Clone)]
pub struct AnnotationElementData<'a> {
    bridge: &'a DexkitBridge,
    name: String,
    value: AnnotationEncodeValue<'a>,
}

impl<'a> AnnotationElementData<'a> {
    /// get value
    pub fn value(&self) -> &AnnotationEncodeValue<'a> {
        &self.value
    }

    /// ...
    pub(crate) fn with_meta(
        bridge: &'a DexkitBridge,
        meta: FBAnnotationElementMeta<'a>,
    ) -> AnnotationElementData<'a> {
        let name = meta.name().unwrap_or("").to_string();
        let value = match meta.value() {
            None => AnnotationEncodeValue::default(),
            Some(encode_value) => AnnotationEncodeValue::with_meta(bridge, encode_value),
        };

        Self {
            bridge,
            name,
            value,
        }
    }
}
