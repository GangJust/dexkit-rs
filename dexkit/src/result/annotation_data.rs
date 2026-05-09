use crate::BridgeCore;
use crate::fb_codec::parse_fb_root_unchecked;
use crate::gen_flatbuffers::dexkit::fb::{
    FBAnnotationMeta, FBAnnotationMetaArrayHolder, FBAnnotationVisibilityType,
    FBParametersAnnotationMetaArrayHoler,
};
use crate::result::AnnotationElementData;
use crate::wrap::DexClass;
use crate::{DexkitBridge, query::enums::AnnotationVisibilityType, result::base::BaseData};
use std::cell::OnceCell;
use std::fmt::Debug;

#[derive(Clone)]
pub struct AnnotationData {
    bridge: BridgeCore,
    dex_id: u32,
    type_id: u32,
    type_descriptor: String,
    visibility: Option<AnnotationVisibilityType>,
    elements: Vec<AnnotationElementData>,
    // Lazy loaded fields
    dex_class: OnceCell<Option<DexClass>>,
}

impl Debug for AnnotationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnotationData")
            .field("dex_id", &self.dex_id)
            .field("type_id", &self.type_id)
            .field("type_descriptor", &self.type_descriptor)
            .field("visibility", &self.visibility)
            .field("elements", &self.elements)
            .finish()
    }
}

impl BaseData for AnnotationData {
    fn bridge(&self) -> &BridgeCore {
        &self.bridge
    }

    fn dex_id(&self) -> u32 {
        self.dex_id
    }

    fn id(&self) -> u32 {
        0
    }
}

impl AnnotationData {
    /// get visibility
    pub fn visibility(&self) -> Option<AnnotationVisibilityType> {
        self.visibility
    }

    /// get elements
    pub fn elements(&self) -> &[AnnotationElementData] {
        &self.elements
    }

    /// get type descriptor
    pub fn type_name(&self) -> Option<&str> {
        self.get_dex_class().map(|dex_class| dex_class.type_name())
    }

    /// internal use, get dex class
    pub(crate) fn get_dex_class(&self) -> Option<&DexClass> {
        self.dex_class
            .get_or_init(|| DexClass::deserialize(&self.type_descriptor))
            .as_ref()
    }

    /// ...
    pub(crate) fn with_meta(bridge: &DexkitBridge, meta: FBAnnotationMeta<'_>) -> AnnotationData {
        let dex_id = meta.dex_id();
        let type_id = meta.type_id();
        let type_descriptor = meta.type_descriptor().unwrap_or("").to_string();
        let visibility = if meta.visibility() == FBAnnotationVisibilityType::None {
            None
        } else {
            Some(AnnotationVisibilityType::from(meta.visibility()))
        };
        let elements = if let Some(elements) = meta.elements() {
            elements
                .iter()
                .map(|element| AnnotationElementData::with_meta(bridge, element))
                .collect()
        } else {
            Vec::new()
        };

        Self {
            bridge: bridge.core_clone(),
            dex_id,
            type_id,
            type_descriptor,
            visibility,
            elements,
            // Lazy loaded fields
            dex_class: OnceCell::new(),
        }
    }

    /// ...
    pub(crate) fn with_annotation_meta_array_raw(
        bridge: &DexkitBridge,
        data: &[u8],
    ) -> Vec<AnnotationData> {
        let annotation_meta_array_holder =
            unsafe { parse_fb_root_unchecked::<FBAnnotationMetaArrayHolder<'_>>(data) }; // not verify data

        Self::with_annotation_meta_array(bridge, annotation_meta_array_holder)
    }

    /// ...
    pub(crate) fn with_parameters_annotation_meta_array_raw(
        bridge: &DexkitBridge,
        data: &[u8],
    ) -> Vec<Vec<AnnotationData>> {
        let parameters_annotation_meta_array_holer =
            unsafe { parse_fb_root_unchecked::<FBParametersAnnotationMetaArrayHoler<'_>>(data) }; // not verify data

        parameters_annotation_meta_array_holer
            .annotations_array()
            .iter()
            .map(|array| {
                array
                    .iter()
                    .flat_map(|annotation_meta| {
                        Self::with_annotation_meta_array(bridge, annotation_meta)
                    })
                    .collect()
            })
            .collect()
    }

    /// ...
    pub(crate) fn with_annotation_meta_array(
        bridge: &DexkitBridge,
        array: FBAnnotationMetaArrayHolder<'_>,
    ) -> Vec<AnnotationData> {
        array.annotations().iter().next().map_or(vec![], |array| {
            array
                .iter()
                .map(|annotation_meta| Self::with_meta(bridge, annotation_meta))
                .collect()
        })
    }
}
