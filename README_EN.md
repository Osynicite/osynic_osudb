<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="OsynicOsudb Logo"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsudb</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue" alt="Rust 1.85+"/></a>
  <a href="https://crates.io/crates/osynic_osudb" target="_blank"><img src="https://img.shields.io/crates/v/osynic_osudb" alt="Crates.io Version"/></a>
  <a href="https://docs.rs/osynic_osudb" target="_blank"><img src="https://img.shields.io/docsrs/osynic_osudb/0.1.3" alt="Docs.rs"/></a>
  <a href="https://github.com/osynicite/osynic_osudb" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg" alt="MIT License"/></a>
  <a href="https://discord.gg/JWyvc6M5" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg" alt="Discord"/></a>
  <a href="https://github.com/osynicite" target="_blank"><img src="https://img.shields.io/badge/support-sponsor-ff69b4.svg" alt="Sponsor"/></a>
</p>

<p align="center">
    🎵 <strong>High-Performance osu! Database Parser</strong> 🎵<br/>
    Refactored from osu-db with modern Rust architecture and latest version support
</p>

<hr />

**🌐 Languages:** [中文版本](README.md) | [English Version](README_EN.md)

# 📄 Introduction

**OsynicOsudb** is a high-performance osu! database parsing library designed for modern Rust applications. This project is a comprehensive refactor of the excellent [osu-db](https://crates.io/crates/osu-db) library, maintaining all original functionality while introducing significant optimizations and improvements.

## ✨ Key Features

- 🚀 **High-Performance Parsing**: Built on nom8 parser for fast and reliable data processing
- 🔧 **Read-Write Separation**: Independent read/write interfaces for better code maintainability
- 📊 **Complete Support**: Full support for osu!.db, collection.db, scores.db and all database files
- 🆕 **Latest Compatibility**: Fully supports osu! 2025.0107 version database format changes
- 🏗️ **Modern Architecture**: Redesigned entity structure following Rust best practices
- 📝 **Type Safety**: Strong type system ensures data integrity and runtime safety
- 🌐 **WASM Support**: Complete WebAssembly bindings for browser and Node.js environments

## 🎯 Use Cases

- osu! beatmap management tool development
- Game data analysis and statistics
- Batch beatmap information processing
- osu! ecosystem toolchain integration
- Web-based osu! tool development (via WASM)
- Node.js server-side data processing

## 📚 Official Documentation

For more information about osu! database file structure, please refer to: [Legacy Database File Structure](https://github.com/ppy/osu/wiki/Legacy-database-file-structure)

# 🌐 Related Projects Ecosystem

## Core Toolchain

### [osynic_serializer](https://github.com/osynicite/osynic_serializer)

An efficient osu! beatmap serialization tool with the following features:

- 💾 **Multiple Serialization Algorithms**: Supports both FOLDER and OSUDB serialization methods
- ⚡ **Fast Serialization**: Optimized algorithms ensure high performance when processing large beatmap collections
- 🔄 **Multi-Device Sync**: Works with osynic_downloader to achieve cross-device beatmap synchronization

![osynic_serializer Demo](https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif)

### [osynic_downloader](https://github.com/osynicite/osynic_downloader)

Smart beatmap downloader that supports:

- 📦 **Batch Downloads**: Efficient concurrent download mechanism
- 🔗 **Ecosystem Integration**: Seamless integration with the entire osynic toolchain

# 📦 Project Architecture

This project adopts a modular design with the following entity structure:

```text
📁 entity/
├── 🎵 osu/                    # osu! core data
│   ├── osudb.rs              # Main database file handling
│   ├── beatmap.rs            # Beatmap information entity
│   └── field/                # Field definitions
│       ├── grade.rs          # Grade levels
│       ├── mode.rs           # Game modes
│       ├── modification.rs   # Mods
│       ├── rank.rs           # Ranking information
│       ├── star.rs           # Star difficulty
│       └── time.rs           # Time-related fields
├── 📚 collection/             # Collection data
│   ├── collection.rs         # Single collection
│   └── collectiondb.rs       # Collection database
└── 🏆 scores/                 # Score data
    ├── scores.rs             # Score records
    ├── scoresdb.rs           # Score database
    └── field/                # Score-related fields
        ├── action.rs         # Action types
        ├── button.rs         # Button information
        └── replay.rs         # Replay data
```

# 🚀 Quick Start

## Installation

### Rust Projects

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
osynic_osudb = "0.1.3"
```

### Web/Node.js Projects

Install the WASM package via npm:

```bash
npm install @osynic/osynic-osudb
```

## Basic Usage

### Rust Environment

#### Parsing osu!.db File

```rust,no_run
use osynic_osudb::prelude::OsuDB;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load osu! database from file
    let mut osudb = OsuDB::from_file("osu!.db")?;
    
    println!("📊 Database Statistics:");
    println!("🎵 Total Beatmaps: {}", osudb.beatmaps.len());
    println!("👤 Player Name: {}", osudb.player_name.as_ref().unwrap_or(&"Unknown".to_string()));
    
    // Display detailed information for the first 3 beatmaps
    for (index, beatmap) in osudb.beatmaps.iter().take(3).enumerate() {
        println!("\n🎼 Beatmap #{}", index + 1);
        println!("   🎨 Artist: {}", beatmap.artist_unicode.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   📝 Title: {}", beatmap.title_unicode.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   👤 Creator: {}", beatmap.creator.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   ⭐ Difficulty: {}", beatmap.difficulty_name.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   🎵 Audio File: {}", beatmap.audio.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   🔑 MD5 Hash: {}", beatmap.hash.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   📁 File Name: {}", beatmap.file_name.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   📊 Status: {:?}", beatmap.status);
        
        // Beatmap statistics
        println!("   🎯 Statistics:");
        println!("      🔵 Hit Circles: {}", beatmap.hitcircle_count);
        println!("      🔗 Sliders: {}", beatmap.slider_count);
        println!("      🌀 Spinners: {}", beatmap.spinner_count);
        
        // Difficulty parameters
        println!("   ⚙️  Difficulty Settings:");
        println!("      📏 Approach Rate (AR): {:.1}", beatmap.approach_rate);
        println!("      🎯 Circle Size (CS): {:.1}", beatmap.circle_size);
        println!("      💔 HP Drain (HP): {:.1}", beatmap.hp_drain);
        println!("      🎚️  Overall Difficulty (OD): {:.1}", beatmap.overall_difficulty);
        
        println!("   ⏰ Last Modified: {}", beatmap.last_modified);
        println!("   {}", "─".repeat(50));
    }
    
    Ok(())
}
```

### Advanced Usage

```rust,no_run
use osynic_osudb::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let osudb = OsuDB::from_file("osu!.db")?;
    
    // Filter beatmaps by specific conditions
    let hard_beatmaps: Vec<_> = osudb.beatmaps
        .iter()
        .filter(|beatmap| beatmap.overall_difficulty > 5.0)
        .collect();
    
    println!("🔥 Hard Beatmaps (OD > 5.0): {} found", hard_beatmaps.len());
    
    // Group statistics by artist
    use std::collections::HashMap;
    let mut artist_count: HashMap<String, usize> = HashMap::new();
    
    for beatmap in &osudb.beatmaps {
        if let Some(artist) = &beatmap.artist_unicode {
            *artist_count.entry(artist.clone()).or_insert(0) += 1;
        }
    }
    
    // Display top 5 artists by beatmap count
    let mut sorted_artists: Vec<_> = artist_count.iter().collect();
    sorted_artists.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎨 Top Artists by Beatmap Count:");
    for (artist, count) in sorted_artists.iter().take(5) {
        println!("   {} - {} beatmaps", artist, count);
    }
    
    Ok(())
}
```

## Web/Node.js Environment

### Browser Usage

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>osu!db Parser</title>
</head>
<body>
    <input type="file" id="file-input" accept=".db" />
    <div id="output"></div>

    <script type="module">
        import init, { WasmOsuDB, WasmUtils } from '@osynic/osynic-osudb';

        async function run() {
            // Initialize WASM module
            await init();

            const fileInput = document.getElementById('file-input');
            const output = document.getElementById('output');

            fileInput.addEventListener('change', async (event) => {
                const file = event.target.files[0];
                if (!file) return;

                try {
                    // Read file as array buffer
                    const arrayBuffer = await file.arrayBuffer();
                    const bytes = new Uint8Array(arrayBuffer);

                    // Parse osu!.db file
                    const osuDB = new WasmOsuDB(bytes);

                    // Get basic info
                    output.innerHTML = `
                        <h3>osu!.db Info</h3>
                        <p>Version: ${osuDB.version}</p>
                        <p>Player Name: ${osuDB.playerName || 'Unknown'}</p>
                        <p>Beatmap Count: ${osuDB.beatmapCount()}</p>
                        <p>Folder Count: ${osuDB.folderCount}</p>
                    `;

                    // Get full data as JavaScript object
                    const data = osuDB.toObject();
                    console.log('Full osu!.db data:', data);

                    // Get beatmaps
                    const beatmaps = osuDB.getBeatmaps();
                    console.log('Beatmaps:', beatmaps);

                } catch (error) {
                    output.innerHTML = `<p style="color: red;">Error: ${error.message}</p>`;
                    console.error('File parsing error:', error);
                }
            });

            // Show library info
            const constants = WasmUtils.getVersionConstants();
            console.log('Library version info:', constants);
            console.log('Has compression support:', WasmUtils.hasCompression());
        }

        run();
    </script>
</body>
</html>
```

### Node.js Usage

```javascript
import { readFileSync } from 'fs';
import init, { WasmOsuDB, WasmScoresDB, WasmCollectionDB, WasmReplay } from '@osynic/osynic-osudb';

async function parseOsuDB() {
    // Initialize WASM module
    await init();

    try {
        // Parse osu!.db
        const osuDbBytes = readFileSync('path/to/osu!.db');
        const osuDB = new WasmOsuDB(osuDbBytes);
        
        console.log(`Player: ${osuDB.playerName}`);
        console.log(`Beatmaps: ${osuDB.beatmapCount()}`);
        
        const data = osuDB.toObject();
        console.log('Full data:', JSON.stringify(data, null, 2));

        // Parse scores.db
        const scoresDbBytes = readFileSync('path/to/scores.db');
        const scoresDB = new WasmScoresDB(scoresDbBytes);
        console.log(`Scores DB version: ${scoresDB.version}`);

        // Parse collection.db
        const collectionDbBytes = readFileSync('path/to/collection.db');
        const collectionDB = new WasmCollectionDB(collectionDbBytes);
        console.log(`Collections: ${collectionDB.collectionCount()}`);

        // Parse replay file
        const replayBytes = readFileSync('path/to/replay.osr');
        const replay = new WasmReplay(replayBytes);
        console.log(`Replay player: ${replay.playerName}`);
        console.log(`Score: ${replay.score}`);

    } catch (error) {
        console.error('Error:', error.message);
    }
}

parseOsuDB();
```

### TypeScript Support

```typescript
import { readFileSync } from 'fs';
import init, { 
    WasmOsuDB, 
    WasmScoresDB, 
    WasmCollectionDB, 
    WasmReplay,
    OsuDBData,
    BeatmapData
} from '@osynic/osynic-osudb';

async function parseWithTypes(): Promise<void> {
    await init();

    const osuDbBytes = readFileSync('path/to/osu!.db');
    const osuDB = new WasmOsuDB(osuDbBytes);
    
    // Get typed data
    const data: OsuDBData = osuDB.toObject();
    const beatmaps: BeatmapData[] = osuDB.getBeatmaps();
    
    // Process beatmaps with full type safety
    beatmaps.forEach((beatmap: BeatmapData) => {
        console.log(`${beatmap.artist_unicode || beatmap.artist_ascii} - ${beatmap.title_unicode || beatmap.title_ascii}`);
        console.log(`Difficulty: ${beatmap.difficulty_name}`);
        console.log(`Creator: ${beatmap.creator}`);
        console.log(`AR: ${beatmap.approach_rate}, CS: ${beatmap.circle_size}`);
        console.log('---');
    });
}
```

### React Example

```tsx
import React, { useState, useCallback } from 'react';
import init, { WasmOsuDB } from '@osynic/osynic-osudb';

const OsuDBViewer: React.FC = () => {
    const [osuDB, setOsuDB] = useState<WasmOsuDB | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleFileChange = useCallback(async (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (!file) return;

        setLoading(true);
        setError(null);

        try {
            // Initialize WASM module if not already done
            await init();

            const arrayBuffer = await file.arrayBuffer();
            const bytes = new Uint8Array(arrayBuffer);
            
            const db = new WasmOsuDB(bytes);
            setOsuDB(db);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Unknown error');
        } finally {
            setLoading(false);
        }
    }, []);

    return (
        <div>
            <input
                type="file"
                accept=".db"
                onChange={handleFileChange}
                disabled={loading}
            />
            
            {loading && <p>Loading...</p>}
            {error && <p style={{ color: 'red' }}>Error: {error}</p>}
            
            {osuDB && (
                <div>
                    <h3>osu!.db Information</h3>
                    <p>Player: {osuDB.playerName || 'Unknown'}</p>
                    <p>Version: {osuDB.version}</p>
                    <p>Beatmaps: {osuDB.beatmapCount()}</p>
                    
                    <h4>Beatmaps</h4>
                    <ul>
                        {osuDB.getBeatmaps().slice(0, 10).map((beatmap, index) => (
                            <li key={index}>
                                {beatmap.artist_unicode || beatmap.artist_ascii} - {beatmap.title_unicode || beatmap.title_ascii}
                                [{beatmap.difficulty_name}]
                            </li>
                        ))}
                    </ul>
                </div>
            )}
        </div>
    );
};

export default OsuDBViewer;
```

## Performance Tips

1. **Initialize once**: Call `init()` only once in your application
2. **Reuse instances**: WASM objects can be reused for multiple operations
3. **Memory management**: WASM objects are automatically garbage collected
4. **Large files**: For very large files, consider processing in chunks

## Error Handling

The WASM bindings provide detailed error messages for parsing failures:

```javascript
try {
    const osuDB = new WasmOsuDB(invalidBytes);
} catch (error) {
    if (error.message.includes('Failed to parse OsuDB')) {
        console.log('Invalid osu!.db file format');
    }
}
```
    sorted_artists.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎨 Top Artists by Beatmap Count:");
    for (artist, count) in sorted_artists.iter().take(5) {
        println!("   {} - {} beatmaps", artist, count);
    }
    
    Ok(())
}
```

# ❤️ Acknowledgments & Credits

## Open Source Spirit

This project's core implementation is based on the excellent open source project [`osu-db`](https://crates.io/crates/osu-db). We have made the following improvements:

- 🏗️ **Architecture Refactoring**: Redesigned project structure for better code maintainability
- 🔄 **Read-Write Separation**: Independent read/write interfaces following modern software design principles
- ⚡ **Performance Optimization**: Upgraded to nom8 parser for enhanced parsing performance
- 🆕 **Version Compatibility**: Resolved osu! 2025.0107 version database format changes
- 📚 **API Improvements**: Provided more friendly and intuitive Rust APIs

## License Information

- **This Project**: Open source under [MIT License](LICENSE)
- **Original osu-db Project**: Open source under [Unlicense](http://unlicense.org) (license file preserved in `licenses/` directory)

We thank all developers who contribute to the open source community! 🙏

# 🤝 Contributing

We warmly welcome community contributions! This project retains all core functionality of `osu-db`, but currently focuses primarily on the stability and performance optimization of OsuDB parsing functionality.

## 🚀 How to Contribute

### Report Issues

- 🐛 Found a bug? Please submit a detailed [Issue](https://github.com/osynicite/osynic_osudb/issues)
- 💡 Have new ideas? Welcome to share in [Discussions](https://github.com/osynicite/osynic_osudb/discussions)

### Code Contributions

**Basic Workflow:**

- **Fork this repository**
- **Create a feature branch**: `git checkout -b feature/amazing-feature`
- **Follow coding standards**:

  ```bash
  # Run development checks
  make quick          # Quick checks (compile + format + clippy)
  make pre-commit     # Pre-commit checks (includes tests)
  
  # Or run individually
  cargo fmt --all     # Code formatting
  cargo clippy --all-features -- -D warnings  # Code linting
  cargo test --all-features  # Run tests
  ```

- **Commit changes**: `git commit -m 'Add some amazing feature'`
- **Push branch**: `git push origin feature/amazing-feature`
- **Create Pull Request**

## 📋 Contribution Guidelines

- ✅ **Code Quality**: Follow official Rust coding standards
- 🧪 **Test Coverage**: New features must include corresponding test cases
- 📝 **Documentation Updates**: Important changes need to update relevant documentation
- 🔧 **Compatibility**: Ensure backward compatibility unless there's a major version update

## 🎯 Areas Currently Needing Help

- 📊 **Other Database Support**: Complete testing for collection.db and scores.db
- 🔧 **Performance Optimization**: Large file parsing performance improvements
- 📚 **Documentation Enhancement**: More usage examples and best practices
- 🌐 **Internationalization**: Multi-language error message support

Thank you for considering contributing to OsynicOsudb! Every PR and Issue is a valuable contribution to the project. ✨

# 📜 License

This project is licensed under the **[MIT License](LICENSE)**, which means you are free to:

- ✅ **Commercial Use**: Use this library in commercial projects
- ✅ **Modify & Distribute**: Modify the code and distribute your versions
- ✅ **Private Use**: Use in personal projects
- ✅ **Patent Grant**: Receive patent rights for related patents

**Usage Conditions:**

- 📄 Retain original license and copyright notices
- 📝 Indicate changes made in modified versions

---

<p align="center">
  <strong>⭐ If this project helps you, please give us a Star! ⭐</strong><br/>
  <em>Your support is our motivation for continuous improvement 💪</em>
</p>

<p align="center">
  <a href="https://github.com/osynicite/osynic_osudb">🌟 GitHub Repository</a> |
  <a href="https://crates.io/crates/osynic_osudb">📦 Crates.io</a> |
  <a href="https://docs.rs/osynic_osudb">📚 Documentation</a> |
  <a href="https://discord.gg/JWyvc6M5">💬 Discord</a>
</p>
