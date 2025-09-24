use crate::gen_flatbuffers::dexkit::schema::FieldMeta as FBFieldMeta;
use crate::{DexkitBridge, result::base::BaseData};

#[allow(unused)]
#[derive(Debug)]
pub struct FieldData<'a> {
    dexkit_bridge: &'a DexkitBridge,
    id: u32,
    dex_id: u32,
    class_id: u32,
    modifiers: u32,
    descriptor: String,
    type_id: u32,
}

impl<'a> BaseData for FieldData<'a> {
    fn get_dex_id(&self) -> u32 {
        self.dex_id
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl<'a> FieldData<'a> {

    /// modifiers bitmask, see `Modifier`
    pub fn modifiers(&self) -> u32 {
        self.modifiers
    }

    /// field descriptor, e.g. "Lcom/example/MyClass;->myField:I"
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    /// ...
    pub(crate) fn with_meta(dexkit_bridge: &'a DexkitBridge, meta: FBFieldMeta<'a>) -> Self {
        let id = meta.id();
        let dex_id = meta.dex_id();
        let class_id = meta.class_id();
        let modifiers = meta.access_flags();
        let descriptor = meta.dex_descriptor().unwrap_or_default().to_string();
        let type_id = meta.type_id();

        Self {
            dexkit_bridge,
            id,
            dex_id,
            class_id,
            modifiers,
            descriptor,
            type_id,
        }
    }
}
