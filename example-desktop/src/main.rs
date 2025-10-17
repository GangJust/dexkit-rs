use dexkit::query::matchers::{ClassMatcher, MethodMatcher};
use dexkit::query::{FindClass, FindMethod};
use dexkit::{DexkitBridge, errors::Error};
use std::path::Path;

fn main() -> Result<(), Error> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_dir = Path::new(&cargo_manifest_dir);
    println!("Current dir: {}", cargo_manifest_dir.display());
    let apk_path = cargo_manifest_dir.join("apk").join("demo.apk");
    if !apk_path.exists() {
        return Err(Error::BridgeCreateError(format!(
            "apk not found: {}",
            apk_path.display()
        )));
    }
    println!("Using apk: {}", apk_path.display());
    let create_time = std::time::Instant::now();
    let bridge = DexkitBridge::create_apk_path(apk_path.to_str().unwrap())?;
    println!("[Rust] Create Bridge time: {:?}", create_time.elapsed());
    do_search(bridge);
    println!("[Rust] Find Use time: {:?}", create_time.elapsed());
    Ok(())
}

fn do_search(bridge: DexkitBridge) {
    let class_data_list = bridge.find_class(FindClass::create().set_matcher(
        ClassMatcher::create().set_class_name_str("io.github.cargo.ndk.plugin.MainActivity"),
    ));
    println!("\nCLASS:");
    for class_data in class_data_list.iter() {
        for annotation in class_data.annotations().iter() {
            println!("{:#?}", annotation);
        }
    }

    let method_data_list = class_data_list.find_method(
        FindMethod::create().set_matcher(MethodMatcher::create().set_method_name_str("test")),
    );
    println!("\nMETHOD:");
    for method_data in method_data_list.iter() {
        for annotation in method_data.annotations().iter() {
            println!("{:#?}", annotation);
        }
    }
}
