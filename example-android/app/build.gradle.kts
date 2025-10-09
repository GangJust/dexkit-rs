import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    id("io.github.thatworld.cargondk") version "0.0.9"
}

cargoNdk {
    release = true
    ndkVersion = "28.2.13676358"
    abiFilters = setOf("arm64-v8a"/* , "armeabi-v7a", "x86", "x86_64" */)
    environment["RUSTFLAGS"] += listOf(
        "--remap-path-prefix ${project.rootDir}=",
        "--remap-path-prefix ${System.getenv("CARGO_HOME")}=",
        "--remap-path-prefix ${System.getenv("RUSTUP_HOME")}=",
    )
}

android {
    namespace = "io.github.dexkit.example"
    compileSdk = 36

    defaultConfig {
        applicationId = "io.github.dexkit.example"
        minSdk = 24
        targetSdk = 36
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }
    kotlin {
        compilerOptions {
            jvmTarget = JvmTarget.JVM_21
        }
    }
}

dependencies {
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.appcompat)
    implementation(libs.material)
    implementation(libs.androidx.activity)
    implementation(libs.androidx.constraintlayout)
}