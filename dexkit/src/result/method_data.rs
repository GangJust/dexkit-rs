use crate::gen_flatbuffers::dexkit::schema::MethodMeta as FBMethodMeta;
use crate::result::{AnnotationData, ClassData, ClassDataList, MethodDataList, UsingFieldData};
use crate::uitls::Opcodes;
use crate::wrap::DexMethod;
use crate::{dexkit_bridge::DexkitBridge, result::base::BaseData};
use std::cell::OnceCell;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct MethodData<'a> {
    bridge: &'a DexkitBridge,
    id: u32,
    dex_id: u32,
    class_id: u32,
    modifiers: u32,
    descriptor: String,
    return_type_id: u32,
    param_type_ids: Vec<i32>,
    // Lazy loaded fields
    dex_method: OnceCell<Option<DexMethod>>,
    declared_class: OnceCell<Option<ClassData<'a>>>,
    return_type_class: OnceCell<Option<ClassData<'a>>>,
    param_types: OnceCell<Option<ClassDataList<'a>>>,
    param_names: OnceCell<Option<Vec<Option<String>>>>,
    annotations: OnceCell<Vec<AnnotationData<'a>>>,
    param_annotations: OnceCell<Vec<Vec<AnnotationData<'a>>>>,
    op_codes: OnceCell<Option<Vec<u8>>>,
    callers: OnceCell<MethodDataList<'a>>,
    invokes: OnceCell<MethodDataList<'a>>,
    using_strings: OnceCell<Vec<String>>,
    using_fields: OnceCell<Vec<UsingFieldData<'a>>>,
}

impl<'a> BaseData for MethodData<'a> {
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

impl<'a> MethodData<'a> {
    /// modifiers bitmask, see `Modifier`
    pub fn modifiers(&self) -> u32 {
        self.modifiers
    }

    /// field descriptor, e.g. "Lcom/example/MyClass;->myMethod(I)V"
    pub fn descriptor(&self) -> String {
        self.descriptor.to_string()
    }

    /// method signature, e.g. "myMethod(ILjava/lang/String;)V"
    pub fn method_signature(&self) -> Option<String> {
        self.get_dex_method().map(|m| m.method_signature())
    }

    /// method class name, e.g. "com.example.MyClass"
    pub fn class_name(&self) -> Option<String> {
        self.get_dex_method().map(|m| m.class_name())
    }

    /// method class name, e.g. "com.example.MyClass"
    pub fn declared_class_name(&self) -> Option<String> {
        self.class_name()
    }

    /// method name, e.g. "myMethod"
    pub fn method_name(&self) -> Option<String> {
        self.get_dex_method().map(|m| m.method_name())
    }

    /// method name, e.g. "myMethod"
    pub fn name(&self) -> Option<String> {
        self.method_name()
    }

    /// parameter type names, e.g. vec!["int", "java.lang.String"]
    pub fn param_type_names(&self) -> Option<Vec<String>> {
        self.get_dex_method().map(|m| m.param_type_names())
    }

    /// parameter type count
    pub fn param_count(&self) -> usize {
        self.param_type_ids.len()
    }

    /// return type name, e.g. "void"
    pub fn return_type_name(&self) -> Option<String> {
        self.get_dex_method().map(|m| m.return_type_name())
    }

    /// is constructor method
    pub fn is_constructor(&self) -> bool {
        self.name().as_deref() == Some("<init>")
    }

    /// is static initializer method
    pub fn is_static_initializer(&self) -> bool {
        self.name().as_deref() == Some("<clinit>")
    }

    /// is method
    pub fn is_method(&self) -> bool {
        !self.is_static_initializer() && !self.is_constructor()
    }

    /// get the class where the method is declared
    pub fn declared_class(&self) -> Option<ClassData<'a>> {
        self.declared_class
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.class_id);
                self.bridge
                    .get_type_by_ids(&vec![encode_id])
                    .get(0)
                    .cloned()
            })
            .clone()
    }

    /// get the class of the return type
    pub fn return_type_class(&self) -> Option<ClassData<'a>> {
        self.return_type_class
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.return_type_id);
                self.bridge
                    .get_type_by_ids(&vec![encode_id])
                    .get(0)
                    .cloned()
            })
            .clone()
    }

    /// get the classes of the parameter types
    pub fn param_types(&self) -> Option<ClassDataList<'a>> {
        self.param_types
            .get_or_init(|| {
                let encode_ids: Vec<i64> = self
                    .param_type_ids
                    .iter()
                    .map(|&type_id| Self::get_encode_id(self.dex_id, type_id as u32))
                    .collect();
                if encode_ids.is_empty() {
                    return None;
                }
                let types = self.bridge.get_type_by_ids(&encode_ids);
                if types.is_empty() { None } else { Some(types) }
            })
            .clone()
    }

    /// get the parameter names, may be None if not available
    pub fn param_names(&self) -> Option<Vec<Option<String>>> {
        self.param_names
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.get_parameter_names(encode_id)
            })
            .clone()
    }

    /// get annotations of this class
    pub fn annotations(&self) -> Vec<AnnotationData<'a>> {
        self.annotations
            .get_or_init(|| self.bridge.get_method_annotations(self.id as i64))
            .clone()
    }

    /// get parameter annotations of this method
    pub fn param_annotations(&self) -> Vec<Vec<AnnotationData<'a>>> {
        self.param_annotations
            .get_or_init(|| self.bridge.get_parameter_annotations(self.id as i64))
            .clone()
    }

    /// get the op codes of this method, range 0~255, may be None if not available
    pub fn op_codes(&self) -> Option<Vec<u8>> {
        self.op_codes
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.get_method_op_codes(encode_id)
            })
            .clone()
    }

    /// get the op code names of this method, may be None if not available
    pub fn op_names(&self) -> Option<Vec<String>> {
        self.op_codes().map(|codes| {
            codes
                .iter()
                .map(|&code| Opcodes::get_op_format(code).unwrap_or_default())
                .collect()
        })
    }

    /// get the method that calls this method
    pub fn callers(&self) -> MethodDataList<'a> {
        self.callers
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.get_call_methods(encode_id)
            })
            .clone()
    }

    /// get the methods that this method invokes
    pub fn invokes(&self) -> MethodDataList<'a> {
        self.invokes
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.get_invoke_methods(encode_id)
            })
            .clone()
    }

    /// get the string literals used in this method
    pub fn using_strings(&self) -> Vec<String> {
        self.using_strings
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.get_method_using_strings(encode_id)
            })
            .clone()
    }

    /// get the fields used in this method
    pub fn using_fields(&self) -> Vec<UsingFieldData<'a>> {
        self.using_fields
            .get_or_init(|| {
                let encode_id = Self::get_encode_id(self.dex_id, self.id);
                self.bridge.get_method_using_fields(encode_id)
            })
            .clone()
    }

    /// get the wrapped DexMethod
    pub fn to_dex_method(&self) -> Option<DexMethod> {
        self.get_dex_method().cloned()
    }

    /// internal use, get the wrapped DexMethod
    pub(crate) fn get_dex_method(&self) -> Option<&DexMethod> {
        self.dex_method
            .get_or_init(|| DexMethod::deserialize(self.descriptor.as_str()))
            .as_ref()
    }

    /// ...
    pub(crate) fn with_meta(bridge: &'a DexkitBridge, meta: FBMethodMeta<'a>) -> Self {
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
            bridge,
            id,
            dex_id,
            class_id,
            modifiers,
            descriptor,
            return_type_id,
            param_type_ids,
            dex_method: OnceCell::new(),
            declared_class: OnceCell::new(),
            return_type_class: OnceCell::new(),
            param_types: OnceCell::new(),
            param_names: OnceCell::new(),
            annotations: OnceCell::new(),
            param_annotations: OnceCell::new(),
            op_codes: OnceCell::new(),
            callers: OnceCell::new(),
            invokes: OnceCell::new(),
            using_strings: OnceCell::new(),
            using_fields: OnceCell::new(),
        }
    }

    /// ...
    pub(crate) fn from_meta_raw(bridge: &'a DexkitBridge, data: &'a [u8]) -> Option<Self> {
        flatbuffers::root::<FBMethodMeta<'_>>(data)
            .map(|meta| Self::with_meta(bridge, meta))
            .ok()
    }
}
