<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsudb</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_osudb" target="_blank"><img src="https://img.shields.io/crates/v/osynic_osudb"/></a>
  <a href="https://docs.rs/osynic_osudb" target="_blank"><img src="https://img.shields.io/docsrs/osynic_osudb/0.1.0"/></a>
  <a href="https://github.com/osynicite/osynic_osudb" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
  <a href="https://discord.gg/JWyvc6M5" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
  <a href="https://github.com/osynicite" target="_blank"><img src="https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square"/></a>

</p>

<p align="center">
    OSU!曲谱同步器Osynic的OSU!DB解析部分
</p>

<hr />

[中文版本](README.md) | [English Version](README_EN.md)

# 📄 简介

Osynic的OSU!DB解析部分，基于[osu-db](https://crates.io/crates/osu-db)重构，主要做了读写分离并改了实体结构，最近把解析部分升级到了nom8，并解决了osu!.db在20250107版本的变更

# 🌐 相关项目

[osynic_serializer](https://github.com/osynicite/osynic_serializer) 是一款高效的osu!谱面序列化工具，基于[osynic_osudb](https://github.com/osynicite/osynic_osudb)开发，支持FOLDER、OSUDB两种序列化算法；搭配[osynic_downloader](https://github.com/osynicite/osynic_downloader)使用可实现osu!谱面的快速序列化与多设备谱面同步。

![osynic_serializer.gif](https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif)

# 📦 实体结构

- entity
  - osu
    - osudb
    - beatmaps
    - fields
      - grade
      - mode
      - modification
      - rank
      - star
      - time
  - collection
    - collection
    - collectiondb
  - scores
    - scores
    - scoresdb
    - fields
      - action
      - button
      - replay

# 📦 安装

在你的`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
osynic_osudb = "0.1.0"
```

# 📖 使用

```rust
use osynic_osudb::prelude::OsuDB;

fn main() {
    // 将osu!.db文件解析为OsuDB结构体
    let mut osudb = OsuDB::from_file("osu!.db").unwrap();

    // 打印前三个谱面的信息
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

# ❤️ 鸣谢

本项目的主要实现来源于`osu-db`这个crate，如果要说我做了什么的话，主要就在项目结构按照自己个人偏好进行的调整了

osynic_osudb是基于[osu-db](https://crates.io/crates/osu-db)重构的，主要做了读写分离并改了实体结构，最近把解析部分升级到了nom8，并解决了osu!.db在20250107版本的变更

`osu-db`基于 [Unlicense](http://unlicense.org) 开源协议，尽管如此，我们也将其项目证书放在了licenses文件夹下

# 🤝 贡献指南

这个库基本上保留了`osu-db`的所有功能，但是我的业务需求只需要使用其中的OsuDB解析部分，其他部分是否好使我并没有测试过

所以，如果代码有任何问题，或者你有任何建议，欢迎提交PR或者Issue，我会尽快处理~

如果你想贡献代码，请遵循以下规则：

- 遵循Rust官方编码规范
- 新增功能需附带测试用例
- 提交前运行`cargo fmt`和`cargo clippy`

# 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。
