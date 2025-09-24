# Dexkit-rs

这是一个套娃了 [Dexkit](https://github.com/LuckyPray/DexKit) 的Rust仓库, 你可以很好的用Rust来操作Dexkit.

## 在桌面平台运行

首先参考 [Dexkit-docs|在桌面平台运行](https://luckypray.org/DexKit/zh-cn/guide/run-on-desktop.html)

rust 在 `windows_x64` 下默认为 `stable-x86_64-pc-windows-msvc` 编译环境

你需要切换到 `gnu` 工具链, 来运行 `example`

1、安装 gnu 工具链：

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
```

2、设置 gnu 为默认工具链：

```bash
rustup default stable-x86_64-pc-windows-gnu
```

3、运行

```bash
cargo run
```

或者，如果你不想改变默认工具链，你也可以在编译时明确指定 `gnu` 工具链：

```bash
cargo +stable-x86_64-pc-windows-gnu run
```

列出已安装的工具链

```bash
rustup toolchain list

# echo
# stable-x86_64-pc-windows-gnu (active, default)
# stable-x86_64-pc-windows-msvc
```

最后，你需要修改 [.env](/.env) 中的 `MYSYS_LIB`，以此来链接到 `stdc++` 和 `zlib`

到这里，你就可以愉快的`cd example & cargo run`了。

## 查询条件

基本完成封装

## 交叉编译

todo
