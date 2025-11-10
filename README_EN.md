<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img width="128" height="128" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo"/>
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
    🚀 High Performance · 🏗️ Well-Structured · 🔧 WASM Support <br/>
    Feature-complete osu! database parser library supporting osu!.db, collection.db and scores.db parsing.
</p>

<p align="center">
  <a href="README.md">🇨🇳 中文</a> ·
  <a href="README_EN.md">🇺🇸 English</a>
</p>

# 📄 Introduction

A high-performance osu! database parser library rebuilt on the excellent [osu-db](https://crates.io/crates/osu-db) library. Supports all database files including osu!.db, collection.db, scores.db, with full compatibility for osu! 2025+ versions.

# ✨ Features

- 🚀 High-performance parsing based on nom8 parser
- 📦 Support for all osu! database file formats
- 🌐 Complete WASM support (Browser and Node.js)
- 🏗️ Read-write separation design for easy maintenance
- 🔒 Strong type system ensures data safety

# 📦 Installation

## Rust

```toml
[dependencies]
osynic_osudb = "0.1.4"
```

## JavaScript/TypeScript (WASM)

```bash
npm install @osynicite/osynic-osudb
```

## Deno

```typescript
import * as OsuDB from "https://osynic-osudb.deno.dev/mod.ts";
```

# 🚀 Quick Start

## Rust Example

```rust
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

## JavaScript/TypeScript Example

```typescript
import { OsuDB } from "@osynicite/osynic-osudb";

async function main() {
    const data = await fetch("osu!.db").then(r => r.arrayBuffer());
    const osudb = new OsuDB(new Uint8Array(data));
    
    for (let i = 0; i < Math.min(3, osudb.beatmaps.length); i++) {
        const beatmap = osudb.beatmaps[i];
        console.log(`Song ${i + 1}: ${beatmap.artist_unicode} - ${beatmap.title_unicode}`);
        console.log(`Creator: ${beatmap.creator}`);
        console.log(`Difficulty: ${beatmap.difficulty_name}`);
    }
}

main();
```

🎯 **More Examples**: Check the `examples/` directory for complete examples, or run `cargo run --example example_name` to see them in action.

# 📚 Documentation

- **Rust Documentation**: Available on [docs.rs](https://docs.rs/osynic_osudb)
- **TypeScript Documentation**: Available on [Typedoc](https://hakochest.github.io/osynic-osudb/)
- **Deno Module**: Available on [deno.dev](https://osynic-osudb.deno.dev)

# ❤️ Acknowledgments

This project is rebuilt on the basis of the excellent [osu-db](https://crates.io/crates/osu-db) library with architectural optimizations, performance improvements, and version compatibility enhancements.

Thanks to the authors of [osu-db](https://crates.io/crates/osu-db)!

The `osu-db` project is based on [Unlicense](licenses/LICENSE-osu-db). The project license is placed in the `licenses/` directory.

# 🤝 Contributing Guide

## How to Contribute

Welcome to submit PRs or Issues! If you find any problems or have improvement suggestions, please follow these guidelines:

### Code Contribution Standards

- **Coding Standards**: Follow the official Rust coding standards
- **Test Requirements**: New features must include test cases
- **Code Quality**: Run `cargo fmt` and `cargo clippy` before submitting
- **Documentation Updates**: Update related documentation and examples when necessary

### Issue Submission Guidelines

- Describe the specific scenario of the problem
- Provide reproduction steps and error messages
- Include relevant API endpoints and parameter information
- Attach osu!.db file samples if applicable (if the issue is related to database parsing)

# 📜 Open Source License

This project is open-sourced under the [MIT License](LICENSE). Please respect the original author's copyright.

# 🔗 Related Links

- [osu! Official Website](https://osu.ppy.sh/)
- [osu-db (Original Library)](https://crates.io/crates/osu-db)
- [Discord Community](https://discord.gg/DRnZSES3BC)

---

**If you find this project helpful, please give it a ⭐ Star!**
