use dexkit::{
    DexkitBridge,
    errors::Error,
    query::{FindClass, FindField, FindMethod, matchers::FieldMatcher},
    uitls::Modifier,
};

fn main() -> Result<(), Error> {
    let current_dir = std::env::current_dir().unwrap();
    println!("Current dir: {}", current_dir.display());
    let apk_path = current_dir.join("apk").join("demo.apk");
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
    let class_data_list = bridge.find_class(FindClass::create());
    println!("[Rust] Found classes: {:#?}", class_data_list.len());
    // for ele in class_data_list.iter() {
    //     println!("descriptor: {:?}", ele.descriptor());
    // }

    let method_data_list = bridge.find_method(FindMethod::create());
    println!("[Rust] Found methods: {:#?}", method_data_list.len());
    // for ele in method_data_list.iter() {
    //     println!("name: {:?}", ele.name());
    // }

    let field_data_list = bridge.find_field(FindField::create());
    println!("[Rust] Found fields: {:#?}", field_data_list.len());
    // for ele in field_data_list.iter() {
    //     println!("name: {:?}", ele.descriptor());
    // }

    let list = bridge.find_field(FindField::create().set_matcher(
        FieldMatcher::create().set_modifiers_u32(Modifier::PUBLIC | Modifier::STATIC),
    ));
    println!("[Rust] Found public static fields: {:#?}", list.len());
    // for ele in list.iter() {
    //     println!("modifiers: {:?}", Modifier::from_bits(ele.modifiers()));
    // }
}
