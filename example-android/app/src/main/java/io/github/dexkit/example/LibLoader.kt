package io.github.dexkit.example

object LibLoader {
    init {
        System.loadLibrary("example_android")
    }

    external fun load(apkPath: String): Boolean
}