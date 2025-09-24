use crate::gen_flatbuffers::dexkit::schema::ClassMeta as FBClassMeta;
use crate::{DexkitBridge, result::base::BaseData};

#[allow(unused)]
#[derive(Debug)]
pub struct ClassData<'a> {
    dexkit_bridge: &'a DexkitBridge,
    id: u32,
    dex_id: u32,
    source_file: String,
    modifiers: u32,
    descriptor: String,
    super_class_id: Option<u32>,
    interface_ids: Vec<i32>,
    method_ids: Vec<i32>,
    field_ids: Vec<i32>,
}

impl<'a> BaseData for ClassData<'a> {
    fn get_dex_id(&self) -> u32 {
        self.dex_id
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl<'a> ClassData<'a> {
    /// modifiers bitmask, see `Modifier`
    pub fn modifiers(&self) -> u32 {
        self.modifiers
    }

    /// class descriptor, e.g. "Lcom/example/MyClass;"
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    /// ...
    pub(crate) fn with_meta(dexkit_bridge: &'a DexkitBridge, meta: FBClassMeta<'a>) -> Self {
        let id = meta.id();
        let dex_id = meta.dex_id();
        let source_file = meta.source_file().unwrap_or_default().to_string();
        let modifiers = meta.access_flags();
        let descriptor = meta.dex_descriptor().unwrap_or_default().to_string();
        let super_class_id = if meta.super_class() == 0 {
            None
        } else {
            Some(meta.super_class())
        };
        let interface_ids = meta
            .interfaces()
            .map_or(vec![], |ints| ints.iter().collect());
        let method_ids = meta.methods().map_or(vec![], |mths| mths.iter().collect());
        let field_ids = meta.fields().map_or(vec![], |flds| flds.iter().collect());

        Self {
            dexkit_bridge,
            id,
            dex_id,
            source_file,
            modifiers,
            descriptor,
            super_class_id,
            interface_ids,
            method_ids,
            field_ids,
        }
    }
}
