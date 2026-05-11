use dexkit::query::matchers::{ClassMatcher, MethodMatcher, StringMatcher};
use dexkit::query::{FindClass, FindMethod};
use dexkit::{DexkitBridge, Error};
use std::path::Path;

fn main() -> Result<(), Error> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_dir = Path::new(&cargo_manifest_dir);
    println!("Current dir: {}", cargo_manifest_dir.display());
    let apk_path = cargo_manifest_dir.join("apk").join("demo.apk");
    if !apk_path.exists() {
        return Err(Error::InvalidInput(format!("apk not found: {}", apk_path.display())));
    }
    println!("Using apk: {}", apk_path.display());
    let create_time = std::time::Instant::now();
    let apk_path = apk_path
        .to_str()
        .ok_or_else(|| Error::InvalidInput(format!("invalid apk path: {}", apk_path.display())))?;
    let bridge = DexkitBridge::new(apk_path)?;
    println!("[Rust] Create Bridge time: {:?}", create_time.elapsed());
    do_search(bridge);
    println!("[Rust] Find Use time: {:?}", create_time.elapsed());
    Ok(())
}

fn do_search(bridge: DexkitBridge) {
    let class_data_list = bridge.find_class(FindClass::new().matcher(
        ClassMatcher::new()
            .class_name(StringMatcher::equals("io.github.cargo.ndk.plugin.MainActivity")),
    ));
    println!("\nCLASS:");
    for class_data in class_data_list.iter() {
        for annotation in class_data.annotations().iter() {
            println!("{:#?}", annotation);
        }
    }

    let method_data_list = class_data_list.find_method(
        FindMethod::new().matcher(MethodMatcher::new().name(StringMatcher::contains("test"))),
    );
    println!("\nMETHOD:");
    for method_data in method_data_list.iter() {
        for annotation in method_data.annotations().iter() {
            println!("{:#?}", annotation);
        }
    }
}
