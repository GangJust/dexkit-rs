use crate::{
    errors::Error,
    query::{FindClass, FindField, FindMethod},
    result::{ClassDataList, FieldDataList, MethodDataList},
};
use std::ffi::{CString, c_void};

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
}
