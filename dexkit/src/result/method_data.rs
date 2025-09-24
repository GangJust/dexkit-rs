use crate::gen_flatbuffers::dexkit::schema::MethodMeta as FBMethodMeta;
use crate::{dexkit_bridge::DexkitBridge, result::base::BaseData};

#[allow(unused)]
#[derive(Debug)]
pub struct MethodData<'a> {
    dexkit_bridge: &'a DexkitBridge,
    id: u32,
    dex_id: u32,
    class_id: u32,
    modifiers: u32,
    descriptor: String,
    return_type_id: u32,
    param_type_ids: Vec<i32>,
}

impl<'a> BaseData for MethodData<'a> {
    fn get_dex_id(&self) -> u32 {
        self.dex_id
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl<'a> MethodData<'a> {
    /// modifiers bitmask, see `Modifier`
    pub fn modifiers(&self) -> u32 {
        self.modifiers
    }

    /// method descriptor. e.g. `Ljava/lang/String;->charAt(I)C`
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    /// ...
    pub(crate) fn with_meta(dexkit_bridge: &'a DexkitBridge, meta: FBMethodMeta<'a>) -> Self {
        let id = meta.id();
        let dex_id = meta.dex_id();
        let class_id = meta.class_id();
        let modifiers = meta.access_flags();
        let descriptor = meta.dex_descriptor().unwrap_or_default().to_string();
        let return_type_id = meta.return_type();
        let param_type_ids = meta
            .parameter_types()
            .map_or(vec![], |params| params.iter().collect());

        Self {
            dexkit_bridge,
            id,
            dex_id,
            class_id,
            modifiers,
            descriptor,
            return_type_id,
            param_type_ids,
        }
    }
}
