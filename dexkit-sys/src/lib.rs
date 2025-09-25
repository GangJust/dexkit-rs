use std::ffi::{c_char, c_int, c_void};

pub type DexkitHandle = *mut c_void;

#[link(name = "dexkit_wrapper", kind = "static")]
unsafe extern "C" {
    pub fn dexkit_new() -> DexkitHandle;

    pub fn dexkit_free(handle: DexkitHandle);

    pub fn dexkit_add_zip_path(
        handle: DexkitHandle,
        apk_path: *mut c_char,
        unzip_thread_num: c_int,
    ) -> c_int;

    pub fn dexkit_set_thread_num(handle: DexkitHandle, num_threads: c_int);

    pub fn dexkit_init_full_cache(handle: DexkitHandle) -> bool;

    pub fn dexkit_get_dex_num(handle: DexkitHandle) -> c_int;

    pub fn dexkit_export_dex_file(handle: DexkitHandle, output_path: *mut c_char) -> bool;

    pub fn dexkit_find_class(
        handle: *mut c_void,
        buffer: *mut c_void,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_find_class_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_find_method(
        handle: *mut c_void,
        buffer: *mut c_void,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_find_method_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_find_field(
        handle: *mut c_void,
        buffer: *mut c_void,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_find_field_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_class_by_ids(
        handle: *mut c_void,
        encode_id_array: *mut c_void,
        ids_len: usize,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_class_by_ids_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_method_by_ids(
        handle: *mut c_void,
        encode_id_array: *mut c_void,
        ids_len: usize,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_method_by_ids_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_field_by_ids(
        handle: *mut c_void,
        encode_id_array: *mut c_void,
        ids_len: usize,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_field_by_ids_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_class_annotations(
        handle: *mut c_void,
        class_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_class_annotations_free(out_buf: *mut *mut c_void, out_len: usize);

    pub fn dexkit_get_field_annotations(
        handle: *mut c_void,
        field_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_field_annotations_free(out_buf: *mut *mut c_void, out_len: usize);

    pub fn dexkit_field_get_methods(
        handle: *mut c_void,
        field_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_field_get_methods_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_field_put_methods(
        handle: *mut c_void,
        field_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_field_put_methods_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_parameter_names(
        handle: *mut c_void,
        method_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_parameter_names_free(buf: *mut *mut c_char, len: usize);

    pub fn dexkit_get_method_annotations(
        handle: *mut c_void,
        method_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_method_annotations_free(out_buf: *mut *mut c_void, out_len: usize);

    pub fn dexkit_get_parameter_annotations(
        handle: *mut c_void,
        method_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_parameter_annotations_free(out_buf: *mut *mut c_void, out_len: usize);

    pub fn dexkit_get_method_op_codes(
        handle: *mut c_void,
        method_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_method_op_codes_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_call_methods(
        handle: *mut c_void,
        method_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_call_methods_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_invoke_methods(
        handle: *mut c_void,
        method_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_invoke_methods_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_method_using_strings(
        handle: *mut c_void,
        string_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_method_using_strings_free(out_buf: *mut *mut c_char, len: usize);

    pub fn dexkit_get_method_using_fields(
        handle: *mut c_void,
        method_id: i64,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_method_using_fields_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_class_data(
        handle: *mut c_void,
        dex_descriptor: *mut c_char,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_class_data_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_method_data(
        handle: *mut c_void,
        dex_descriptor: *mut c_char,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_method_data_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_get_field_data(
        handle: *mut c_void,
        dex_descriptor: *mut c_char,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_get_field_data_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_batch_find_class_using_strings(
        handle: *mut c_void,
        buffer: *mut c_void,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_batch_find_class_using_strings_free(out_buf: *mut *mut c_void, len: usize);

    pub fn dexkit_batch_find_method_using_strings(
        handle: *mut c_void,
        buffer: *mut c_void,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
    pub fn dexkit_batch_find_method_using_strings_free(out_buf: *mut *mut c_void, len: usize);
}
