use crate::DexkitBridge;
use crate::errors::Error;
use crate::fb_codec::ToFbBytes;
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
    sync::Arc,
};

#[derive(Debug)]
struct BridgeHandle {
    dexkit_handle: Cell<dexkit_sys::DexkitHandle>,
}

impl BridgeHandle {
    fn new(handle: dexkit_sys::DexkitHandle) -> Self {
        Self {
            dexkit_handle: Cell::new(handle),
        }
    }

    fn handle(&self) -> dexkit_sys::DexkitHandle {
        let handle = self.dexkit_handle.get();
        assert!(
            !handle.is_null(),
            "DexkitBridge has already been closed and cannot be used"
        );
        handle
    }

    fn close(&self) {
        let handle = self.dexkit_handle.replace(std::ptr::null_mut());
        if !handle.is_null() {
            unsafe { dexkit_sys::dexkit_free(handle) };
        }
    }
}

impl Drop for BridgeHandle {
    fn drop(&mut self) {
        self.close();
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BridgeCore {
    handle: Arc<BridgeHandle>,
}

impl BridgeCore {
    fn from_raw_handle(handle: dexkit_sys::DexkitHandle) -> Self {
        Self {
            handle: Arc::new(BridgeHandle::new(handle)),
        }
    }

    pub(crate) fn new<S>(apk_path: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let dexkit_handle = unsafe { dexkit_sys::dexkit_new() };
        let c_apk_path = CString::new(apk_path.as_ref())?;
        let added =
            unsafe { dexkit_sys::dexkit_add_zip_path(dexkit_handle, c_apk_path.as_ptr(), 0) };
        if added == 0 {
            unsafe { dexkit_sys::dexkit_free(dexkit_handle) };
            return Err(Error::BridgeCreate("failed to add APK path"));
        }

        Ok(Self::from_raw_handle(dexkit_handle))
    }

    pub(crate) fn from_dex_bytes<B>(dex_bytes: B) -> Result<Self, Error>
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
            return Err(Error::BridgeCreate("failed to add dex bytes"));
        }

        Ok(Self::from_raw_handle(dexkit_handle))
    }

    pub(crate) fn from_dex_bytes_array<I, B>(dex_bytes_array: I) -> Result<Self, Error>
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
            return Err(Error::BridgeCreate("failed to add dex bytes array"));
        }

        Ok(Self::from_raw_handle(dexkit_handle))
    }

    fn handle(&self) -> dexkit_sys::DexkitHandle {
        self.handle.handle()
    }

    pub(crate) fn close(&self) {
        self.handle.close();
    }

    pub(crate) fn init_full_cache(&self) -> Result<(), Error> {
        let res = unsafe { dexkit_sys::dexkit_init_full_cache(self.handle()) };
        if res == 0 {
            return Err(Error::BridgeOperation("failed to initialize full cache"));
        }
        Ok(())
    }

    pub(crate) fn set_thread_num(&self, num_threads: i32) {
        unsafe { dexkit_sys::dexkit_set_thread_num(self.handle(), num_threads) };
    }

    pub(crate) fn set_max_concurrent_queries(&self, max_concurrent_queries: u32) {
        unsafe {
            dexkit_sys::dexkit_set_max_concurrent_queries(self.handle(), max_concurrent_queries)
        };
    }

    pub(crate) fn get_dex_num(&self) -> i32 {
        unsafe { dexkit_sys::dexkit_get_dex_num(self.handle()) }
    }

    pub(crate) fn export_dex_file(&self, output_path: &str) -> Result<(), Error> {
        let c_output_path = CString::new(output_path)?;
        let success =
            unsafe { dexkit_sys::dexkit_export_dex_file(self.handle(), c_output_path.as_ptr()) };
        if success == 0 {
            return Err(Error::BridgeOperation("failed to export DEX file"));
        }
        Ok(())
    }

    pub(crate) fn batch_find_class_using_strings(
        &self,
        batch_find: BatchFindClassUsingStrings,
    ) -> HashMap<String, ClassDataList> {
        unsafe {
            let mut buffer = batch_find.to_fb_bytes();
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = ClassDataList::from_batch_data(&bridge, data);
            dexkit_sys::dexkit_batch_find_class_using_strings_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn batch_find_method_using_strings(
        &self,
        batch_find: BatchFindMethodUsingStrings,
    ) -> HashMap<String, MethodDataList> {
        unsafe {
            let mut buffer = batch_find.to_fb_bytes();
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = MethodDataList::from_batch_data(&bridge, data);
            dexkit_sys::dexkit_batch_find_method_using_strings_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn find_class(&self, find_class: FindClass) -> ClassDataList {
        unsafe {
            let mut buffer = find_class.to_fb_bytes();
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = ClassDataList::from_data(&bridge, data);
            dexkit_sys::dexkit_find_class_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn find_method(&self, find_method: FindMethod) -> MethodDataList {
        unsafe {
            let mut buffer = find_method.to_fb_bytes();
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = MethodDataList::form_data(&bridge, data);
            dexkit_sys::dexkit_find_method_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn find_field(&self, find_field: FindField) -> FieldDataList {
        unsafe {
            let mut buffer = find_field.to_fb_bytes();
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = FieldDataList::form_data(&bridge, data);
            dexkit_sys::dexkit_find_field_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_class_data<T>(&self, identifier: T) -> Option<ClassData>
    where
        T: AsRef<str>,
    {
        let descriptor = identifier.as_ref();
        let descriptor = if descriptor.starts_with("L") && descriptor.ends_with(";") {
            descriptor
        } else {
            &format!("L{};", descriptor.replace('.', "/"))
        };

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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = ClassData::with_meta_raw(&bridge, data);
            dexkit_sys::dexkit_get_class_data_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_method_data<T>(&self, descriptor: T) -> Option<MethodData>
    where
        T: AsRef<str>,
    {
        let descriptor = descriptor.as_ref();
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = MethodData::from_meta_raw(&bridge, data);
            dexkit_sys::dexkit_get_method_data_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_field_data<T>(&self, descriptor: T) -> Option<FieldData>
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = FieldData::with_meta_raw(&bridge, data);
            dexkit_sys::dexkit_get_field_data_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_type_by_ids(&self, encode_id_array: &[i64]) -> ClassDataList {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = ClassDataList::from_data(&bridge, data);
            dexkit_sys::dexkit_get_class_by_ids_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_method_by_ids(&self, encode_id_array: &[i64]) -> MethodDataList {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = MethodDataList::form_data(&bridge, data);
            dexkit_sys::dexkit_get_method_by_ids_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_field_by_ids(&self, encode_id_array: &[i64]) -> FieldDataList {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = FieldDataList::form_data(&bridge, data);
            dexkit_sys::dexkit_get_field_by_ids_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_class_annotations(&self, class_id: i64) -> Vec<AnnotationData> {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = AnnotationData::with_annotation_meta_array_raw(&bridge, data);
            dexkit_sys::dexkit_get_class_annotations_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_field_annotations(&self, field_id: i64) -> Vec<AnnotationData> {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = AnnotationData::with_annotation_meta_array_raw(&bridge, data);
            dexkit_sys::dexkit_get_field_annotations_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn read_field_methods(&self, field_id: i64) -> MethodDataList {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = MethodDataList::form_data(&bridge, data);
            dexkit_sys::dexkit_field_get_methods_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn write_field_methods(&self, field_id: i64) -> MethodDataList {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = MethodDataList::form_data(&bridge, data);
            dexkit_sys::dexkit_field_put_methods_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_method_annotations(&self, method_id: i64) -> Vec<AnnotationData> {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = AnnotationData::with_annotation_meta_array_raw(&bridge, data);
            dexkit_sys::dexkit_get_method_annotations_free(&mut out_buf, out_len);
            result
        }
    }

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
                    let c_str = std::ffi::CStr::from_ptr(char_ptr);
                    match c_str.to_str() {
                        Ok(s) => names.push(Some(s.to_owned())),
                        Err(_) => names.push(None),
                    }
                }
            }

            dexkit_sys::dexkit_get_parameter_names_free(out_buf, out_len);
            Some(names)
        }
    }

    pub(crate) fn get_parameter_annotations(&self, method_id: i64) -> Vec<Vec<AnnotationData>> {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = AnnotationData::with_parameters_annotation_meta_array_raw(&bridge, data);
            dexkit_sys::dexkit_get_parameter_annotations_free(&mut out_buf, out_len);
            result
        }
    }

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
            dexkit_sys::dexkit_get_method_op_codes_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_call_methods(&self, method_id: i64) -> MethodDataList {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = MethodDataList::form_data(&bridge, data);
            dexkit_sys::dexkit_get_call_methods_free(&mut out_buf, out_len);
            result
        }
    }

    pub(crate) fn get_invoke_methods(&self, method_id: i64) -> MethodDataList {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = MethodDataList::form_data(&bridge, data);
            dexkit_sys::dexkit_get_invoke_methods_free(&mut out_buf, out_len);
            result
        }
    }

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
                }
                let c_str = std::ffi::CStr::from_ptr(char_ptr);
                if let Ok(s) = c_str.to_str() {
                    names.push(s.to_owned());
                }
            }

            dexkit_sys::dexkit_get_method_using_strings_free(out_buf, out_len);
            names
        }
    }

    pub(crate) fn get_method_using_fields(&self, method_id: i64) -> Vec<UsingFieldData> {
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

            let bridge = DexkitBridge::from_core(self.clone());
            let result = UsingFieldData::with_using_field_meta_array_raw(&bridge, data);
            dexkit_sys::dexkit_get_method_using_fields_free(&mut out_buf, out_len);
            result
        }
    }
}
