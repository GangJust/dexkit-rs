use crate::BridgeCore;
use crate::errors::Error;
use crate::query::{
    BatchFindClassUsingStrings, BatchFindMethodUsingStrings, FindClass, FindField, FindMethod,
};
use crate::result::{
    ClassData, ClassDataList, FieldData, FieldDataList, MethodData, MethodDataList,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DexkitBridge {
    core: BridgeCore,
}

impl DexkitBridge {
    pub(crate) fn from_core(core: BridgeCore) -> Self {
        Self { core }
    }

    pub(crate) fn core_clone(&self) -> BridgeCore {
        self.core.clone()
    }

    pub fn new<S>(apk_path: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        BridgeCore::new(apk_path).map(Self::from_core)
    }

    pub fn from_dex_bytes<B>(dex_bytes: B) -> Result<Self, Error>
    where
        B: AsRef<[u8]>,
    {
        BridgeCore::from_dex_bytes(dex_bytes).map(Self::from_core)
    }

    pub fn from_dex_bytes_array<I, B>(dex_bytes_array: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        BridgeCore::from_dex_bytes_array(dex_bytes_array).map(Self::from_core)
    }

    pub fn close(&self) {
        self.core.close();
    }

    pub fn init_full_cache(&self) -> Result<(), Error> {
        self.core.init_full_cache()
    }

    pub fn set_thread_num(&self, num_threads: i32) {
        self.core.set_thread_num(num_threads);
    }

    pub fn set_max_concurrent_queries(&self, max_concurrent_queries: u32) {
        self.core
            .set_max_concurrent_queries(max_concurrent_queries);
    }

    pub fn get_dex_num(&self) -> i32 {
        self.core.get_dex_num()
    }

    pub fn export_dex_file(&self, output_path: &str) -> Result<(), Error> {
        self.core.export_dex_file(output_path)
    }

    pub fn batch_find_class_using_strings(
        &self,
        batch_find: BatchFindClassUsingStrings,
    ) -> HashMap<String, ClassDataList> {
        self.core.batch_find_class_using_strings(batch_find)
    }

    pub fn batch_find_method_using_strings(
        &self,
        batch_find: BatchFindMethodUsingStrings,
    ) -> HashMap<String, MethodDataList> {
        self.core.batch_find_method_using_strings(batch_find)
    }

    pub fn find_class(&self, find_class: FindClass) -> ClassDataList {
        self.core.find_class(find_class)
    }

    pub fn find_method(&self, find_method: FindMethod) -> MethodDataList {
        self.core.find_method(find_method)
    }

    pub fn find_field(&self, find_field: FindField) -> FieldDataList {
        self.core.find_field(find_field)
    }

    pub fn get_class_data<T>(&self, identifier: T) -> Option<ClassData>
    where
        T: AsRef<str>,
    {
        self.core.get_class_data(identifier)
    }

    pub fn get_method_data<T>(&self, descriptor: T) -> Option<MethodData>
    where
        T: AsRef<str>,
    {
        self.core.get_method_data(descriptor)
    }

    pub fn get_field_data<T>(&self, descriptor: T) -> Option<FieldData>
    where
        T: AsRef<str>,
    {
        self.core.get_field_data(descriptor)
    }
}
