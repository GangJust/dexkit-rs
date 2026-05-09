use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=external/dexkit-wrapper/CMakeLists.txt");
    println!("cargo:rerun-if-changed=external/dexkit-wrapper/dexkit_wrapper.cpp");
    println!("cargo:rerun-if-changed=external/dexkit-wrapper/dexkit_wrapper.h");

    // get build target
    let target = env::var("TARGET").unwrap();
    let is_android = target.contains("android");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir_path = PathBuf::from(manifest_dir);
    let mut config = Config::new("external/dexkit-wrapper");

    // android configuration
    if is_android {
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=z");

        let target_arch = get_target_arch_abi(&target);
        let toolchain = env::var("CARGO_NDK_CMAKE_TOOLCHAIN_PATH")
            .expect("CARGO_NDK_CMAKE_TOOLCHAIN_PATH must be set for Android builds");

        // custom libcxx library path.
        // link prefab cxx library
        // let cxx_libs_dir = manifest_dir_path.join(r"external\cxxlib\libs");
        // let cxx_libs_path = PathBuf::from(cxx_libs_dir).join(format!("android.{}", target_arch));
        // println!("cargo:rustc-link-search=native={}", cxx_libs_path.display());
        // println!("cargo:rustc-link-lib=static=cxx");
        // config
        //     .define("CMAKE_FIND_ROOT_PATH", r"<your prefab build root path>") // Afterwards, you can `find_package(cxx REQUIRED CONFIG)` & `link_libraries(cxx::cxx)` in CMakeLists.txt, more: https://google.github.io/prefab/example-workflow.html
        //     .define("ANDROID_STL", "none");

        // or local cxx library
        let cxx_include_path = manifest_dir_path.join(r"external\cxxlib\include");
        let cxx_libs_dir = manifest_dir_path.join(r"external\cxxlib\libs");
        let cxx_libs_path = PathBuf::from(cxx_libs_dir).join(format!("android.{}", target_arch));
        println!("cargo:rustc-link-search=native={}", cxx_libs_path.display());
        println!("cargo:rustc-link-lib=static=cxx");
        config
            .cflag(&format!("-I{}", cxx_include_path.display()))
            .cxxflag(&format!("-I{}", cxx_include_path.display()))
            .define("ANDROID_STL", "none");

        // common android
        config
            .cflag("-std=c18")
            .cxxflag("-std=c++20")
            .cxxflag("-funwind-tables")
            .cxxflag("-fasynchronous-unwind-tables")
            .cxxflag("-Qunused-arguments")
            .cxxflag("-fno-rtti")
            .cxxflag("-fno-exceptions")
            .cxxflag("-fvisibility=hidden")
            .cxxflag("-fvisibility-inlines-hidden")
            .cxxflag("-Wno-unused-value")
            .cxxflag("-Wno-unused-variable")
            .cxxflag("-Wno-unused-command-line-argument")
            .define("CMAKE_SYSTEM_NAME", "Android")
            .define("ANDROID_ABI", &target_arch)
            .define("CMAKE_ANDROID_ARCH_ABI", &target_arch)
            .define("CMAKE_TOOLCHAIN_FILE", &toolchain);
    }

    // common configuration
    config
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_CXX_FLAGS_RELEASE", "-O3 -DNDEBUG")
        .define("CMAKE_C_FLAGS_RELEASE", "-O3 -DNDEBUG")
        .build_target("all") //
        .generator("Ninja");

    let dst = config.build();
    let core_dst = dst.join(r"build\Core");
    println!("cargo:rustc-link-search=native={}", core_dst.display());
    println!("cargo:rustc-link-lib=static=dexkit_static");
    let wrapper_dst = dst.join(r"build");
    println!("cargo:rustc-link-search=native={}", wrapper_dst.display());
    println!("cargo:rustc-link-lib=static=dexkit_wrapper");

    // windows host environment configuration
    #[cfg(target_os = "windows")]
    {
        if !is_android {
            if let Err(e) = dotenv::dotenv() {
                println!("cargo:err=Failed to read .env file: {}", e);
                std::process::exit(1);
            }
            let mysys_lib = env::var("MYSYS_LIB")
                .expect("MYSYS_LIB must be set in .env file or environment variables");

            println!("cargo:rustc-link-search=native={}", mysys_lib);
            println!("cargo:rustc-link-lib=static=stdc++");
            println!("cargo:rustc-link-lib=static=z");
        }
    }

    // linux host environment configuration
    #[cfg(target_os = "linux")]
    {
        if !is_android {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=dylib=z");
        }
    }

    // macos host environment configuration
    #[cfg(target_os = "macos")]
    {
        if !is_android {
            println!("cargo:rustc-link-lib=dylib=c++");
            println!("cargo:rustc-link-lib=dylib=z");
        }
    }
}

fn get_target_arch_abi(target: &str) -> &str {
    if target.contains("aarch64") || target.contains("arm64") {
        "arm64-v8a"
    } else if target.contains("arm") {
        "armeabi-v7a"
    } else if target.contains("x86_64") {
        "x86_64"
    } else if target.contains("i686") || target.contains("i386") {
        "x86"
    } else {
        panic!("Unsupported target architecture: {}", target);
    }
}
