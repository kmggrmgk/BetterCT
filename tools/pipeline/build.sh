#!/bin/bash
# Pipeline 项目构建和测试脚本

echo "=== Pipeline 项目构建脚本 ==="

# 检查 Rust 是否安装
if ! command -v rustc &> /dev/null; then
    echo "错误: 未找到 Rust 编译器，请先安装 Rust"
    echo "可以通过以下命令安装: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust 环境检查通过"

# 清理之前的构建
echo "清理旧构建..."
cargo clean

# 检查依赖
echo "检查依赖..."
cargo check

if [ $? -ne 0 ]; then
    echo "错误: 依赖检查失败"
    exit 1
fi

echo "✓ 依赖检查通过"

# 构建项目
echo "构建项目..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "错误: 构建失败"
    exit 1
fi

echo "✓ 项目构建成功"

# 运行测试
echo "运行测试..."
cargo test

if [ $? -ne 0 ]; then
    echo "警告: 部分测试可能失败"
else
    echo "✓ 所有测试通过"
fi

# 运行示例
echo "运行使用示例..."
cargo run --example usage_example

echo "=== 构建完成 ==="
echo "可执行文件位置: target/release/pipeline"
echo "使用方法: ./target/release/pipeline --help"