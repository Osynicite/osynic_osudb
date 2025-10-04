我来为你详细讲解使用 wasm-pack 构建和发布 WASM 包到 npm 时需要注意的重要参数！

## 核心构建参数

### 1. 目标平台 (`--target`)
```bash
# 针对不同的 JavaScript 环境
wasm-pack build --target web          # 浏览器环境 (ES modules)
wasm-pack build --target bundler      # 打包工具 (webpack, rollup 等)
wasm-pack build --target nodejs       # Node.js 环境
wasm-pack build --target no-modules   # 无模块系统的浏览器
```

### 2. 输出目录 (`--out-dir`)
```bash
wasm-pack build --out-dir pkg-web      # 指定输出目录
wasm-pack build --out-dir dist/wasm    # 自定义构建输出路径
```

### 3. 作用域和包名 (`--scope`)
```bash
wasm-pack build --scope myorg          # 发布为 @myorg/package-name
```

## 优化相关参数

### 4. 发布模式 (`--release`)
```bash
wasm-pack build --release              # 生产环境优化构建
wasm-pack build --dev                  # 开发模式 (默认)
wasm-pack build --profiling            # 性能分析模式
```

### 5. 优化级别
在 `Cargo.toml` 中配置：
```toml
[profile.release]
opt-level = "s"          # 优化大小
lto = true              # 链接时优化
codegen-units = 1       # 减少代码分割
panic = "abort"         # 减小体积
```

## TypeScript 支持

### 6. 生成 TypeScript 声明
```bash
wasm-pack build --target bundler --out-dir pkg
# 自动生成 .d.ts 文件
```

## 包管理配置

### 7. package.json 自定义
在项目根目录创建 `package.json` 模板：
```json
{
  "name": "my-wasm-package",
  "version": "0.1.0",
  "description": "My WebAssembly package",
  "main": "my_wasm_package.js",
  "types": "my_wasm_package.d.ts",
  "files": [
    "my_wasm_package_bg.wasm",
    "my_wasm_package.js",
    "my_wasm_package.d.ts"
  ],
  "keywords": ["wasm", "webassembly"],
  "repository": "https://github.com/username/repo"
}
```

## 发布流程

### 8. 完整的发布命令示例
```bash
# 1. 构建
wasm-pack build --release --target nodejs --out-dir pkg --scope osynic
wasm-pack build --release --target web --out-dir pkg --scope osynic
wasm-pack build --release --target bundler --out-dir pkg --scope osynic

# 2. 测试包
wasm-pack test --headless --firefox

# 3. 发布到 npm
wasm-pack publish --access public
# 或者手动发布
cd pkg && npm publish --access public
```

## 重要注意事项

### 9. Cargo.toml 配置
```toml
[package]
name = "my-wasm-package"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # 必须指定为 cdylib

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"          # JavaScript 绑定
web-sys = "0.3"         # Web API 绑定

[dependencies.web-sys]
version = "0.3"
features = [
  "console",
  "Document",
  "Element",
  # 根据需要添加更多 Web API
]
```

### 10. 常用完整构建脚本
```bash
#!/bin/bash
# build.sh

# 清理之前的构建
rm -rf pkg/

# 构建不同目标
wasm-pack build --release --target web --out-dir pkg-web
wasm-pack build --release --target bundler --out-dir pkg-bundler
wasm-pack build --release --target nodejs --out-dir pkg-node

echo "Build completed for all targets!"
```

### 11. 文件大小优化技巧
```bash
# 使用 wee_alloc 减小体积
wasm-pack build --release --target bundler -- --features wee_alloc

# 在 Cargo.toml 中
[dependencies]
wee_alloc = "0.4.5"

[features]
default = ["wee_alloc"]
```

### 12. 调试和测试参数
```bash
# 生成调试信息
wasm-pack build --dev --target bundler

# 运行测试
wasm-pack test --headless --chrome
wasm-pack test --headless --firefox --features test-feature
```

## 发布前检查清单

✅ 确认 `Cargo.toml` 中的包信息正确  
✅ 测试在目标环境中正常工作  
✅ 检查生成的 `.wasm` 文件大小  
✅ 验证 TypeScript 声明文件  
✅ 确认 `package.json` 元数据  
✅ 测试 npm 包的导入和使用

这些参数和配置将帮助你成功构建和发布高质量的 WASM 包到 npm！有任何特定的使用场景需要详细说明吗？
