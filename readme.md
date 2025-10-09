# Dexkit-Rs

这是一个包装了 [Dexkit](https://github.com/LuckyPray/DexKit) 的 Rust 仓库, 它允许通过 Rust 来直接调用 Dexkit.

## 快速开始

> 构建本项目需要 `rust gnu` 工具链

1、安装 `gnu` 工具链：

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
```

2、设置 `gnu` 为默认工具链：

```bash
rustup default stable-x86_64-pc-windows-gnu
```

或者，如果不想改变默认工具链，也可以在编译时明确指定 `gnu` 工具链：

```bash
cargo +stable-x86_64-pc-windows-gnu run
```

列出已安装的工具链

```bash
rustup toolchain list

# stable-x86_64-pc-windows-gnu (active, default)
# stable-x86_64-pc-windows-msvc
```

## 在桌面平台运行

请首先参考 [Dexkit-docs|在桌面平台运行](https://luckypray.org/DexKit/zh-cn/guide/run-on-desktop.html) 完成基本设置

### Windows

- 修改 [.env](/.env) 中的 `MYSYS_LIB` 环境变量路径，以此来链接到 `stdc++` 和 `zlib` 随后分别执行以下两条命令

    ```bash
    cd example-desktop
    cargo run
    ```

### Linux

- 未测试

### Mac

- 未测试

## 在Android平台上运行

see: [example-andoird](/example-android/)
