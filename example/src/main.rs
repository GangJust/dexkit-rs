use dexkit::{
    DexkitBridge,
    errors::Error,
    query::{
        BatchFindClassUsingStrings, BatchFindMethodUsingStrings, FindClass, FindField, FindMethod,
        enums::StringMatchType,
        matchers::{ClassMatcher, FieldMatcher, StringMatchersGroup, base::StringMatcher},
    },
    uitls::Modifier,
};
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
        FieldMatcher::create().set_modifiers(Modifier::PUBLIC | Modifier::STATIC),
    ));
    println!("[Rust] Found public static fields: {:#?}", list.len());
    // for ele in list.iter() {
    //     println!("modifiers: {:?}", Modifier::from_bits(ele.modifiers()));
    // }

    let class_data_list = bridge.find_class(
        FindClass::create().set_find_first(true).set_matcher(
            ClassMatcher::create().set_class_name_matcher(
                StringMatcher::create()
                    .set_value("io/github/cargo/ndk/plugin/MainActivity")
                    .set_match_type(StringMatchType::Equals),
            ),
        ),
    );
    println!(
        "[Rust] Found class with super class MainActivity: {:#?}",
        class_data_list.len()
    );

    let first = class_data_list.first().unwrap();
    // println!("[Rust] First class: {:#?}", first);

    let supper_class = first.supper_class();
    // println!("[Rust] Found supper class: {:#?}", supper_class);

    let interfaces = supper_class.clone().unwrap().interfaces();
    println!("[Rust] Found interfaces: {:#?}", interfaces.len());
    // for ele in interfaces.iter() {
    //     println!("interface descriptor: {:?}", ele.descriptor());
    // }

    // let annotations = supper_class.clone().unwrap().annotations();
    // std::fs::write("output.txt", format!("{:?}", annotations)).unwrap();

    // let fields = supper_class.clone().unwrap().fields();
    // println!("[Rust] Found fields: {:#?}", fields.len());

    // for field in fields.iter() {
    //     // println!("[Rust] Field: {:#?}", field.descriptor());
    //     // println!("[Rust] First field: {:#?}", field.readers());
    //     // println!("[Rust] First field: {:#?}", field.writers());
    // }

    // let methods = supper_class.unwrap().methods();
    // println!("[Rust] Found methods: {:#?}", methods.len());

    // for method in methods.iter() {
    //     // println!("[Rust] Found method: {:#?}", method.descriptor());
    //     // println!("[Rust] Found method: {:#?}", method.annotations());
    //     // println!("[Rust] Found method: {:#?}", method.using_fields());
    // }

    // let cls_data = bridge.get_class_data("io/github/cargo/ndk/plugin/MainActivity");
    // println!("[Rust] Get class data: {:#?}", cls_data);

    // let method_data = bridge.get_method_data("Landroidx/activity/ComponentActivity;->startActivityForResult(Landroid/content/Intent;ILandroid/os/Bundle;)V");
    // println!("[Rust] Get method data: {:#?}", method_data);

    // let field_data = bridge.get_filed_data("Landroidx/activity/ComponentActivity;->ACTIVITY_RESULT_TAG:Ljava/lang/String;");
    // println!("[Rust] Get field data: {:#?}", field_data);

    let find_class_groups = bridge.batch_find_class_using_strings(
        BatchFindClassUsingStrings::create()
            .add_group(
                StringMatchersGroup::create("group1")
                    .add_string_matcher_str("M")
                    .add_string_matcher_str("A"),
            )
            .add_group(
                StringMatchersGroup::create("group2")
                    .add_string_matcher_str("N")
                    .add_string_matcher_str("B"),
            ),
    );
    println!(
        "[Rust] Batch find classes using strings: group1.len = {:#?}, group2.len = {:#?}",
        find_class_groups["group1"].len(),
        find_class_groups["group2"].len()
    );

    let find_method_groups = bridge.batch_find_method_using_strings(
        BatchFindMethodUsingStrings::create()
            .add_group(
                StringMatchersGroup::create("group1")
                    .add_string_matcher_str("M")
                    .add_string_matcher_str("A"),
            )
            .add_group(
                StringMatchersGroup::create("group2")
                    .add_string_matcher_str("N")
                    .add_string_matcher_str("B"),
            ),
    );
    println!(
        "[Rust] Batch find methods using strings: group1.len = {:#?}, group2.len = {:#?}",
        find_method_groups["group1"].len(),
        find_method_groups["group2"].len()
    );
}
