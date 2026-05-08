use std::ffi::c_void;

use android_logcat::Log;
use dexkit::{
    errors::Error,
    query::{matchers::StringMatchersGroup, BatchFindMethodUsingStrings},
    DexkitBridge,
};
use jni::{
    objects::{JObject, JString},
    sys::{jboolean, jint, JavaVM, JNI_VERSION_1_6},
    JNIEnv,
};

/// JNI OnLoad
#[unsafe(no_mangle)]
pub extern "C" fn JNI_OnLoad(_vm: JavaVM, _: *mut c_void) -> jint {
    Log::init("DexkitRs", false); // Initialize logging
    JNI_VERSION_1_6
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_io_github_dexkit_example_LibLoader_load<'a>(
    mut env: JNIEnv,
    _thiz: JObject<'a>,
    apk_path: JString,
) -> jboolean {
    let apk_path: String = env
        .get_string(&apk_path)
        .expect("Couldn't get java string!")
        .into();

    match do_search(apk_path) {
        Ok(_) => jni::sys::JNI_TRUE,
        Err(_) => jni::sys::JNI_FALSE,
    }
}

fn do_search(apk_path: String) -> Result<(), Error> {
    let bridge = DexkitBridge::new(apk_path)?;

    let result = bridge.batch_find_method_using_strings(
        BatchFindMethodUsingStrings::create()
            .add_group(StringMatchersGroup::create("group1").add_string_matcher_str("John")),
    );

    Log::d(format!("Result: {:#?}", result));

    Ok(())
}
