use cmake::Config;
use std::env;

fn main() {
    cmake_build();
}

fn cmake_build() {
    println!("Starting dexkit-wrapper build...");

    #[cfg(target_os = "windows")] // Specify the path to the MSYS2 mingw64 libraries
    {
        if let Err(e) = dotenv::dotenv() {
            eprintln!("Failed to read .env file: {}", e);
        }
        let mysys_lib = env::var("MYSYS_LIB")
            .expect("MYSYS_LIB must be set in .env file or environment variables");

        println!("cargo:rustc-link-search=native={}", mysys_lib);
        println!("cargo:rustc-link-lib=static=stdc++"); // Link to the C++ standard library
        println!("cargo:rustc-link-lib=static=z"); // Link to the zlib library
    }

    // todo: other os...

    // Build the C++ project using CMake
    let dst = Config::new("external/dexkit-wrapper")
        .generator("Ninja")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_CXX_FLAGS_RELEASE", "-O3 -DNDEBUG")
        .define("CMAKE_C_FLAGS_RELEASE", "-O3 -DNDEBUG")
        .build();
    println!("cargo:rustc-link-search=native={}/libs", dst.display());
    println!("cargo:rustc-link-lib=static=dexkit_static");
    println!("cargo:rustc-link-lib=static=dexkit_wrapper");

    println!("dexkit-wrapper build finsh.");
}
