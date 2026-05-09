use crate::BridgeCore;
use crate::gen_flatbuffers::dexkit::fb::FBAnnotationEncodeArray;
use crate::{DexkitBridge, result::AnnotationEncodeValue};
use std::fmt::Debug;

#[derive(Clone)]
pub struct AnnotationEncodeArrayData {
    bridge: BridgeCore,
    values: Vec<AnnotationEncodeValue>,
}

impl Debug for AnnotationEncodeArrayData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnotationEncodeArrayData")
            .field("values", &self.values)
            .finish()
    }
}

impl AnnotationEncodeArrayData {
    /// get values
    pub fn values(&self) -> &[AnnotationEncodeValue] {
        &self.values
    }

    pub(crate) fn with_meta(bridge: &DexkitBridge, meta: FBAnnotationEncodeArray<'_>) -> Self {
        let values = match meta.values() {
            None => Vec::new(),
            Some(vs) => vs
                .iter()
                .map(|encode_value| AnnotationEncodeValue::with_meta(bridge, encode_value))
                .collect(),
        };

        Self {
            bridge: bridge.core_clone(),
            values,
        }
    }
}
