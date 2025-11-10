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
    🚀 High Performance · 🏗️ Well-Architected · 🔧 WASM Support <br/>
    A feature-complete osu! database parser library that supports parsing osu!.db, collection.db, and scores.db.
</p>

<p align="center">
  <a href="README.md">🇨🇳 Chinese</a> ·
  <a href="README_EN.md">🇺🇸 English</a>
</p>

# 📄 Introduction

A high-performance osu! database parser library, reconstructed based on the excellent [osu-db](https://crates.io/crates/osu-db) library. Supports all database file formats including osu!.db, collection.db, scores.db, and more, with full compatibility for osu! 2025+ versions.

# ✨ Features

- 🚀 High-performance parsing based on nom8 parser
- 📦 Support for all osu! database file formats
- 🌐 Complete WASM support (browser and Node.js)
- 🏗️ Read-write separation design for easy maintenance
- 🔒 Strong type system ensures data safety

# 📦 Installation

## Rust

```toml
[dependencies]
osynic_osudb = "0.1.4"
```

## JavaScript/TypeScript (via WASM)

```bash
npm install @osynicite/osynic-osudb
```

or

```bash
yarn add @osynicite/osynic-osudb
```

# 🚀 Quick Start

## Rust Example

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

## JavaScript/TypeScript Example

```javascript
import { OsuDB } from '@osynicite/osynic-osudb';

async function main() {
    const response = await fetch('osu!.db');
    const buffer = await response.arrayBuffer();
    
    const osudb = OsuDB.from_buffer(new Uint8Array(buffer));
    
    osudb.beatmaps.slice(0, 3).forEach((beatmap, index) => {
        console.log(`Song ${index + 1}: ${beatmap.artist_unicode} - ${beatmap.title_unicode}`);
        console.log(`Creator: ${beatmap.creator}`);
        console.log(`Difficulty: ${beatmap.difficulty_name}`);
        console.log(`Hash: ${beatmap.hash}`);
        console.log('---------------------------------');
    });
}

main();
```

🎯 More examples: Check the `examples/` directory for complete examples, or run `cargo run --example example_name` to see the actual effect.

# 📚 Documentation

- [Rust API Documentation](https://docs.rs/osynic_osudb)
- [TypeScript/JavaScript Documentation](https://hakochest.github.io/osynic-osudb/)
- [Deno Documentation](https://osynic-osudb.deno.dev)

# ❤️ Acknowledgments

This project has been reconstructed based on the [osu-db](https://crates.io/crates/osu-db) library with architectural optimizations, performance improvements, and version compatibility enhancements.

Special thanks to the authors of [osu-db](https://crates.io/crates/osu-db)!

The `osu-db` project is based on [Unlicense](licenses/LICENSE-osu-db), and the project licenses are placed in the `licenses/` directory.

# 🤝 Contribution Guidelines

## How to Contribute

We welcome PRs and Issues! If you discover any issues or have improvement suggestions, please follow these guidelines:

### Code Contribution Standards

- **Coding Standards**: Follow [Rust official coding standards](https://doc.rust-lang.org/1.0.0/style/)
- **Testing Requirements**: New features must include test cases
- **Code Quality**: Run `cargo fmt` and `cargo clippy` before submitting
- **Documentation Updates**: Update relevant documentation and examples as needed

### Issue Submission Guidelines

- Describe the specific scenario of the problem
- Provide reproduction steps and error messages
- Include relevant API endpoints and parameters if applicable

## Development Setup

<!-- markdownlint-disable MD029 -->
1. Clone the repository:

```bash
git clone https://github.com/osynicite/osynic_osudb.git
cd osynic_osudb
```

2. Build the project:

```bash
cargo build
```

3. Run tests:

```bash
cargo test
```

4. Format code:

```bash
cargo fmt
```

5. Lint with clippy:

```bash
cargo clippy --all-targets --all-features
```
<!-- markdownlint-enable MD029 -->

# 📜 License

This project is open source under the [MIT License](LICENSE). Please respect the original author's copyright.

---

<!-- markdownlint-disable MD036 -->
**Made with ❤️ by the Osynicite team**
<!-- markdownlint-enable MD036 -->

For support, join us on [Discord](https://discord.gg/DRnZSES3BC)
