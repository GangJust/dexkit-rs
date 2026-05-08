use std::collections::BTreeSet;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let result = flatc_exec();
    match result {
        Ok(_) => println!("FlatBuffers code generation succeeded."),
        Err(e) => eprintln!("{}", e),
    }
}

fn get_flatc_path() -> PathBuf {
    let mut path = current_dir().unwrap();
    path.push("flatc");
    if cfg!(target_os = "windows") {
        path.set_extension("exe");
    }
    path
}

fn get_flatc_version() -> Result<String, String> {
    let output = Command::new(get_flatc_path())
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to execute flatc: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "flatc returned a non-zero exit code: {}",
            output.status
        ));
    }

    let version = String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in flatc version output: {}", e))?;
    Ok(version.trim().to_string())
}

fn flatc_exec() -> Result<(), String> {
    let flatc_path = get_flatc_path();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", flatc_path.display());
    println!("Using flatc at: {}", flatc_path.display());

    let flatc_version = get_flatc_version()?;
    println!("flatc version: {}", flatc_version);

    let schema_path = "../dexkit-sys/external/DexKit/schema/fbs";
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let fbs = [
        format!("{}/encode_value.fbs", schema_path),
        format!("{}/enums.fbs", schema_path),
        format!("{}/matchers.fbs", schema_path),
        format!("{}/querys.fbs", schema_path),
        format!("{}/ranges.fbs", schema_path),
        format!("{}/results.fbs", schema_path),
    ];
    for item in &fbs {
        println!("cargo:rerun-if-changed={item}");
    }

    // Bug: `{out_path}/mod.rs` 尽管被生成了，但没有包含所有的子模块, 所以这里单独处理
    //
    // let output_path = Path::new(&out_dir).join("flatbuffers");
    // let output = Command::new(flatc_path)
    //     .args([
    //         "--rust",
    //         "--rust-module-root-file",
    //         "--gen-all",
    //         "-o",
    //         output_path.to_str().unwrap(),
    //     ])
    //     .args(&fbs)
    //     .output();
    // println!("{:?}", output);

    // 单独处理每个 fbs 文件
    let flatbuffers_generate_path = Path::new(&out_dir).join("flatbuffers_generate");
    let output_path = Path::new(&out_dir).join("flatbuffers/dexkit/schema");

    if output_path.exists() {
        fs::remove_dir_all(&output_path)
            .map_err(|e| format!("Failed to remove existing output directory: {}", e))?;
    }
    fs::create_dir_all(&output_path)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    // 对每个 fbs 文件单独生成代码
    for item in fbs {
        let output = Command::new(&flatc_path)
            .args([
                "--rust",
                "--rust-module-root-file",
                "--gen-all",
                "-o",
                &flatbuffers_generate_path.to_str().unwrap(),
            ])
            .arg(&item)
            .output()
            .map_err(|e| format!("Failed to execute flatc for {}: {}", item, e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "flatc failed for {} with status {}: {}",
                item,
                output.status,
                stderr.trim()
            ));
        }
    }

    // 遍历生成目录：
    // 1. 复制 flatc 原始输出到最终 schema 目录，保留与上游一一对应的名字
    // 2. 收集所有 public 类型，后续自动生成 FB* 别名导出层
    let mut mod_set = BTreeSet::<String>::new();
    let mut alias_set = BTreeSet::<String>::new();
    let generate_path = flatbuffers_generate_path.join("dexkit/schema"); // flatbuffers_generate/dexkit/schema
    let generate_dir =
        fs::read_dir(&generate_path).map_err(|e| format!("Failed to read directory: {}", e))?;
    for entry in generate_dir {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap(); // 获取文件名（不含扩展名）
        let new_entry = output_path.join(file_name).with_extension("rs"); // 目标路径
        fs::copy(&path, &new_entry).map_err(|e| format!("Failed to copy file: {}", e))?; // 复制文件到最终目录
        collect_pub_item_names(&path, &mut alias_set)
            .map_err(|e| format!("Failed to collect public item names: {}", e))?;
        mod_set.insert(file_name.to_string()); // 添加到集合
    }
    fs::remove_dir_all(&flatbuffers_generate_path)
        .map_err(|e| format!("Failed to remove temporary output directory: {}", e))?; // 删除临时目录(含所有子文件)

    // 生成 mod.rs 文件：
    // - schema: 仅 crate 内部可见，作为 flatc 原始镜像层，便于和上游 schema 对照
    // - fb: 对外公开的别名层，只导出 FB* 名字，避免和业务实体名称冲突
    let mut mod_content = String::new();
    mod_content.push_str("// @generated by build.rs\n");
    mod_content.push_str("pub mod dexkit {\n");
    mod_content.push_str("  pub(crate) mod schema {\n");
    mod_content.push_str("    use super::*;\n");
    for item in &mod_set {
        mod_content.push_str(&format!("    mod {};\n", item));
        mod_content.push_str(&format!("    pub use self::{}::*;\n", item));
    }
    mod_content.push_str("  } // schema\n");
    mod_content.push_str("  pub mod fb {\n");
    for item in &alias_set {
        mod_content.push_str(&format!(
            "    pub use super::schema::{} as FB{};\n",
            item, item
        ));
    }
    mod_content.push_str("  } // fb\n");
    mod_content.push_str("} // dexkit\n");

    let mod_file_path = output_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("mod.rs");
    fs::write(&mod_file_path, mod_content)
        .map_err(|e| format!("Failed to write mod.rs file: {}", e))?;

    Ok(())
}

fn collect_pub_item_names(path: &Path, out: &mut BTreeSet<String>) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    for line in content.lines() {
        // 只为 flatc 生成出来的 public 类型建立 FB* 别名。
        // 这样上游新增 Args / Builder / Offset / Union 等类型时，别名层会自动同步。
        let trimmed = line.trim_start();
        let Some(rest) = trimmed
            .strip_prefix("pub struct ")
            .or_else(|| trimmed.strip_prefix("pub enum "))
            .or_else(|| trimmed.strip_prefix("pub type "))
            .or_else(|| trimmed.strip_prefix("pub union "))
        else {
            continue;
        };

        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() {
            continue;
        }
        out.insert(name);
    }
    Ok(())
}
