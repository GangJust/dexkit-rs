use crate::gen_flatbuffers::dexkit::schema::AnnotationEncodeArray as FBAnnotationEncodeArray;
use crate::{DexkitBridge, result::AnnotationEncodeValue};
use std::fmt::Debug;

#[derive(Clone)]
pub struct AnnotationEncodeArrayData<'a> {
    bridge: &'a DexkitBridge,
    values: Vec<AnnotationEncodeValue<'a>>,
}

impl<'a> Debug for AnnotationEncodeArrayData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnotationEncodeArrayData")
            .field("values", &self.values)
            .finish()
    }
}

impl<'a> AnnotationEncodeArrayData<'a> {
    pub(crate) fn with_meta(
        bridge: &'a DexkitBridge,
        meta: FBAnnotationEncodeArray<'a>,
    ) -> AnnotationEncodeArrayData<'a> {
        let values = match meta.values() {
            None => Vec::new(),
            Some(vs) => vs
                .iter()
                .map(|encode_value| AnnotationEncodeValue::with_meta(bridge, encode_value))
                .collect(),
        };

        Self { bridge, values }
    }
}
