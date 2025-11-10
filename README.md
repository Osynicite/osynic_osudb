<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img width="128" height="128" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsudb</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_osudb" target="_blank"><img src="https://img.shields.io/crates/v/osynic_osudb"/></a>
  <a href="https://docs.rs/osynic_osudb" target="_blank"><img src="https://img.shields.io/docsrs/osynic_osudb/0.1.4"/></a>
  <a href="https://www.npmjs.com/package/@osynicite/osynic-osudb" target="_blank"><img src="https://img.shields.io/npm/v/@osynicite/osynic-osudb"/></a>
  <a href="https://www.npmjs.com/package/@osynicite/osynic-osudb" target="_blank"><img src="https://img.shields.io/npm/dm/@osynicite/osynic-osudb"/></a>
  <a href="https://hakochest.github.io/osynic-osudb/" target="_blank"><img src="https://img.shields.io/badge/Typedoc-Documentation-blue"/></a>
  <a href="https://osynic-osudb.deno.dev" target="_blank"><img src="https://img.shields.io/badge/Deno-white?logo=deno&logoColor=black"/></a>
  <a href="https://github.com/osynicite/osynic_osudb" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
  <a href="https://discord.gg/DRnZSES3BC" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
  <a href="https://github.com/osynicite" target="_blank"><img src="https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square"/></a>
</p>

<p align="center">
    🚀 高性能 · 🏗️ 结构优良 · 🔧 WASM 支持 <br/>
    功能完整的 osu! 数据库解析库，支持解析 osu!.db, collection.db 与 scores.db 。
</p>

<p align="center">
  <a href="README.md">🇨🇳 中文</a> ·
  <a href="README_EN.md">🇺🇸 English</a>
</p>

# 📄 简介

高性能的 osu! 数据库解析库，基于优秀的 [osu-db](https://crates.io/crates/osu-db) 库重构。支持 osu!.db、collection.db、scores.db 等所有数据库文件，完全兼容 osu! 2025+ 版本。

# ✨ 特性

- 🚀 基于 nom8 解析器，高性能解析
- 📦 支持所有 osu! 数据库文件格式
- 🌐 完整的 WASM 支持（浏览器和 Node.js）
- 🏗️ 读写分离设计，易于维护
- 🔒 强类型系统确保数据安全

# 📦 安装

## Rust

```toml
[dependencies]
osynic_osudb = "0.1.4"
```

# 🚀 快速开始

```rust,no_run
use osynic_osudb::entity::osu::osudb::OsuDB;

fn main() {
    let mut osudb = OsuDB::from_file("osu!.db").unwrap();

    for (index, beatmap) in osudb.beatmaps.iter_mut().take(3).enumerate() {
        println!(
            "Song {}: {} - {}",
            index + 1,
            beatmap.artist_unicode.as_ref().unwrap(),
            beatmap.title_unicode.as_ref().unwrap()
        );
        println!("Creator: {}", beatmap.creator.as_ref().unwrap());
        println!("Difficulty: {}", beatmap.difficulty_name.as_ref().unwrap());
        println!("Audio: {}", beatmap.audio.as_ref().unwrap());
        println!("Hash: {}", beatmap.hash.as_ref().unwrap());
        println!("File Name: {}", beatmap.file_name.as_ref().unwrap());
        println!("Status: {:?}", beatmap.status);
        println!("Hitcircle Count: {}", beatmap.hitcircle_count);
        println!("Slider Count: {}", beatmap.slider_count);
        println!("Spinner Count: {}", beatmap.spinner_count);
        println!("Last Modified: {}", beatmap.last_modified);
        println!("Approach Rate: {}", beatmap.approach_rate);
        println!("Circle Size: {}", beatmap.circle_size);
        println!("HP Drain: {}", beatmap.hp_drain);
        println!("Overall Difficulty: {}", beatmap.overall_difficulty);
        println!("---------------------------------");
    }
}

```

🎯 更多示例：查看 `examples/` 目录获取完整示例，或运行 `cargo run --example 示例名` 查看实际效果。

# ❤️ 鸣谢

本项目基于 [osu-db](https://crates.io/crates/osu-db) 库重构，进行了架构优化、性能提升和版本兼容性改进。

感谢 [osu-db](https://crates.io/crates/osu-db) 的作者们！

`osu-db` 项目基于 [Unlicense](licenses/LICENSE-osu-db)，项目证书放置在 `licenses/` 目录。

# 🤝 贡献指南

## 如何贡献

欢迎提交 PR 或 Issue！如果您发现任何问题或有改进建议，请遵循以下规则：

### 代码贡献规范

- **编码规范**：遵循 Rust 官方编码规范
- **测试要求**：新增功能需附带测试用例  
- **代码质量**：提交前运行 `cargo fmt` 和 `cargo clippy`
- **文档更新**：必要时更新相关文档和示例

### Issue 提交指南

- 描述问题的具体场景
- 提供复现步骤和错误信息
- 附上相关的 API 端点和参数信息

# 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。
