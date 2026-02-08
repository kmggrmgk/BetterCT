@echo off
REM Pipeline 项目构建和测试脚本 (Windows)

echo === Pipeline 项目构建脚本 ===

REM 检查 Rust 是否安装
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 错误: 未找到 Rust 编译器，请先安装 Rust
    echo 可以通过以下方式安装: 访问 https://www.rust-lang.org/tools/install
    exit /b 1
)

echo ✓ Rust 环境检查通过

REM 清理之前的构建
echo 清理旧构建...
cargo clean

REM 检查依赖
echo 检查依赖...
cargo check

if %errorlevel% neq 0 (
    echo 错误: 依赖检查失败
    exit /b 1
)

echo ✓ 依赖检查通过

REM 构建项目
echo 构建项目...
cargo build --release

if %errorlevel% neq 0 (
    echo 错误: 构建失败
    exit /b 1
)

echo ✓ 项目构建成功

REM 运行测试
echo 运行测试...
cargo test

if %errorlevel% neq 0 (
    echo 警告: 部分测试可能失败
) else (
    echo ✓ 所有测试通过
)

REM 运行示例
echo 运行使用示例...
cargo run --example usage_example

echo === 构建完成 ===
echo 可执行文件位置: target\release\pipeline.exe
echo 使用方法: target\release\pipeline.exe --help

pause