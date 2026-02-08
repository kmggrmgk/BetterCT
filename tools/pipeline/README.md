# **PIPELINE (使用 Rust 仿写 Git)**

一个基于 Rust 开发的 Git 风格多媒体资产管理管道系统，专门用于处理纹理、3D 模型、音频等二进制资源。

## **特性**

- 📦 **内容寻址存储**: 基于 BLAKE3 哈希的内容寻址存储引擎
- 🔍 **差异化算法**: 高效的二进制数据差异计算和补丁应用
- 🎨 **多格式支持**: 支持纹理、3D 模型、音频等多种媒体格式
- 📊 **依赖图管理**: 基于有向图的资源依赖关系管理
- 🔄 **版本控制**: Git 风格的分支和合并功能
- ⚡ **并行处理**: 利用 Rayon 进行并行数据处理
- 🛠️ **命令行工具**: 完整的 CLI 接口

## **项目结构**
```
pipeline/
|-- src/
|   |-- storage/                # 内容寻址存储引擎
|       |-- content_store.rs    # 对象存储
|       |-- diff_engine.rs      # 差异化算法
|   |-- format/                 # 格式解析器
|       |-- texture.rs          # 图片格式处理
|       |-- model3d.rs          # 3D 模型处理
|       |-- audio.rs            # 音频处理
|   |-- graph/                  # 资源关系图
|       |-- dependency.rs       # 依赖分析
|       |-- versioning.rs       # 版本管理
|   |-- cli/                    # 命令行接口
|   |-- lib.rs                  # 库入口
|   |-- main.rs                 # 主程序入口
|-- tests/                      # 集成测试
|-- examples/                   # 使用示例
|-- assets/                     # 测试资产
|-- Cargo.toml                  # 项目配置
```

## **安装与构建**

```bash
# 克隆项目
git clone <repository-url>
cd pipeline

# 构建项目
cargo build --release

# 运行测试
cargo test

# 运行示例
cargo run --example usage_example
```

## **快速开始**

### 初始化仓库
```bash
pipeline init --path ./my-project
```

### 存储资源
```bash
# 存储纹理文件
pipeline store --path ./textures/logo.png --resource-type texture

# 存储3D模型
pipeline store --path ./models/character.gltf --resource-type model3d

# 存储音频文件
pipeline store --path ./audio/background.mp3 --resource-type audio
```

### 版本控制
```bash
# 提交更改
pipeline commit --message "Add initial assets" --author "Your Name"

# 查看历史
pipeline log

# 创建分支
pipeline branch feature/new-textures --from main

# 合并分支
pipeline merge feature/new-textures main
```

### 资源管理
```bash
# 检索资源
pipeline retrieve abc123def456 --output ./output/texture.png

# 查看依赖关系
pipeline graph --hash abc123def456

# 比较资源差异
pipeline diff hash1 hash2
```

## **API 使用示例**

```rust
use pipeline::{
    storage::{ContentStore, ObjectType},
    graph::{DependencyGraph, VersionManager},
    ResourceMetadata, ResourceType
};
use std::collections::HashMap;

// 创建内容存储
let mut store = ContentStore::new("./.pipeline")?;

// 存储资源
let data = std::fs::read("./texture.png")?;
let hash = store.store_object(ObjectType::Blob, data)?;

// 构建依赖图
let mut graph = DependencyGraph::new();
let metadata = ResourceMetadata {
    hash,
    resource_type: ResourceType::Texture,
    size: data.len() as u64,
    create_at: chrono::Utc::now().timestamp(),
    dependencies: vec!["texture:1024x1024".to_string()],
};
graph.add_resource(metadata);

// 版本管理
let mut version_manager = VersionManager::new();
let mut resources = HashMap::new();
resources.insert("my_texture".to_string(), metadata);

let version_id = version_manager.create_version(
    vec!["initial".to_string()],
    "Add texture resource".to_string(),
    resources,
    "author_name",
)?;
```

## **测试**

项目包含全面的单元测试和集成测试：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_content_store_basic_operations

# 运行文档测试
cargo test --doc
```

## **性能特点**

- **高效哈希**: 使用 BLAKE3 算法确保快速且安全的哈希计算
- **并行处理**: 利用 Rayon 实现多线程并行数据处理
- **增量存储**: 差异化算法减少存储空间占用
- **内存优化**: 智能缓存策略平衡内存使用和访问速度

## **贡献指南**

欢迎提交 Issue 和 Pull Request！

### 开发环境设置
```bash
# 安装开发依赖
cargo install cargo-watch

# 监听文件变化并自动测试
cargo watch -x test
```

## **许可证**

MIT License

---

*该项目旨在为游戏开发、CG 制作和其他多媒体项目提供高效的资源管理解决方案。*