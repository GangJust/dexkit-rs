# AGENTS.md

本文件作用于仓库根目录及其所有子目录，除非更深层目录存在新的 `AGENTS.md` 覆盖本文件中的部分规则。

## 项目概览

- 这是一个为 [DexKit](https://github.com/LuckyPray/DexKit) 提供 Rust 封装的仓库。
- 顶层 Rust workspace 成员只有 `dexkit` 和 `dexkit-sys`。
- `example-desktop` 是独立的桌面示例 crate，不属于顶层 workspace。
- `example-android` 是 Android Gradle 工程，不是独立 Cargo workspace 成员。
- `dexkit-sys/external/DexKit` 是 Git 子模块，默认视为上游外部代码。

## 目录职责

- `dexkit/`: 高层 Rust API、类型封装、FlatBuffers 相关生成逻辑。
- `dexkit-sys/`: 底层 FFI 绑定、原生库构建与链接逻辑。
- `example-desktop/`: 桌面端运行示例。
- `example-android/`: Android 端示例。
- `target/`: 构建产物目录，不应手工修改。

## 修改规则

- 优先在最小范围内修改，避免无关重构。
- 若任务不明确要求，不要修改 `dexkit-sys/external/DexKit` 子模块中的内容。
- 不要提交或依赖 `target/` 下的构建产物。
- 若修改 public API、FFI 签名、构建脚本或示例用法，需同步检查受影响的示例是否仍然成立。
- 保持现有代码风格；仅在必要处补充简短注释，注释应解释原因或约束，而不是翻译代码。

## 构建与验证

- 本项目在 Windows 上依赖 `stable-x86_64-pc-windows-gnu` 工具链；不要默认假设 `msvc` 可直接替代。
- 若涉及 workspace 代码，优先验证：
  - `cargo +stable-x86_64-pc-windows-gnu check`
- 若涉及 `example-desktop`，可额外验证：
  - `cargo +stable-x86_64-pc-windows-gnu run`（在 `example-desktop/` 下执行）
- 若涉及 Android 示例，优先使用 Gradle 侧命令验证，避免臆测 Rust 侧单独可构建。
- 若构建依赖 `.env` 中的本机路径配置，例如 `MYSYS_LIB`，在结果中明确说明该前置条件，避免将本机环境问题误判为代码问题。

## 输出要求

- 汇报修改时，说明是否运行了验证命令；若未运行，明确原因。
- 若发现任务需要改动子模块、工具链配置或外部环境，先说明约束，再继续实施最小可行修改。

## 提交约定

- 若用户要求创建提交，提交信息优先采用类似 `type: summary` 的 Conventional Commits 风格。
- 提交前缀应根据实际变更类型判断，例如功能使用 `feat:`，修复使用 `fix:`，维护性调整或依赖更新可使用 `chore:`。
- 若用户未指定提交信息，应结合本次改动的目的选择最贴切的前缀与简短摘要。
