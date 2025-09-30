use crate::gen_flatbuffers::dexkit::schema::FieldMeta as FBFieldMeta;
use crate::result::{AnnotationData, ClassData, MethodDataList};
use crate::wrap::DexField;
use crate::{DexkitBridge, result::base::BaseData};
use std::cell::OnceCell;
use std::fmt::Debug;

#[allow(unused)]
#[derive(Clone)]
pub struct FieldData<'a> {
    bridge: &'a DexkitBridge,
    id: u32,
    dex_id: u32,
    class_id: u32,
    modifiers: u32,
    descriptor: String,
    type_id: u32,
    // Lazy loaded fields
    dex_field: OnceCell<Option<DexField>>,
    declared_class: OnceCell<Option<Box<ClassData<'a>>>>,
    type_class: OnceCell<Option<Box<ClassData<'a>>>>,
    annotations: OnceCell<Vec<AnnotationData<'a>>>,
    readers: OnceCell<MethodDataList<'a>>,
    writers: OnceCell<MethodDataList<'a>>,
}

impl<'a> Debug for FieldData<'a> {
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

impl<'a> BaseData for FieldData<'a> {
    fn bridge(&self) -> &DexkitBridge {
        self.bridge
    }

    fn dex_id(&self) -> u32 {
        self.dex_id
    }

    fn id(&self) -> u32 {
        self.id
    }
}

impl<'a> FieldData<'a> {
    /// modifiers bitmask, see `Modifier`
    pub fn modifiers(&self) -> u32 {
        self.modifiers
    }

    /// field descriptor, e.g. "Lcom/example/MyClass;->myField:I"
    pub fn descriptor(&self) -> String {
        self.descriptor.to_string()
    }

    /// field type signature, e.g. "I"
    pub fn type_signature(&self) -> Option<String> {
        self.get_dex_field()
            .map(|f| f.type_signature().unwrap_or_default())
    }

    /// field class name, e.g. "com.example.MyClass"
    pub fn class_name(&self) -> Option<String> {
        self.get_dex_field().map(|f| f.class_name())
    }

    /// field name, e.g. "com.example.MyClass"
    pub fn declared_class_name(&self) -> Option<String> {
        self.class_name()
    }

    /// field name, e.g. "myField"
    pub fn field_name(&self) -> Option<String> {
        self.get_dex_field().map(|f| f.field_name())
    }

    /// field name, e.g. "myField"
    pub fn name(&self) -> Option<String> {
        self.field_name()
    }

    /// field type name, e.g. "int"
    pub fn type_name(&self) -> Option<String> {
        self.get_dex_field().map(|f| f.type_name())
    }

    /// get the class where the field is declared
    pub fn declared_class(&self) -> Option<ClassData<'a>> {
        let cls = self.declared_class.get_or_init(|| {
            let encode_id = Self::get_encode_id(self.dex_id, self.class_id);
            self.bridge
                .get_type_by_ids(&vec![encode_id])
                .get(0)
                .cloned()
                .map(Box::new)
        });
        cls.as_deref().cloned()
    }

    /// get the class of the field type
    pub fn type_class(&self) -> Option<ClassData<'a>> {
        let cls = self.type_class.get_or_init(|| {
            let encode_id = Self::get_encode_id(self.dex_id, self.type_id);
            self.bridge
                .get_type_by_ids(&vec![encode_id])
                .get(0)
                .cloned()
                .map(Box::new)
        });
        cls.as_deref().cloned()
    }

    /// get annotations of this class
    pub fn annotations(&self) -> Vec<AnnotationData<'a>> {
        self.annotations
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.get_field_annotations(encode_id)
            })
            .clone()
    }

    /// using smali `iput-*`、`sput-*` instructions to read this field's methods
    pub fn readers(&self) -> MethodDataList<'a> {
        self.readers
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.read_field_methods(encode_id)
            })
            .clone()
    }

    /// using smali `iget-*`、`sget-*` instructions to write this field's methods
    pub fn writers(&self) -> MethodDataList<'a> {
        self.writers
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.write_field_methods(encode_id)
            })
            .clone()
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
    pub(crate) fn with_meta(bridge: &'a DexkitBridge, meta: FBFieldMeta<'a>) -> Self {
        let id = meta.id();
        let dex_id = meta.dex_id();
        let class_id = meta.class_id();
        let modifiers = meta.access_flags();
        let descriptor = meta.dex_descriptor().unwrap_or_default().to_string();
        let type_id = meta.type_id();

        Self {
            bridge,
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
    pub(crate) fn with_meta_raw(bridge: &'a DexkitBridge, data: &'a [u8]) -> Option<Self> {
        flatbuffers::root::<FBFieldMeta<'_>>(data)
            .map(|meta| Self::with_meta(bridge, meta))
            .ok()
    }
}
