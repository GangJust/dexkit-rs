use std::cell::OnceCell;
use std::fmt::Debug;
use std::sync::LazyLock;

use crate::gen_flatbuffers::dexkit::schema::ClassMeta as FBClassMeta;
use crate::result::{
    AnnotationData, ClassDataList, FieldData, FieldDataList, MethodData, MethodDataList,
};
use crate::wrap::DexClass;
use crate::{DexkitBridge, result::base::BaseData};

#[allow(unused)]
#[derive(Clone)]
pub struct ClassData<'a> {
    bridge: &'a DexkitBridge,
    id: u32,
    dex_id: u32,
    source_file: String,
    modifiers: u32,
    descriptor: String,
    super_class_id: Option<u32>,
    interface_ids: Vec<i32>,
    method_ids: Vec<i32>,
    field_ids: Vec<i32>,
    // Lazy loaded fields
    dex_class: OnceCell<Option<DexClass>>,
    super_class: OnceCell<Option<Box<ClassData<'a>>>>,
    interfaces: OnceCell<ClassDataList<'a>>,
    methods: OnceCell<MethodDataList<'a>>,
    fields: OnceCell<FieldDataList<'a>>,
    annotations: OnceCell<Vec<AnnotationData<'a>>>,
}

impl Debug for ClassData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassData")
            .field("id", &self.id)
            .field("dex_id", &self.dex_id)
            .field("source_file", &self.source_file)
            .field("modifiers", &self.modifiers)
            .field("descriptor", &self.descriptor)
            .field("super_class_id", &self.super_class_id)
            .field("interface_ids", &self.interface_ids)
            .field("method_ids", &self.method_ids)
            .field("field_ids", &self.field_ids)
            .finish()
    }
}

impl<'a> BaseData for ClassData<'a> {
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

impl<'a> ClassData<'a> {
    /// source file name, e.g. "MyClass.java"
    pub fn source_file(&self) -> String {
        self.source_file.to_string()
    }

    /// modifiers bitmask, see `Modifier`
    pub fn modifiers(&self) -> u32 {
        self.modifiers
    }

    /// class descriptor, e.g. "Lcom/example/MyClass;"
    pub fn descriptor(&self) -> String {
        self.descriptor.to_string()
    }

    /// class name, e.g. "com.example.MyClass"
    pub fn name(&self) -> Option<String> {
        self.get_dex_class().map(|dc| dc.type_name())
    }

    /// simple class name, e.g. "MyClass"
    pub fn simple_name(&self) -> Option<String> {
        self.get_dex_class().map(|dc| dc.simple_name())
    }

    /// is array type, e.g. "com.example.MyClass[]" -> true
    pub fn is_array(&self) -> bool {
        self.get_dex_class().map_or(false, |dc| dc.is_array())
    }

    /// get super class, None if no super class
    pub fn supper_class(&self) -> Option<ClassData<'a>> {
        let cls = self.super_class.get_or_init(|| {
            self.super_class_id.and_then(|id| {
                let encode_id = Self::get_encode_id(self.dex_id, id);
                self.bridge
                    .get_type_by_ids(&vec![encode_id])
                    .get(0)
                    .cloned()
                    .map(Box::new)
            })
        });
        cls.as_deref().cloned()
    }

    /// get implemented interfaces of this class
    pub fn interfaces(&self) -> ClassDataList<'a> {
        self.interfaces
            .get_or_init(|| {
                let encode_ids: Vec<i64> = self
                    .interface_ids
                    .iter()
                    .map(|&id| Self::get_encode_id(self.dex_id, id as u32))
                    .collect();
                self.bridge.get_type_by_ids(&encode_ids)
            })
            .clone()
    }

    /// get declared interfaces count
    pub fn interface_count(&self) -> usize {
        self.interface_ids.len()
    }

    /// get methods of this class
    pub fn methods(&self) -> MethodDataList<'a> {
        self.methods
            .get_or_init(|| {
                let encode_ids: Vec<i64> = self
                    .method_ids
                    .iter()
                    .map(|&id| Self::get_encode_id(self.dex_id, id as u32))
                    .collect();
                self.bridge.get_method_by_ids(&encode_ids)
            })
            .clone()
    }

    /// get declared methods count
    pub fn method_count(&self) -> usize {
        self.method_ids.len()
    }

    /// get fields of this class
    pub fn fields(&self) -> FieldDataList<'a> {
        self.fields
            .get_or_init(|| {
                let encode_ids: Vec<i64> = self
                    .field_ids
                    .iter()
                    .map(|&id| Self::get_encode_id(self.dex_id, id as u32))
                    .collect();
                self.bridge.get_field_by_ids(&encode_ids)
            })
            .clone()
    }

    /// get declared fields count
    pub fn field_count(&self) -> usize {
        self.field_ids.len()
    }

    /// get annotations of this class
    pub fn annotations(&self) -> Vec<AnnotationData<'a>> {
        self.annotations
            .get_or_init(|| self.bridge.get_class_annotations(self.id as i64))
            .clone()
    }

    /// convert to `DexClass`, None if parse failed
    pub fn to_dex_type(&self) -> Option<DexClass> {
        self.get_dex_class().cloned()
    }

    /// internal use, get the wrapped DexClass
    pub(crate) fn get_dex_class(&self) -> Option<&DexClass> {
        self.dex_class
            .get_or_init(|| DexClass::deserialize(&self.descriptor))
            .as_ref()
    }

    /// ...
    pub(crate) fn with_meta(bridge: &'a DexkitBridge, meta: FBClassMeta<'a>) -> Self {
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
            bridge,
            id,
            dex_id,
            source_file,
            modifiers,
            descriptor,
            super_class_id,
            interface_ids,
            method_ids,
            field_ids,
            // Lazy loaded fields
            dex_class: OnceCell::new(),
            super_class: OnceCell::new(),
            interfaces: OnceCell::new(),
            methods: OnceCell::new(),
            fields: OnceCell::new(),
            annotations: OnceCell::new(),
        }
    }

    /// ...
    pub(crate) fn with_meta_raw(bridge: &'a DexkitBridge, data: &'a [u8]) -> Option<Self> {
        flatbuffers::root::<FBClassMeta<'_>>(data)
            .map(|meta| Self::with_meta(bridge, meta))
            .ok()
    }
}
