use crate::{
    errors::Error,
    query::{FindClass, FindField, FindMethod},
    result::{AnnotationData, ClassDataList, FieldDataList, MethodDataList},
};
use std::ffi::{CString, c_char, c_void};

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
    pub fn free(&self) {
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

    /// Find classes based on the provided FindClass query.
    /// Returns a ClassDataList containing the results.
    pub fn find_class(&self, find_class: FindClass) -> ClassDataList<'_> {
        let mut buffer: Vec<u8> = find_class.into();
        let mut out_buf: *mut c_void = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let data = unsafe {
            dexkit_sys::dexkit_find_class(
                self.dexkit_handle,
                buffer.as_mut_ptr() as *mut c_void,
                &mut out_buf,
                &mut out_len,
            );
            if !out_buf.is_null() && out_len > 0 {
                let result = std::slice::from_raw_parts(out_buf as *const u8, out_len);
                // Assuming the C side allocates memory for out_buf, we should free it here if needed.
                // For example, if there's a corresponding `dexkit_free_buffer` function, call it here.
                // dexkit_free_buffer(out_buf);
                result
            } else {
                &[]
            }
        };

        ClassDataList::form_data(self, data)
    }

    /// Find methods based on the provided FindMethod query.
    /// Returns a MethodDataList containing the results.
    pub fn find_method(&self, find_method: FindMethod) -> MethodDataList<'_> {
        let mut buffer: Vec<u8> = find_method.into();
        let mut out_buf: *mut c_void = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let data = unsafe {
            dexkit_sys::dexkit_find_method(
                self.dexkit_handle,
                buffer.as_mut_ptr() as *mut c_void,
                &mut out_buf,
                &mut out_len,
            );
            if !out_buf.is_null() && out_len > 0 {
                let result = std::slice::from_raw_parts(out_buf as *const u8, out_len);
                // Assuming the C side allocates memory for out_buf, we should free it here if needed.
                // For example, if there's a corresponding `dexkit_free_buffer` function, call it here.
                // dexkit_free_buffer(out_buf);
                result
            } else {
                &[]
            }
        };

        MethodDataList::form_data(self, data)
    }

    /// Find fields based on the provided FindField query.
    /// Returns a FieldDataList containing the results.
    pub fn find_field(&self, find_field: FindField) -> FieldDataList<'_> {
        let mut buffer: Vec<u8> = find_field.into();
        let mut out_buf: *mut c_void = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let data = unsafe {
            dexkit_sys::dexkit_find_field(
                self.dexkit_handle,
                buffer.as_mut_ptr() as *mut c_void,
                &mut out_buf,
                &mut out_len,
            );
            if !out_buf.is_null() && out_len > 0 {
                let result = std::slice::from_raw_parts(out_buf as *const u8, out_len);
                // Assuming the C side allocates memory for out_buf, we should free it here if needed.
                // For example, if there's a corresponding `dexkit_free_buffer` function, call it here.
                // dexkit_free_buffer(out_buf);
                result
            } else {
                &[]
            }
        };

        FieldDataList::form_data(self, data)
    }

    /// Get classes by their encoded IDs.
    pub(crate) fn get_class_by_ids(&self, encode_id_array: &[i64]) -> ClassDataList<'_> {
        let mut out_buf: *mut c_void = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let data = unsafe {
            dexkit_sys::dexkit_get_class_by_ids(
                self.dexkit_handle,
                encode_id_array.as_ptr() as *mut c_void,
                encode_id_array.len(),
                &mut out_buf,
                &mut out_len,
            );
            if !out_buf.is_null() && out_len > 0 {
                let result = std::slice::from_raw_parts(out_buf as *const u8, out_len);
                // Assuming the C side allocates memory for out_buf, we should free it here if needed.
                // For example, if there's a corresponding `dexkit_free_buffer` function, call it here.
                // dexkit_free_buffer(out_buf);
                result
            } else {
                &[]
            }
        };

        ClassDataList::form_data(self, data)
    }

    /// Get methods by their encoded IDs.
    pub(crate) fn get_method_by_ids(&self, encode_id_array: &[i64]) -> MethodDataList<'_> {
        let mut out_buf: *mut c_void = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let data = unsafe {
            dexkit_sys::dexkit_get_method_by_ids(
                self.dexkit_handle,
                encode_id_array.as_ptr() as *mut c_void,
                encode_id_array.len(),
                &mut out_buf,
                &mut out_len,
            );
            if !out_buf.is_null() && out_len > 0 {
                let result = std::slice::from_raw_parts(out_buf as *const u8, out_len);
                // Assuming the C side allocates memory for out_buf, we should free it here if needed.
                // For example, if there's a corresponding `dexkit_free_buffer` function, call it here.
                // dexkit_free_buffer(out_buf);
                result
            } else {
                &[]
            }
        };

        MethodDataList::form_data(self, data)
    }

    /// Get fields by their encoded IDs.
    pub(crate) fn get_field_by_ids(&self, encode_id_array: &[i64]) -> FieldDataList<'_> {
        let mut out_buf: *mut c_void = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let data = unsafe {
            dexkit_sys::dexkit_get_field_by_ids(
                self.dexkit_handle,
                encode_id_array.as_ptr() as *mut c_void,
                encode_id_array.len(),
                &mut out_buf,
                &mut out_len,
            );
            if !out_buf.is_null() && out_len > 0 {
                let result = std::slice::from_raw_parts(out_buf as *const u8, out_len);
                // Assuming the C side allocates memory for out_buf, we should free it here if needed.
                // For example, if there's a corresponding `dexkit_free_buffer` function, call it here.
                // dexkit_free_buffer(out_buf);
                result
            } else {
                &[]
            }
        };

        FieldDataList::form_data(self, data)
    }

    /// Get annotations for a class by its class ID.
    pub(crate) fn get_class_annotations(&self, class_id: i64) -> Vec<AnnotationData<'_>> {
        let mut out_buf: *mut c_void = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let data = unsafe {
            dexkit_sys::dexkit_get_class_annotations(
                self.dexkit_handle,
                class_id,
                &mut out_buf,
                &mut out_len,
            );
            if !out_buf.is_null() && out_len > 0 {
                let result = std::slice::from_raw_parts(out_buf as *const u8, out_len);
                // Assuming the C side allocates memory for out_buf, we should free it here if needed.
                // For example, if there's a corresponding `dexkit_free_buffer` function, call it here.
                // dexkit_free_buffer(out_buf);
                result
            } else {
                &[]
            }
        };

        AnnotationData::with_data_raw(self, data)
    }
}

impl Drop for DexkitBridge {
    fn drop(&mut self) {
        self.free();
    }
}
