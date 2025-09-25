use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationMeta as FBAnnotationMeta, AnnotationMetaArrayHolder as FBAnnotationMetaArrayHolder,
    AnnotationVisibilityType as FBAnnotationVisibilityType,
};
use crate::result::AnnotationElementData;
use crate::{DexkitBridge, query::enums::AnnotationVisibilityType, result::base::BaseData};

#[derive(Debug, Clone)]
pub struct AnnotationData<'a> {
    bridge: &'a DexkitBridge,
    dex_id: u32,
    type_id: u32,
    type_descriptor: String,
    visibility: Option<AnnotationVisibilityType>,
    elements: Vec<AnnotationElementData<'a>>,
}

impl<'a> BaseData for AnnotationData<'a> {
    fn get_bradge(&self) -> &DexkitBridge {
        self.bridge
    }

    fn get_dex_id(&self) -> u32 {
        self.dex_id
    }

    fn get_id(&self) -> u32 {
        0
    }
}

impl<'a> AnnotationData<'a> {
    /// ...
    pub(crate) fn with_meta(
        bridge: &'a DexkitBridge,
        meta: &FBAnnotationMeta,
    ) -> AnnotationData<'a> {
        let dex_id = meta.dex_id();
        let type_id = meta.type_id();
        let type_descriptor = meta.type_descriptor().unwrap_or("").to_string();
        let visibility = if meta.visibility() == FBAnnotationVisibilityType::None {
            None
        } else {
            Some(AnnotationVisibilityType::from(meta.visibility()))
        };
        let elements = if let Some(fb_elements) = meta.elements() {
            let mut list = Vec::new();
            for fb_element in fb_elements.iter() {
                list.push(AnnotationElementData::with_meta(bridge, &fb_element));
            }
            list
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
        }
    }

    /// ...
    pub(crate) fn with_data_raw(bridge: &'a DexkitBridge, data: &[u8]) -> Vec<AnnotationData<'a>> {
        let annotation_meta_list = unsafe {
            flatbuffers::root_unchecked::<FBAnnotationMetaArrayHolder>(data)
        };
        println!("[Rust] AnnotationMetaArrayHolder: {:#?}", annotation_meta_list);
        // error: 解析 @kotlin.Metadata 注解时会报错，\u0000\u0001... 等等是无效的 utf8 字符, 待解决.

        // let mut list = Vec::new();
        // if let Some(array) = annotation_meta_list.annotations().iter().next() {
        //     for meta in array {
        //         list.push(AnnotationData::with_meta(bridge, &meta));
        //     }
        // }
        // list

        Vec::new()
    }
}
