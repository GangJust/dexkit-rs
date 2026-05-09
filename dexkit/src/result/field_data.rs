use crate::BridgeCore;
use crate::fb_codec::parse_fb_root;
use crate::gen_flatbuffers::dexkit::fb::FBFieldMeta;
use crate::result::{AnnotationData, ClassData, MethodDataList};
use crate::wrap::DexField;
use crate::{DexkitBridge, result::base::BaseData};
use std::cell::OnceCell;
use std::fmt::Debug;

#[allow(unused)]
#[derive(Clone)]
pub struct FieldData {
    bridge: BridgeCore,
    id: u32,
    dex_id: u32,
    class_id: u32,
    modifiers: u32,
    descriptor: String,
    type_id: u32,
    // Lazy loaded fields
    dex_field: OnceCell<Option<DexField>>,
    declared_class: OnceCell<Option<Box<ClassData>>>,
    type_class: OnceCell<Option<Box<ClassData>>>,
    annotations: OnceCell<Vec<AnnotationData>>,
    readers: OnceCell<MethodDataList>,
    writers: OnceCell<MethodDataList>,
}

impl Debug for FieldData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldData")
            .field("id", &self.id)
            .field("dex_id", &self.dex_id)
            .field("class_id", &self.class_id)
            .field("modifiers", &self.modifiers)
            .field("descriptor", &self.descriptor)
            .field("type_id", &self.type_id)
            .finish()
    }
}

impl BaseData for FieldData {
    fn bridge(&self) -> &BridgeCore {
        &self.bridge
    }

    fn dex_id(&self) -> u32 {
        self.dex_id
    }

    fn id(&self) -> u32 {
        self.id
    }
}

impl FieldData {
    /// modifiers bitmask, see `Modifier`
    pub fn modifiers(&self) -> u32 {
        self.modifiers
    }

    /// field descriptor, e.g. "Lcom/example/MyClass;->myField:I"
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    /// field type signature, e.g. "I"
    pub fn type_signature(&self) -> Option<&str> {
        self.get_dex_field()
            .map(|f| f.type_signature().unwrap_or_default())
    }

    /// field class name, e.g. "com.example.MyClass"
    pub fn class_name(&self) -> Option<&str> {
        self.get_dex_field().map(|f| f.class_name())
    }

    /// field name, e.g. "com.example.MyClass"
    pub fn declared_class_name(&self) -> Option<&str> {
        self.class_name()
    }

    /// field name, e.g. "myField"
    pub fn field_name(&self) -> Option<&str> {
        self.get_dex_field().map(|f| f.field_name())
    }

    /// field name, e.g. "myField"
    pub fn name(&self) -> Option<&str> {
        self.field_name()
    }

    /// field type name, e.g. "int"
    pub fn type_name(&self) -> Option<&str> {
        self.get_dex_field().map(|f| f.type_name())
    }

    /// get the class where the field is declared
    pub fn declared_class(&self) -> Option<&ClassData> {
        let cls = self.declared_class.get_or_init(|| {
            let encode_id = Self::get_encode_id(self.dex_id, self.class_id);
            self.bridge
                .get_type_by_ids(&vec![encode_id])
                .get(0)
                .cloned()
                .map(Box::new)
        });
        cls.as_deref()
    }

    /// get the class of the field type
    pub fn type_class(&self) -> Option<&ClassData> {
        let cls = self.type_class.get_or_init(|| {
            let encode_id = Self::get_encode_id(self.dex_id, self.type_id);
            self.bridge
                .get_type_by_ids(&vec![encode_id])
                .get(0)
                .cloned()
                .map(Box::new)
        });
        cls.as_deref()
    }

    /// get annotations of this class
    pub fn annotations(&self) -> &[AnnotationData] {
        self.annotations
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.get_field_annotations(encode_id)
            })
            .as_slice()
    }

    /// using smali `iput-*`、`sput-*` instructions to read this field's methods
    pub fn readers(&self) -> &MethodDataList {
        self.readers.get_or_init(|| {
            let encode_id = Self::get_encode_id(self.dex_id, self.id);
            self.bridge.read_field_methods(encode_id)
        })
    }

    /// using smali `iget-*`、`sget-*` instructions to write this field's methods
    pub fn writers(&self) -> &MethodDataList {
        self.writers.get_or_init(|| {
            let encode_id = Self::get_encode_id(self.dex_id, self.id);
            self.bridge.write_field_methods(encode_id)
        })
    }

    /// get the wrapped DexField
    pub fn to_dex_field(&self) -> Option<DexField> {
        self.get_dex_field().cloned()
    }

    /// internal use, get the wrapped DexField
    pub(crate) fn get_dex_field(&self) -> Option<&DexField> {
        self.dex_field
            .get_or_init(|| DexField::deserialize(&self.descriptor))
            .as_ref()
    }

    /// ...
    pub(crate) fn with_meta(bridge: &DexkitBridge, meta: FBFieldMeta<'_>) -> Self {
        let id = meta.id();
        let dex_id = meta.dex_id();
        let class_id = meta.class_id();
        let modifiers = meta.access_flags();
        let descriptor = meta.dex_descriptor().unwrap_or_default().to_string();
        let type_id = meta.type_id();

        Self {
            bridge: bridge.core_clone(),
            id,
            dex_id,
            class_id,
            modifiers,
            descriptor,
            type_id,
            dex_field: OnceCell::new(),
            declared_class: OnceCell::new(),
            type_class: OnceCell::new(),
            annotations: OnceCell::new(),
            readers: OnceCell::new(),
            writers: OnceCell::new(),
        }
    }

    /// ...
    pub(crate) fn with_meta_raw(bridge: &DexkitBridge, data: &[u8]) -> Option<Self> {
        parse_fb_root::<FBFieldMeta<'_>>(data)
            .map(|meta| Self::with_meta(bridge, meta))
            .ok()
    }
}
