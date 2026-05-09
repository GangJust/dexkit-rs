use crate::BridgeCore;
use crate::gen_flatbuffers::dexkit::fb::FBAnnotationElementMeta;
use crate::{DexkitBridge, result::AnnotationEncodeValue};
use std::fmt::Debug;

#[derive(Clone)]
pub struct AnnotationElementData {
    bridge: BridgeCore,
    name: String,
    value: AnnotationEncodeValue,
}

impl Debug for AnnotationElementData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnotationElementData")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}

impl AnnotationElementData {
    /// get value
    pub fn value(&self) -> &AnnotationEncodeValue {
        &self.value
    }

    /// ...
    pub(crate) fn with_meta(
        bridge: &DexkitBridge,
        meta: FBAnnotationElementMeta<'_>,
    ) -> AnnotationElementData {
        let name = meta.name().unwrap_or("").to_string();
        let value = match meta.value() {
            None => AnnotationEncodeValue::default(),
            Some(encode_value) => AnnotationEncodeValue::with_meta(bridge, encode_value),
        };

        Self {
            bridge: bridge.core_clone(),
            name,
            value,
        }
    }
}
