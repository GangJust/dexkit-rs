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

    pub fn dexkit_find_method(
        handle: *mut c_void,
        buffer: *mut c_void,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );

    pub fn dexkit_find_field(
        handle: *mut c_void,
        buffer: *mut c_void,
        out_buf: *mut *mut c_void,
        out_len: *mut usize,
    );
}
