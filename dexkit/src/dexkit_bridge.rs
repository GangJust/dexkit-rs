use crate::errors::Error;
use crate::query::{
    BatchFindClassUsingStrings, BatchFindMethodUsingStrings, FindClass, FindField, FindMethod,
};
use crate::result::{
    AnnotationData, ClassData, ClassDataList, FieldData, FieldDataList, MethodData, MethodDataList,
    UsingFieldData,
};
use crate::wrap::{DexClass, DexMethod};
use std::{
    cell::Cell,
    collections::HashMap,
    ffi::{CString, c_char, c_void},
};

#[derive(Debug)]
pub struct DexkitBridge {
    dexkit_handle: Cell<dexkit_sys::DexkitHandle>,
}

impl DexkitBridge {
    fn handle(&self) -> dexkit_sys::DexkitHandle {
        let handle = self.dexkit_handle.get();
        assert!(
            !handle.is_null(),
            "DexkitBridge has already been closed and cannot be used"
        );
        handle
    }

    /// Create a new DexkitBridge instance with the given APK path.
    pub fn new<S>(apk_path: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let dexkit_handle = unsafe { dexkit_sys::dexkit_new() };
        let c_apk_path =
            CString::new(apk_path.as_ref()).map_err(|e| Error::BridgeCreateError(e.to_string()))?;
        let added =
            unsafe { dexkit_sys::dexkit_add_zip_path(dexkit_handle, c_apk_path.as_ptr(), 0) };
        if added == 0 {
            return Err(Error::BridgeCreateError("Failed to add APK path".into()));
        }

        Ok(DexkitBridge {
            dexkit_handle: Cell::new(dexkit_handle),
        })
    }

    /// Create a new DexkitBridge instance from an in-memory dex image.
    pub fn from_dex_bytes<B>(dex_bytes: B) -> Result<Self, Error>
    where
        B: AsRef<[u8]>,
    {
        let dexkit_handle = unsafe { dexkit_sys::dexkit_new() };
        let dex_bytes = dex_bytes.as_ref();
        let added = unsafe {
            dexkit_sys::dexkit_add_dex_bytes(dexkit_handle, dex_bytes.as_ptr(), dex_bytes.len())
        };
        if added == 0 {
            unsafe { dexkit_sys::dexkit_free(dexkit_handle) };
            return Err(Error::BridgeCreateError("Failed to add dex bytes".into()));
        }

        Ok(DexkitBridge {
            dexkit_handle: Cell::new(dexkit_handle),
        })
    }

    /// Create a new DexkitBridge instance from multiple in-memory dex images.
    pub fn from_dex_bytes_array<I, B>(dex_bytes_array: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        let dex_bytes: Vec<Vec<u8>> = dex_bytes_array
            .into_iter()
            .map(|bytes| bytes.as_ref().to_vec())
            .collect();
        let dex_ptrs: Vec<*const u8> = dex_bytes.iter().map(|bytes| bytes.as_ptr()).collect();
        let dex_lens: Vec<usize> = dex_bytes.iter().map(|bytes| bytes.len()).collect();

        let dexkit_handle = unsafe { dexkit_sys::dexkit_new() };
        let added = unsafe {
            dexkit_sys::dexkit_add_dex_bytes_array(
                dexkit_handle,
                dex_ptrs.as_ptr(),
                dex_lens.as_ptr(),
                dex_ptrs.len(),
            )
        };
        if added == 0 {
            unsafe { dexkit_sys::dexkit_free(dexkit_handle) };
            return Err(Error::BridgeCreateError(
                "Failed to add dex bytes array".into(),
            ));
        }

        Ok(DexkitBridge {
            dexkit_handle: Cell::new(dexkit_handle),
        })
    }

    /// Free the DexkitBridge instance and its resources.
    /// Calling this more than once is safe.
    pub fn close(&self) {
        let handle = self.dexkit_handle.replace(std::ptr::null_mut());
        if !handle.is_null() {
            unsafe { dexkit_sys::dexkit_free(handle) };
        }
    }

    /// Initialize the full cache for faster queries.
    pub fn init_full_cache(&self) -> Result<(), Error> {
        let res = unsafe { dexkit_sys::dexkit_init_full_cache(self.handle()) };
        if res == 0 {
            return Err(Error::BridgeOperationError(
                "Failed to initialize full cache".into(),
            ));
        }
        Ok(())
    }

    /// Set the number of threads to use for operations.
    pub fn set_thread_num(&self, num_threads: i32) {
        unsafe { dexkit_sys::dexkit_set_thread_num(self.handle(), num_threads) };
    }

    /// Get all parsed DEX file count.
    pub fn get_dex_num(&self) -> i32 {
        unsafe { dexkit_sys::dexkit_get_dex_num(self.handle()) }
    }

    /// Export all parsed DEX files to the specified output path.
    /// Returns an error if the export fails.
    pub fn export_dex_file(&self, output_path: &str) -> Result<(), Error> {
        let c_output_path =
            CString::new(output_path).map_err(|e| Error::BridgeOperationError(e.to_string()))?;
        let success =
            unsafe { dexkit_sys::dexkit_export_dex_file(self.handle(), c_output_path.as_ptr()) };
        if success == 0 {
            return Err(Error::BridgeOperationError(
                "Failed to export DEX file".into(),
            ));
        }
        Ok(())
    }

    /// Batch find classes based on the provided BatchFindClassUsingStrings query.
    pub fn batch_find_class_using_strings(
        &self,
        batch_find: BatchFindClassUsingStrings,
    ) -> HashMap<String, ClassDataList<'_>> {
        unsafe {
            let mut buffer: Vec<u8> = batch_find.into();
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_batch_find_class_using_strings(
                self.handle(),
                buffer.as_mut_ptr() as *mut c_void,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = ClassDataList::from_batch_data(self, data);
            dexkit_sys::dexkit_batch_find_class_using_strings_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Batch find methods based on the provided BatchFindMethodUsingStrings query.
    pub fn batch_find_method_using_strings(
        &self,
        batch_find: BatchFindMethodUsingStrings,
    ) -> HashMap<String, MethodDataList<'_>> {
        unsafe {
            let mut buffer: Vec<u8> = batch_find.into();
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_batch_find_method_using_strings(
                self.handle(),
                buffer.as_mut_ptr() as *mut c_void,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = MethodDataList::from_batch_data(self, data);
            dexkit_sys::dexkit_batch_find_method_using_strings_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Find classes based on the provided FindClass query.
    /// Returns a ClassDataList containing the results.
    pub fn find_class(&self, find_class: FindClass) -> ClassDataList<'_> {
        unsafe {
            let mut buffer: Vec<u8> = find_class.into();
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_find_class(
                self.handle(),
                buffer.as_mut_ptr() as *mut c_void,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = ClassDataList::from_data(self, data);
            dexkit_sys::dexkit_find_class_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Find methods based on the provided FindMethod query.
    /// Returns a MethodDataList containing the results.
    pub fn find_method(&self, find_method: FindMethod) -> MethodDataList<'_> {
        unsafe {
            let mut buffer: Vec<u8> = find_method.into();
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_find_method(
                self.handle(),
                buffer.as_mut_ptr() as *mut c_void,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = MethodDataList::form_data(self, data);
            dexkit_sys::dexkit_find_method_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Find fields based on the provided FindField query.
    /// Returns a FieldDataList containing the results.
    pub fn find_field(&self, find_field: FindField) -> FieldDataList<'_> {
        unsafe {
            let mut buffer: Vec<u8> = find_field.into();
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_find_field(
                self.handle(),
                buffer.as_mut_ptr() as *mut c_void,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = FieldDataList::form_data(self, data);
            dexkit_sys::dexkit_find_field_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get class data by its descriptor or simple name.
    /// The identifier can be a full descriptor (e.g., "Lcom/example/MyClass;")
    pub fn get_class_data<T>(&self, identifier: T) -> Option<ClassData<'_>>
    where
        T: AsRef<str>,
    {
        let descriptor = identifier.as_ref();
        let descriptor = if descriptor.starts_with("L") && descriptor.ends_with(";") {
            descriptor
        } else {
            &format!("L{};", descriptor.replace('.', "/"))
        };

        // validate the descriptor
        if DexClass::deserialize(descriptor).is_none() {
            return None;
        }

        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_class_data(
                self.handle(),
                CString::new(descriptor).unwrap().as_ptr(),
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = ClassData::with_meta_raw(self, data);
            dexkit_sys::dexkit_get_class_data_free(&mut out_buf, out_len);
            result
        }
    }

    /// Get method data by its descriptor.
    pub fn get_method_data<T>(&self, descriptor: T) -> Option<MethodData<'_>>
    where
        T: AsRef<str>,
    {
        let descriptor = descriptor.as_ref();

        // validate the method signature
        if DexMethod::deserialize(descriptor).is_none() {
            return None;
        }

        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_method_data(
                self.handle(),
                CString::new(descriptor).unwrap().as_ptr(),
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = MethodData::from_meta_raw(self, data);
            dexkit_sys::dexkit_get_method_data_free(&mut out_buf, out_len);
            result
        }
    }

    /// Get field data by its descriptor.
    pub fn get_field_data<T>(&self, descriptor: T) -> Option<FieldData<'_>>
    where
        T: AsRef<str>,
    {
        let descriptor = descriptor.as_ref();

        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_field_data(
                self.handle(),
                CString::new(descriptor).unwrap().as_ptr(),
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = FieldData::with_meta_raw(self, data);
            dexkit_sys::dexkit_get_field_data_free(&mut out_buf, out_len);
            result
        }
    }

    /// ---> Internal use only --->
    /// Get classes by their encoded IDs.
    pub(crate) fn get_type_by_ids(&self, encode_id_array: &[i64]) -> ClassDataList<'_> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_class_by_ids(
                self.handle(),
                encode_id_array.as_ptr() as *mut c_void,
                encode_id_array.len(),
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = ClassDataList::from_data(self, data);
            dexkit_sys::dexkit_get_class_by_ids_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get methods by their encoded IDs.
    pub(crate) fn get_method_by_ids(&self, encode_id_array: &[i64]) -> MethodDataList<'_> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_method_by_ids(
                self.handle(),
                encode_id_array.as_ptr() as *mut c_void,
                encode_id_array.len(),
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = MethodDataList::form_data(self, data);
            dexkit_sys::dexkit_get_method_by_ids_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get fields by their encoded IDs.
    pub(crate) fn get_field_by_ids(&self, encode_id_array: &[i64]) -> FieldDataList<'_> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_field_by_ids(
                self.handle(),
                encode_id_array.as_ptr() as *mut c_void,
                encode_id_array.len(),
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = FieldDataList::form_data(self, data);
            dexkit_sys::dexkit_get_field_by_ids_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get annotations for a class by its class ID.
    pub(crate) fn get_class_annotations(&self, class_id: i64) -> Vec<AnnotationData<'_>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_class_annotations(
                self.handle(),
                class_id,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = AnnotationData::with_annotation_meta_array_raw(self, data);
            dexkit_sys::dexkit_get_class_annotations_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get annotations for a field by its field ID.
    pub(crate) fn get_field_annotations(&self, field_id: i64) -> Vec<AnnotationData<'_>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_field_annotations(
                self.handle(),
                field_id,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = AnnotationData::with_annotation_meta_array_raw(self, data);
            dexkit_sys::dexkit_get_field_annotations_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Read methods that reference the given field ID.
    pub(crate) fn read_field_methods(&self, field_id: i64) -> MethodDataList<'_> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_field_get_methods(
                self.handle(),
                field_id,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = MethodDataList::form_data(self, data);
            dexkit_sys::dexkit_field_get_methods_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Write methods that reference the given field ID.
    pub(crate) fn write_field_methods(&self, field_id: i64) -> MethodDataList<'_> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_field_put_methods(
                self.handle(),
                field_id,
                &mut out_buf,
                &mut out_len,
            );
            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };
            let result = MethodDataList::form_data(self, data);
            dexkit_sys::dexkit_field_put_methods_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get annotations for a method by its method ID.
    pub(crate) fn get_method_annotations(&self, method_id: i64) -> Vec<AnnotationData<'_>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;

            dexkit_sys::dexkit_get_method_annotations(
                self.handle(),
                method_id,
                &mut out_buf,
                &mut out_len,
            );
            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = AnnotationData::with_annotation_meta_array_raw(self, data);
            dexkit_sys::dexkit_get_method_annotations_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get parameter names for a method by its method ID.
    pub(crate) fn get_parameter_names(&self, method_id: i64) -> Option<Vec<Option<String>>> {
        unsafe {
            let mut out_buf: *mut *mut c_char = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_parameter_names(
                self.handle(),
                method_id,
                &mut out_buf,
                &mut out_len,
            );

            if out_buf.is_null() || out_len == 0 {
                return None;
            }

            let mut names: Vec<Option<String>> = Vec::with_capacity(out_len);

            for i in 0..out_len {
                let char_ptr = *out_buf.add(i);
                if char_ptr.is_null() {
                    names.push(None);
                } else {
                    // convert C string to Rust String
                    let c_str = std::ffi::CStr::from_ptr(char_ptr);
                    match c_str.to_str() {
                        Ok(s) => names.push(Some(s.to_owned())),
                        Err(_) => names.push(None), // invalid UTF-8 sequence
                    }
                }
            }

            dexkit_sys::dexkit_get_parameter_names_free(out_buf, out_len); // release the memory allocated by layer C
            Some(names)
        }
    }

    /// Get parameter annotations for a method by its method ID.
    pub(crate) fn get_parameter_annotations(&self, method_id: i64) -> Vec<Vec<AnnotationData<'_>>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_parameter_annotations(
                self.handle(),
                method_id,
                &mut out_buf,
                &mut out_len,
            );
            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = AnnotationData::with_parameters_annotation_meta_array_raw(self, data);
            dexkit_sys::dexkit_get_parameter_annotations_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get the op codes of a method by its encoded ID, range 0~255, may be None if not available
    pub(crate) fn get_method_op_codes(&self, encode_id: i64) -> Option<Vec<u8>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_method_op_codes(
                self.handle(),
                encode_id,
                &mut out_buf,
                &mut out_len,
            );

            if out_buf.is_null() || out_len == 0 {
                return None;
            }

            let data = std::slice::from_raw_parts(out_buf as *const u8, out_len);
            let result = Some(data.to_vec());
            dexkit_sys::dexkit_get_method_op_codes_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get the methods that this method calls by its method ID.
    pub(crate) fn get_call_methods(&self, method_id: i64) -> MethodDataList<'_> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_call_methods(
                self.handle(),
                method_id,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = MethodDataList::form_data(self, data);
            dexkit_sys::dexkit_get_call_methods_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get all methods invoke within this method by method ID
    pub(crate) fn get_invoke_methods(&self, method_id: i64) -> MethodDataList<'_> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_invoke_methods(
                self.handle(),
                method_id,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = MethodDataList::form_data(self, data);
            dexkit_sys::dexkit_get_invoke_methods_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }

    /// Get all string literals used in this method by method ID
    pub(crate) fn get_method_using_strings(&self, method_id: i64) -> Vec<String> {
        unsafe {
            let mut out_buf: *mut *mut c_char = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_method_using_strings(
                self.handle(),
                method_id,
                &mut out_buf,
                &mut out_len,
            );

            if out_buf.is_null() || out_len == 0 {
                return Vec::new();
            }

            let mut names: Vec<String> = Vec::with_capacity(out_len);

            for i in 0..out_len {
                let char_ptr = *out_buf.add(i);
                if char_ptr.is_null() {
                    continue;
                } else {
                    // convert C string to Rust String
                    let c_str = std::ffi::CStr::from_ptr(char_ptr);
                    match c_str.to_str() {
                        Ok(s) => names.push(s.to_owned()),
                        Err(_) => continue, // invalid UTF-8 sequence
                    }
                }
            }

            dexkit_sys::dexkit_get_method_using_strings_free(out_buf, out_len); // release the memory allocated by layer C
            names
        }
    }

    /// Get all fields used in this method by method ID
    pub(crate) fn get_method_using_fields(&self, method_id: i64) -> Vec<UsingFieldData<'_>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_method_using_fields(
                self.handle(),
                method_id,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = UsingFieldData::with_using_field_meta_array_raw(self, data);
            dexkit_sys::dexkit_get_method_using_fields_free(&mut out_buf, out_len); // release the memory allocated by layer C
            result
        }
    }
}

impl Drop for DexkitBridge {
    fn drop(&mut self) {
        self.close();
    }
}
