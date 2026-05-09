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

    /// Create a new bridge from an APK or ZIP path.
    pub fn new<S>(apk_path: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        BridgeCore::new(apk_path).map(Self::from_core)
    }

    /// Create a new bridge from a single in-memory dex image.
    pub fn from_dex_bytes<B>(dex_bytes: B) -> Result<Self, Error>
    where
        B: AsRef<[u8]>,
    {
        BridgeCore::from_dex_bytes(dex_bytes).map(Self::from_core)
    }

    /// Create a new bridge from multiple in-memory dex images.
    pub fn from_dex_bytes_array<I, B>(dex_bytes_array: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        BridgeCore::from_dex_bytes_array(dex_bytes_array).map(Self::from_core)
    }

    /// Free the underlying native bridge resources.
    ///
    /// Calling this more than once is safe.
    pub fn close(&self) {
        self.core.close();
    }

    /// Initialize the full cache for faster follow-up queries.
    pub fn init_full_cache(&self) -> Result<(), Error> {
        self.core.init_full_cache()
    }

    /// Set the number of native worker threads used by DexKit.
    pub fn set_thread_num(&self, num_threads: i32) {
        self.core.set_thread_num(num_threads);
    }

    /// Set the maximum number of concurrent queries.
    ///
    /// `0` keeps DexKit's default behavior without an explicit limit.
    pub fn set_max_concurrent_queries(&self, max_concurrent_queries: u32) {
        self.core
            .set_max_concurrent_queries(max_concurrent_queries);
    }

    /// Get the number of parsed dex files.
    pub fn get_dex_num(&self) -> i32 {
        self.core.get_dex_num()
    }

    /// Export all parsed dex files to the specified output path.
    pub fn export_dex_file(&self, output_path: &str) -> Result<(), Error> {
        self.core.export_dex_file(output_path)
    }

    /// Batch find classes for the provided grouped string query.
    pub fn batch_find_class_using_strings(
        &self,
        batch_find: BatchFindClassUsingStrings,
    ) -> HashMap<String, ClassDataList> {
        self.core.batch_find_class_using_strings(batch_find)
    }

    /// Batch find methods for the provided grouped string query.
    pub fn batch_find_method_using_strings(
        &self,
        batch_find: BatchFindMethodUsingStrings,
    ) -> HashMap<String, MethodDataList> {
        self.core.batch_find_method_using_strings(batch_find)
    }

    /// Find classes that match the provided query.
    pub fn find_class(&self, find_class: FindClass) -> ClassDataList {
        self.core.find_class(find_class)
    }

    /// Find methods that match the provided query.
    pub fn find_method(&self, find_method: FindMethod) -> MethodDataList {
        self.core.find_method(find_method)
    }

    /// Find fields that match the provided query.
    pub fn find_field(&self, find_field: FindField) -> FieldDataList {
        self.core.find_field(find_field)
    }

    /// Get class metadata by descriptor or dotted class name.
    pub fn get_class_data<T>(&self, identifier: T) -> Option<ClassData>
    where
        T: AsRef<str>,
    {
        self.core.get_class_data(identifier)
    }

    /// Get method metadata by descriptor.
    pub fn get_method_data<T>(&self, descriptor: T) -> Option<MethodData>
    where
        T: AsRef<str>,
    {
        self.core.get_method_data(descriptor)
    }

    /// Get field metadata by descriptor.
    pub fn get_field_data<T>(&self, descriptor: T) -> Option<FieldData>
    where
        T: AsRef<str>,
    {
        self.core.get_field_data(descriptor)
    }
}
