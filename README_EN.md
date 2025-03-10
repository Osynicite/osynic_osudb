<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="logo"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsudb</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_osudb" target="_blank"><img src="https://img.shields.io/crates/v/osynic_osudb"/></a>
  <a href="https://docs.rs/osynic_osudb" target="_blank"><img src="https://img.shields.io/docsrs/osynic_osudb/0.1.0"/></a>
  <a href="https://github.com/osynicite/osynic_osudb" target="_blank"><img src="https://img.shields.io/badge/License-MIT%202-green.svg"/></a>
  <a href="https://discord.gg/JWyvc6M5" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
  <a href="https://github.com/osynicite" target="_blank"><img src="https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square"/></a>

</p>

<p align="center">
    Osu!DB parser for Osynic Osu! Beatmapsets Sync
</p>

<hr />

[中文版本](README.md) | [English Version](README_EN.md)

# Introduction

Osynic's OSU!DB parsing part, based on [osu-db](https://crates.io/crates/osu-db) refactoring, mainly did read-write separation and changed the entity structure, recently upgraded the parsing part to nom8, and solved the change of osu!.db in the 20250107 version

# Related Projects

[osynic_serializer](https://github.com/osynicite/osynic_serializer) is an efficient osu! beatmap serialization tool developed based on [osynic_osudb](https://github.com/osynicite/osynic_osudb), supporting FOLDER and OSUDB serialization algorithms; used with [osynic_downloader](https://github.com/osynicite/osynic_downloader) can achieve fast beatmap serialization and multi-device beatmap synchronization.

![osynic_serializer.gif](https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif)

# Entity Structure

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

# Acknowledgement

The main implementation of this project comes from the `osu-db` crate. If you want to say what I did, the main thing is to adjust the project structure according to my personal preferences

osynic_osudb is based on [osu-db](https://crates.io/crates/osu-db) refactoring, mainly doing read-write separation and changing the entity structure, and recently upgrading the parsing part to nom8 and solving the change of osu!.db in the 20250107 version

The `osu-db` is based on the [Unlicense](http://unlicense.org) license. Nevertheless, we also put its license in the licenses folder.

# 🤝 Contribution Guidelines

This library basically retains all the functions of `osu-db`, but my business needs only need to use the OsuDB parsing part, and I have not tested whether other parts are working well

So, if there is any problem with the code, or if you have any suggestions, please submit a PR or Issue, and I will deal with it as soon as possible~

If you want to contribute code, please follow these rules:

- Follow the official Rust coding specifications
- New features must be accompanied by test cases
- Run `cargo fmt` and `cargo clippy` before submitting

# 📜 License

This project is open source based on the [MIT License](LICENSE), please respect the original author's copyright.
