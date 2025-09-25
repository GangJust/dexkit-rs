use crate::{
    errors::Error,
    query::{
        BatchFindClassUsingStrings, BatchFindMethodUsingStrings, FindClass, FindField, FindMethod,
    },
    result::{
        AnnotationData, ClassData, ClassDataList, FieldData, FieldDataList, MethodData,
        MethodDataList,
    },
    wrap::{DexClass, DexMethod},
};
use std::{
    collections::HashMap,
    ffi::{CString, c_char, c_void},
};

#[derive(Debug)]
pub struct DexkitBridge {
    dexkit_handle: dexkit_sys::DexkitHandle,
}

impl DexkitBridge {
    /// Create a new DexkitBridge instance with the given APK path.
    /// Panics if the APK path cannot be added.
    pub fn create_apk_path<S: Into<String>>(apk_path: S) -> Result<Self, Error> {
        let dexkit_handle = unsafe { dexkit_sys::dexkit_new() };
        let c_apk_path =
            CString::new(apk_path.into()).map_err(|e| Error::BridgeCreateError(e.to_string()))?;
        let added = unsafe {
            dexkit_sys::dexkit_add_zip_path(dexkit_handle, c_apk_path.as_ptr() as *mut i8, 0)
        };
        if added == 0 {
            return Err(Error::BridgeCreateError("Failed to add APK path".into()));
        }

        Ok(DexkitBridge { dexkit_handle })
    }

    /// Free the DexkitBridge instance and its resources.
    pub fn close(&self) {
        unsafe { dexkit_sys::dexkit_free(self.dexkit_handle) };
    }

    /// Initialize the full cache for faster queries.
    pub fn init_full_cache(&self) -> Result<(), Error> {
        let res = unsafe { dexkit_sys::dexkit_init_full_cache(self.dexkit_handle) };
        if !res {
            return Err(Error::BridgeOperationError(
                "Failed to initialize full cache".into(),
            ));
        }
        Ok(())
    }

    /// Set the number of threads to use for operations.
    pub fn set_thread_num(&self, num_threads: i32) {
        unsafe { dexkit_sys::dexkit_set_thread_num(self.dexkit_handle, num_threads) };
    }

    /// Get all parsed DEX file count.
    pub fn get_dex_num(&self) -> i32 {
        unsafe { dexkit_sys::dexkit_get_dex_num(self.dexkit_handle) }
    }

    /// Export all parsed DEX files to the specified output path.
    /// Returns an error if the export fails.
    pub fn export_dex_file(&self, output_path: &str) -> Result<(), Error> {
        let c_output_path =
            CString::new(output_path).map_err(|e| Error::BridgeOperationError(e.to_string()))?;
        let success = unsafe {
            dexkit_sys::dexkit_export_dex_file(
                self.dexkit_handle,
                c_output_path.as_ptr() as *mut c_char,
            )
        };
        if !success {
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
                CString::new(descriptor).unwrap().as_ptr() as *mut c_char,
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
                self.dexkit_handle,
                CString::new(descriptor).unwrap().as_ptr() as *mut c_char,
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
    pub fn get_filed_data<T>(&self, descriptor: T) -> Option<FieldData<'_>>
    where
        T: AsRef<str>,
    {
        let descriptor = descriptor.as_ref();

        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_field_data(
                self.dexkit_handle,
                CString::new(descriptor).unwrap().as_ptr() as *mut c_char,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_parameter_names(
                self.dexkit_handle,
                method_id,
                &mut out_buf,
                &mut out_len,
            );

            if out_buf.is_null() || out_len == 0 {
                return None;
            }

            // cast the out_buf to char**
            let char_ptr_array = out_buf as *mut *mut c_char;
            let mut names: Vec<Option<String>> = Vec::with_capacity(out_len);

            for i in 0..out_len {
                let char_ptr = *char_ptr_array.add(i);
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

            dexkit_sys::dexkit_get_parameter_names_free(char_ptr_array, out_len); // release the memory allocated by layer C
            Some(names)
        }
    }

    /// Get parameter annotations for a method by its method ID.
    pub(crate) fn get_parameter_annotations(&self, method_id: i64) -> Vec<Vec<AnnotationData<'_>>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_parameter_annotations(
                self.dexkit_handle,
                method_id,
                &mut out_buf,
                &mut out_len,
            );
            let _data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };
            //todo: parse _data to Vec<Vec<AnnotationData<'_>>>
            dexkit_sys::dexkit_get_parameter_annotations_free(&mut out_buf, out_len); // release the memory allocated by layer C
            Vec::new() //todo: implement
        }
    }

    /// Get the op codes of a method by its encoded ID, range 0~255, may be None if not available
    pub(crate) fn get_method_op_codes(&self, encode_id: i64) -> Option<Vec<u8>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_method_op_codes(
                self.dexkit_handle,
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
                self.dexkit_handle,
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
                self.dexkit_handle,
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
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_method_using_strings(
                self.dexkit_handle,
                method_id,
                &mut out_buf,
                &mut out_len,
            );

            if out_buf.is_null() || out_len == 0 {
                return Vec::new();
            }

            // cast the out_buf to char**
            let char_ptr_array = out_buf as *mut *mut c_char;
            let mut names: Vec<String> = Vec::with_capacity(out_len);

            for i in 0..out_len {
                let char_ptr = *char_ptr_array.add(i);
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

            dexkit_sys::dexkit_get_method_using_strings_free(char_ptr_array, out_len); // release the memory allocated by layer C
            names
        }
    }

    /// Get all fields used in this method by method ID
    pub(crate) fn get_method_using_fields(
        &self,
        method_id: i64,
    ) -> Vec<crate::result::UsingFieldData<'_>> {
        unsafe {
            let mut out_buf: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            dexkit_sys::dexkit_get_method_using_fields(
                self.dexkit_handle,
                method_id,
                &mut out_buf,
                &mut out_len,
            );

            let data = if !out_buf.is_null() && out_len > 0 {
                std::slice::from_raw_parts(out_buf as *const u8, out_len)
            } else {
                &[]
            };

            let result = crate::result::UsingFieldData::with_using_field_meta_array_raw(self, data);
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
