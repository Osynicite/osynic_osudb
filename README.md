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
    🎵 <strong>高性能的 osu! 数据库解析库</strong> 🎵<br/>
    基于 osu-db 重构，支持最新版本，提供现代化的 Rust API
</p>

<hr />

**🌐 语言版本：** [中文版本](README.md) | [English Version](README_EN.md)

# 📄 项目简介

**OsynicOsudb** 是一个高性能的 osu! 数据库解析库，专为现代 Rust 应用程序设计。本项目基于优秀的 [osu-db](https://crates.io/crates/osu-db) 库重构而来，在保持原有功能的基础上进行了大量优化和改进。

## ✨ 主要特性

- 🚀 **高性能解析**：基于 nom8 解析器，提供快速可靠的数据解析
- 🔧 **读写分离**：独立的读写接口设计，提高代码可维护性
- 📊 **完整支持**：支持 osu!.db、collection.db、scores.db 等所有数据库文件
- 🆕 **最新兼容**：完全支持 osu! 2025.0107 版本的数据库格式变更
- 🏗️ **现代架构**：重新设计的实体结构，更符合 Rust 最佳实践
- 📝 **类型安全**：强类型系统确保数据完整性和运行时安全

## 🎯 适用场景

- osu! 谱面管理工具开发
- 游戏数据分析和统计
- 谱面信息批量处理
- osu! 生态系统工具链集成

## 📚 官方文档

更多关于 osu! 数据库文件结构的信息，请参考：[Legacy Database File Structure](https://github.com/ppy/osu/wiki/Legacy-database-file-structure)

# 🌐 相关项目生态

## 核心工具链

### [osynic_serializer](https://github.com/osynicite/osynic_serializer)

高效的 osu! 谱面序列化工具，提供以下特性：

- 💾 **多种序列化算法**：支持 FOLDER 和 OSUDB 两种序列化方式
- ⚡ **快速序列化**：优化的算法确保处理大量谱面时的高性能
- 🔄 **多设备同步**：与 osynic_downloader 配合实现跨设备谱面同步

![osynic_serializer演示](https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif)

### [osynic_downloader](https://github.com/osynicite/osynic_downloader)

智能谱面下载器，支持：

- 📦 **批量下载**：高效的并发下载机制
- 🔗 **生态集成**：与整个 osynic 工具链无缝集成

# 📦 项目架构

本项目采用模块化设计，主要包含以下实体结构：

```text
📁 entity/
├── 🎵 osu/                    # osu! 核心数据
│   ├── osudb.rs              # 主数据库文件处理
│   ├── beatmap.rs            # 谱面信息实体
│   └── field/                # 字段定义
│       ├── grade.rs          # 成绩等级
│       ├── mode.rs           # 游戏模式
│       ├── modification.rs   # 模组(Mods)
│       ├── rank.rs           # 排名信息
│       ├── star.rs           # 星级难度
│       └── time.rs           # 时间相关
├── 📚 collection/             # 收藏夹数据
│   ├── collection.rs         # 单个收藏夹
│   └── collectiondb.rs       # 收藏夹数据库
└── 🏆 scores/                 # 成绩数据
    ├── scores.rs             # 成绩记录
    ├── scoresdb.rs           # 成绩数据库
    └── field/                # 成绩相关字段
        ├── action.rs         # 操作类型
        ├── button.rs         # 按键信息
        └── replay.rs         # 回放数据
```

# 🚀 快速开始

## 安装

在您的 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
osynic_osudb = "0.1.3"
```

## 基础用法

### 解析 osu!.db 文件

```rust,no_run
use osynic_osudb::prelude::OsuDB;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从文件加载 osu! 数据库
    let mut osudb = OsuDB::from_file("osu!.db")?;
    
    println!("📊 数据库统计信息:");
    println!("🎵 总谱面数: {}", osudb.beatmaps.len());
    println!("👤 玩家名称: {}", osudb.player_name.as_ref().unwrap_or(&"未知".to_string()));
    
    // 遍历并显示前 3 个谱面的详细信息
    for (index, beatmap) in osudb.beatmaps.iter().take(3).enumerate() {
        println!("\n🎼 谱面 #{}", index + 1);
        println!("   🎨 艺术家: {}", beatmap.artist_unicode.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   📝 标题: {}", beatmap.title_unicode.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   👤 制作者: {}", beatmap.creator.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   ⭐ 难度: {}", beatmap.difficulty_name.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   🎵 音频文件: {}", beatmap.audio.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   🔑 MD5 哈希: {}", beatmap.hash.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   📁 文件名: {}", beatmap.file_name.as_ref().unwrap_or(&"N/A".to_string()));
        println!("   📊 状态: {:?}", beatmap.status);
        
        // 谱面统计信息
        println!("   🎯 统计信息:");
        println!("      🔵 圆圈数: {}", beatmap.hitcircle_count);
        println!("      🔗 滑条数: {}", beatmap.slider_count);
        println!("      🌀 转盘数: {}", beatmap.spinner_count);
        
        // 难度参数
        println!("   ⚙️  难度参数:");
        println!("      📏 缩圈速度 (AR): {:.1}", beatmap.approach_rate);
        println!("      🎯 圆圈大小 (CS): {:.1}", beatmap.circle_size);
        println!("      💔 掉血速度 (HP): {:.1}", beatmap.hp_drain);
        println!("      🎚️  总体难度 (OD): {:.1}", beatmap.overall_difficulty);
        
        println!("   ⏰ 最后修改: {}", beatmap.last_modified);
        println!("   {}", "─".repeat(50));
    }
    
    Ok(())
}
```

### 进阶用法

```rust,no_run
use osynic_osudb::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let osudb = OsuDB::from_file("osu!.db")?;
    
    // 筛选特定条件的谱面
    let hard_beatmaps: Vec<_> = osudb.beatmaps
        .iter()
        .filter(|beatmap| beatmap.overall_difficulty > 5.0)
        .collect();
    
    println!("🔥 高难度谱面 (OD > 5.0): {} 个", hard_beatmaps.len());
    
    // 按艺术家分组统计
    use std::collections::HashMap;
    let mut artist_count: HashMap<String, usize> = HashMap::new();
    
    for beatmap in &osudb.beatmaps {
        if let Some(artist) = &beatmap.artist_unicode {
            *artist_count.entry(artist.clone()).or_insert(0) += 1;
        }
    }
    
    // 显示谱面数最多的前 5 个艺术家
    let mut sorted_artists: Vec<_> = artist_count.iter().collect();
    sorted_artists.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎨 谱面数量最多的艺术家:");
    for (artist, count) in sorted_artists.iter().take(5) {
        println!("   {} - {} 个谱面", artist, count);
    }
    
    Ok(())
}
```

# ❤️ 致谢与声明

## 开源精神

本项目的核心实现基于优秀的开源项目 [`osu-db`](https://crates.io/crates/osu-db)。我们在此基础上进行了以下改进：

- 🏗️ **架构重构**：重新设计项目结构，提高代码可维护性
- 🔄 **读写分离**：独立的读写接口，符合现代软件设计原则  
- ⚡ **性能优化**：升级到 nom8 解析器，提升解析性能
- 🆕 **版本兼容**：解决 osu! 2025.0107 版本的数据库格式变更问题
- 📚 **API 改进**：提供更加友好和直观的 Rust API

## 许可证声明

- **本项目**：基于 [MIT License](LICENSE) 开源
- **原始项目 osu-db**：基于 [Unlicense](http://unlicense.org) 开源（许可证文件已保存在 `licenses/` 目录下）

我们感谢所有为开源社区做出贡献的开发者们！ 🙏

# 🤝 参与贡献

我们热烈欢迎社区贡献！本项目保留了 `osu-db` 的所有核心功能，但目前主要专注于 OsuDB 解析功能的稳定性和性能优化。

## 🚀 如何贡献

### 报告问题

- 🐛 发现 Bug？请提交详细的 [Issue](https://github.com/osynicite/osynic_osudb/issues)
- 💡 有新想法？欢迎在 [Discussions](https://github.com/osynicite/osynic_osudb/discussions) 中分享

### 代码贡献

**基本流程：**

- **Fork 本仓库**
- **创建特性分支**：`git checkout -b feature/amazing-feature`
- **遵循代码规范**：

  ```bash
  # 运行开发检查
  make quick          # 快速检查（编译 + 格式 + Clippy）
  make pre-commit     # 提交前检查（包含测试）
  
  # 或者单独运行
  cargo fmt --all     # 代码格式化
  cargo clippy --all-features -- -D warnings  # 代码检查
  cargo test --all-features  # 运行测试
  ```

- **提交更改**：`git commit -m 'Add some amazing feature'`
- **推送分支**：`git push origin feature/amazing-feature`
- **创建 Pull Request**

## 📋 贡献规范

- ✅ **代码质量**：遵循 Rust 官方编码规范
- 🧪 **测试覆盖**：新功能必须包含相应的测试用例
- 📝 **文档更新**：重要变更需要更新相关文档
- 🔧 **兼容性**：确保向后兼容性，除非有重大版本更新

## 🎯 当前需要帮助的领域

- 📊 **其他数据库支持**：collection.db、scores.db 的完整测试
- 🔧 **性能优化**：大文件解析性能提升
- 📚 **文档完善**：更多使用示例和最佳实践
- 🌐 **国际化**：多语言错误消息支持

感谢您考虑为 OsynicOsudb 做出贡献！每一个 PR 和 Issue 都是对项目的宝贵贡献。 ✨

# 📜 开源协议

本项目采用 **[MIT License](LICENSE)** 开源协议，这意味着您可以自由地：

- ✅ **商业使用**：在商业项目中使用本库
- ✅ **修改分发**：修改代码并分发您的版本
- ✅ **私人使用**：在个人项目中使用
- ✅ **专利授权**：获得相关专利的使用权

**使用条件：**

- 📄 保留原始许可证和版权声明
- 📝 在修改版本中标明更改内容

---

<p align="center">
  <strong>⭐ 如果这个项目对您有帮助，请给我们一个 Star！ ⭐</strong><br/>
  <em>您的支持是我们持续改进的动力 💪</em>
</p>

<p align="center">
  <a href="https://github.com/osynicite/osynic_osudb">🌟 GitHub 仓库</a> |
  <a href="https://crates.io/crates/osynic_osudb">📦 Crates.io</a> |
  <a href="https://docs.rs/osynic_osudb">📚 文档</a> |
  <a href="https://discord.gg/JWyvc6M5">💬 Discord</a>
</p>
