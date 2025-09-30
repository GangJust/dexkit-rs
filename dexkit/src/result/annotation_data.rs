use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationMeta as FBAnnotationMeta, AnnotationMetaArrayHolder as FBAnnotationMetaArrayHolder,
    AnnotationVisibilityType as FBAnnotationVisibilityType,
    ParametersAnnotationMetaArrayHoler as FBParametersAnnotationMetaArrayHolder,
};
use crate::result::AnnotationElementData;
use crate::wrap::DexClass;
use crate::{DexkitBridge, query::enums::AnnotationVisibilityType, result::base::BaseData};
use std::cell::OnceCell;
use std::fmt::Debug;

#[derive(Clone)]
pub struct AnnotationData<'a> {
    bridge: &'a DexkitBridge,
    dex_id: u32,
    type_id: u32,
    type_descriptor: String,
    visibility: Option<AnnotationVisibilityType>,
    elements: Vec<AnnotationElementData<'a>>,
    // Lazy loaded fields
    dex_class: OnceCell<Option<DexClass>>,
}

impl<'a> Debug for AnnotationData<'a> {
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

impl<'a> BaseData for AnnotationData<'a> {
    fn bridge(&self) -> &DexkitBridge {
        self.bridge
    }

    fn dex_id(&self) -> u32 {
        self.dex_id
    }

    fn id(&self) -> u32 {
        0
    }
}

impl<'a> AnnotationData<'a> {
    /// get visibility
    pub fn visibility(&self) -> Option<AnnotationVisibilityType> {
        self.visibility
    }

    /// get elements
    pub fn elements(&self) -> &Vec<AnnotationElementData<'a>> {
        &self.elements
    }

    /// get type descriptor
    pub fn type_name(&self) -> Option<String> {
        self.get_dex_class().map(|dex_class| dex_class.type_name())
    }

    /// internal use, get dex class
    pub(crate) fn get_dex_class(&self) -> Option<&DexClass> {
        self.dex_class
            .get_or_init(|| DexClass::deserialize(&self.type_descriptor))
            .as_ref()
    }

    /// ...
    pub(crate) fn with_meta(
        bridge: &'a DexkitBridge,
        meta: FBAnnotationMeta<'a>,
    ) -> AnnotationData<'a> {
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
            bridge,
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
        bridge: &'a DexkitBridge,
        data: &'a [u8],
    ) -> Vec<AnnotationData<'a>> {
        let annotation_meta_array_holder =
            unsafe { flatbuffers::root_unchecked::<FBAnnotationMetaArrayHolder>(data) }; // not verify data

        Self::with_annotation_meta_array(bridge, annotation_meta_array_holder)
    }

    /// ...
    pub(crate) fn with_parameters_annotation_meta_array_raw(
        bridge: &'a DexkitBridge,
        data: &'a [u8],
    ) -> Vec<Vec<AnnotationData<'a>>> {
        let parameters_annotation_meta_array_holer =
            unsafe { flatbuffers::root_unchecked::<FBParametersAnnotationMetaArrayHolder>(data) }; // not verify data

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
        bridge: &'a DexkitBridge,
        array: FBAnnotationMetaArrayHolder<'a>,
    ) -> Vec<AnnotationData<'a>> {
        array.annotations().iter().next().map_or(vec![], |array| {
            array
                .iter()
                .map(|annotation_meta| Self::with_meta(bridge, annotation_meta))
                .collect()
        })
    }
}
