# Osynic OsuDB - Architecture Diagrams

这个文档包含了 osynic_osudb 项目的各种架构图，使用 PlantUML 绘制。

## 1. C4 Context Diagram - 系统上下文图

展示整个系统与外部系统的交互关系。

```plantuml
@startuml C4_Context
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

title osynic_osudb - System Context

Person(user, "osu! 用户", "需要解析游戏数据库文件")
System(osynic, "Osynic OsuDB", "高性能 osu! 数据库解析库")

System_Ext(osu_game, "osu! Game Client", "生成 .db 文件")
System_Ext(rust_app, "Rust 应用", "调用库进行数据解析")
System_Ext(web_app, "Web/Node.js 应用", "通过 WASM 调用库")

Rel(user, osu_game, "使用")
Rel(osu_game, osynic, "生成数据库文件")
Rel(rust_app, osynic, "调用库 API")
Rel(web_app, osynic, "通过 WASM 调用")

footer C4 Context Diagram
@enduml
```

## 2. C4 Container Diagram - 容器图

展示系统内部的主要容器和它们之间的关系。

```plantuml
@startuml C4_Container
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

title osynic_osudb - Container Diagram

Person(user, "开发者", "使用库来解析 osu! 数据库")

Container(io_module, "IO Module", "Rust", "处理二进制读写操作\n- read.rs: 二进制读取\n- write.rs: 二进制写入\n- bit.rs: 位操作")

Container(entity_osu, "OSU Entity", "Rust", "osu!.db 相关数据结构\n- OsuDB: 主数据库\n- Beatmap: 谱面信息\n- Field: 枚举和类型")

Container(entity_collection, "Collection Entity", "Rust", "collection.db 相关数据结构\n- CollectionDB: 集合数据库\n- Collection: 单个集合")

Container(entity_scores, "Scores Entity", "Rust", "scores.db 相关数据结构\n- ScoresDB: 分数数据库\n- Scores: 单个分数记录")

Container(wasm_module, "WASM Module", "Rust/JavaScript", "WASM 绑定和导出\n支持浏览器和 Node.js")

Container(error_module, "Error Module", "Rust", "错误处理\n- 解析错误\n- IO 错误")

Rel(user, entity_osu, "使用")
Rel(user, entity_collection, "使用")
Rel(user, entity_scores, "使用")
Rel(user, wasm_module, "通过 WASM 使用")

Rel(entity_osu, io_module, "依赖")
Rel(entity_collection, io_module, "依赖")
Rel(entity_scores, io_module, "依赖")

Rel(entity_osu, error_module, "使用")
Rel(entity_collection, error_module, "使用")
Rel(entity_scores, error_module, "使用")
Rel(io_module, error_module, "使用")

Rel(wasm_module, entity_osu, "包装")
Rel(wasm_module, entity_collection, "包装")
Rel(wasm_module, entity_scores, "包装")

footer C4 Container Diagram
@enduml
```

## 3. Component Diagram - 组件图

详细展示各个模块的内部结构和组件。

```plantuml
@startuml Components
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

title osynic_osudb - Component Diagram

Container_Boundary(io, "IO Module") {
    Component(read, "ReadExt", "Trait", "读取二进制数据")
    Component(write, "WriteExt", "Trait", "写入二进制数据")
    Component(bit, "BitReader", "Struct", "位级读取操作")
}

Container_Boundary(osu, "OSU Entity") {
    Component(osudb, "OsuDB", "Struct", "主数据库结构")
    Component(beatmap, "Beatmap", "Struct", "谱面信息")
    Component(grade, "Grade", "Enum", "成绩等级")
    Component(mode, "Mode", "Enum", "游戏模式")
    Component(rank_status, "RankedStatus", "Enum", "谱面排名状态")
    Component(star, "StarRatings", "Struct", "星级评分")
    Component(timing, "TimingPoint", "Struct", "定时点")
}

Container_Boundary(collection, "Collection Entity") {
    Component(collectiondb, "CollectionDB", "Struct", "集合数据库")
    Component(coll, "Collection", "Struct", "单个集合")
}

Container_Boundary(scores, "Scores Entity") {
    Component(scoresdb, "ScoresDB", "Struct", "分数数据库")
    Component(score, "Scores", "Struct", "单个分数记录")
    Component(action, "Action", "Enum", "玩家操作")
    Component(button, "Button", "Enum", "按键信息")
    Component(replay, "Replay", "Struct", "重放数据")
}

Rel(osudb, beatmap, "包含")
Rel(osudb, read, "使用")
Rel(beatmap, star, "包含")
Rel(beatmap, timing, "包含")
Rel(beatmap, grade, "使用")
Rel(beatmap, mode, "使用")
Rel(beatmap, rank_status, "使用")

Rel(collectiondb, coll, "包含")
Rel(collectiondb, read, "使用")

Rel(scoresdb, score, "包含")
Rel(scoresdb, read, "使用")
Rel(score, action, "使用")
Rel(score, button, "使用")
Rel(score, replay, "包含")

footer Component Diagram
@enduml
```

## 4. Class Diagram - 类图

展示主要数据结构的关系。

```plantuml
@startuml ClassDiagram
title osynic_osudb - Class Diagram

package "OSU Module" {
    class OsuDB {
        - version: u32
        - folder_count: u32
        - unban_date: Option<DateTime>
        - player_name: Option<String>
        - beatmaps: Vec<Beatmap>
        - user_permissions: u32
        + new()
        + read()
        + write()
    }

    class Beatmap {
        - artist_ascii: Option<String>
        - artist_unicode: Option<String>
        - title_ascii: Option<String>
        - title_unicode: Option<String>
        - creator: Option<String>
        - difficulty_name: Option<String>
        - audio: Option<String>
        - hash: Option<String>
        - status: RankedStatus
        - hitcircle_count: u16
        - slider_count: u16
        - spinner_count: u16
        - last_modified: DateTime
        - approach_rate: f32
        - circle_size: f32
        - hp_drain: f32
        - overall_difficulty: f32
        - slider_velocity: f64
        - std_ratings: StarRatings
        - taiko_ratings: StarRatings
        - ctb_ratings: StarRatings
        - mania_ratings: StarRatings
        - timing_points: Vec<TimingPoint>
        + new()
        + read()
        + write()
    }

    enum RankedStatus {
        Unknown
        Unsubmitted
        Pending
        Ranked
        Approved
        Qualified
        Loved
    }

    enum Mode {
        Standard
        Taiko
        CatchTheBeat
        Mania
    }

    class StarRatings {
        - star_rating: f32
    }

    class TimingPoint {
        - bpm: f64
        - offset: f64
        - meter: u32
        - sample_type: u32
        - sample_set: u32
        - volume: u32
        - is_inherited: bool
        - kiai_mode: bool
    }

    OsuDB *-- "many" Beatmap : contains
    Beatmap -- RankedStatus : uses
    Beatmap -- Mode : uses
    Beatmap *-- "4" StarRatings : has
    Beatmap *-- "many" TimingPoint : has
}

package "Collection Module" {
    class CollectionDB {
        - version: u32
        - collections: Vec<Collection>
        + new()
        + read()
        + write()
    }

    class Collection {
        - name: String
        - beatmap_hashes: Vec<String>
        + new()
        + add_beatmap()
        + remove_beatmap()
    }

    CollectionDB *-- "many" Collection : contains
}

package "Scores Module" {
    class ScoresDB {
        - version: u32
        - beatmaps: Vec<BeatmapScores>
        + new()
        + read()
        + write()
    }

    class BeatmapScores {
        - beatmap_hash: String
        - scores: Vec<Scores>
    }

    class Scores {
        - mode: Mode
        - score: u32
        - accuracy: f32
        - max_combo: u16
        - perfect_combo: bool
        - mods: Vec<Mod>
        - hp_graph: Option<String>
        - timestamp: DateTime
        - replay_data: Option<Vec<u8>>
        - action: Action
        - button_press: ButtonSet
        - replay: Option<Replay>
        + new()
        + read()
        + write()
    }

    enum Action {
        Standard
        Osr
    }

    class Replay {
        - frames: Vec<ReplayFrame>
        - seed: Option<u32>
    }

    class ReplayFrame {
        - delta_time: i32
        - x: f32
        - y: f32
        - buttons: u32
    }

    ScoresDB *-- "many" BeatmapScores : contains
    BeatmapScores *-- "many" Scores : contains
    Scores -- Action : uses
    Scores -- Mode : uses
    Scores *-- "0..1" Replay : has
    Replay *-- "many" ReplayFrame : contains
}

package "IO Module" {
    interface ReadExt {
        + read_u8()
        + read_u16()
        + read_u32()
        + read_u64()
        + read_f32()
        + read_f64()
        + read_string()
        + read_datetime()
    }

    interface WriteExt {
        + write_u8()
        + write_u16()
        + write_u32()
        + write_u64()
        + write_f32()
        + write_f64()
        + write_string()
        + write_datetime()
    }

    class BitReader {
        - position: usize
        - data: Vec<u8>
        + new()
        + read_bit()
        + read_bits()
        + skip()
    }
}

OsuDB --> ReadExt : uses
CollectionDB --> ReadExt : uses
ScoresDB --> ReadExt : uses
Beatmap --> ReadExt : uses
Collection --> ReadExt : uses
Scores --> ReadExt : uses

@enduml
```

## 5. Sequence Diagram - 序列图

展示系统中的主要交互流程。

```plantuml
@startuml Sequence_Parse
title osynic_osudb - OSU Database Parsing Sequence

actor User
participant "Rust App" as App
participant "OsuDB Parser" as Parser
participant "IO Module" as IO
participant "Binary File" as File

User -> App: Load osu!.db file
App -> Parser: new(file_path)
Parser -> IO: open_file(path)
IO -> File: read bytes
IO --> Parser: file handle

Parser -> IO: read_u32() for version
IO -> File: read 4 bytes
File --> IO: bytes
IO --> Parser: version

Parser -> IO: read_u32() for folder_count
IO -> File: read 4 bytes
File --> IO: bytes
IO --> Parser: folder_count

loop For each beatmap
    Parser -> IO: read_beatmap_data()
    IO -> File: read variable bytes
    File --> IO: beatmap bytes
    IO --> Parser: Beatmap struct
end

Parser --> App: OsuDB {version, folder_count, beatmaps}
App --> User: Parsed data

@enduml
```

```plantuml
@startuml Sequence_WASM
title osynic_osudb - WASM Module Usage Sequence

actor "JavaScript" as JS
participant "WASM Module" as WASM
participant "Rust Core" as Core
participant "Memory" as Mem

JS -> WASM: load_wasm()
WASM --> JS: module ready

JS -> WASM: parse_osudb(file_buffer)
WASM -> Core: call parse function
Core -> Mem: allocate memory
Core -> Mem: read binary data
Core -> Core: parse data
Mem <-- Core: write parsed data
WASM <- Core: return result
WASM -> WASM: serialize to JS types
WASM --> JS: OsuDB object (JSON)

JS -> JS: use data in application

@enduml
```

## 6. Data Flow Diagram - 数据流图

展示不同类型的数据库文件的处理流程。

```plantuml
@startuml DataFlow
title osynic_osudb - Data Processing Flow

rectangle "Input Files" {
    file osutag as "osu!.db"
    file collectiontag as "collection.db"
    file scorestag as "scores.db"
}

rectangle "Parsing Layer" {
    process osu_parser as "OsuDB Parser"
    process collection_parser as "CollectionDB Parser"
    process scores_parser as "ScoresDB Parser"
}

rectangle "Entity Layer" {
    process osu_entity as "OsuDB Entity"
    process collection_entity as "CollectionDB Entity"
    process scores_entity as "ScoresDB Entity"
}

rectangle "Output" {
    file rust_output as "Rust Objects\n(serde)"
    file wasm_output as "WASM Objects\n(tsify)"
    file json_output as "JSON\n(serde_json)"
}

osutag --> osu_parser
collectiontag --> collection_parser
scorestag --> scores_parser

osu_parser --> osu_entity
collection_parser --> collection_entity
scores_parser --> scores_entity

osu_entity --> rust_output
collection_entity --> rust_output
scores_entity --> rust_output

rust_output --> wasm_output
rust_output --> json_output

@enduml
```

## 7. Deployment Diagram - 部署图

展示库在不同环境中的部署方式。

```plantuml
@startuml Deployment
title osynic_osudb - Deployment Diagram

rectangle "Development Environment" {
    node rust_dev as "Rust Project" {
        component lib as "osynic_osudb\nlibrary"
    }
}

rectangle "Rust Ecosystem" {
    node cargo as "Cargo Package" {
        component crate as "crates.io\nosynic_osudb"
    }
}

rectangle "JavaScript/WASM Ecosystem" {
    node npm as "NPM Registry" {
        component pkg as "npm package\n@osynicite/osynic-osudb"
    }
    
    node deno as "Deno" {
        component deno_pkg as "Deno Module"
    }
}

rectangle "Runtime Environments" {
    node rust_app as "Rust Application" {
        component lib_usage as "Use Library"
    }
    
    node browser as "Web Browser" {
        component wasm_js as "JavaScript + WASM"
    }
    
    node nodejs as "Node.js" {
        component node_pkg as "JavaScript Module"
    }
}

lib --> crate : publish
lib --> npm : build WASM
lib --> deno : publish

crate --> rust_app : depend
npm --> browser : load
npm --> nodejs : require
deno --> nodejs : use

@enduml
```

## 8. Entity Relationship Diagram - 实体关系图

展示各个数据结构之间的关系。

```plantuml
@startuml ER_Diagram
title osynic_osudb - Entity Relationship Diagram

entity "OsuDB" as osu {
    * version: u32
    * folder_count: u32
    --
    o unban_date: DateTime
    o player_name: String
    * user_permissions: u32
}

entity "Beatmap" as beatmap {
    * hash: String
    --
    o artist_ascii: String
    o artist_unicode: String
    o title_ascii: String
    o title_unicode: String
    o creator: String
    o difficulty_name: String
    * status: RankedStatus
    * hitcircle_count: u16
    * slider_count: u16
    * spinner_count: u16
    * last_modified: DateTime
    * circle_size: f32
    * approach_rate: f32
    * overall_difficulty: f32
    * hp_drain: f32
}

entity "StarRatings" as star {
    * star_rating: f32
}

entity "TimingPoint" as timing {
    * bpm: f64
    * offset: f64
    * meter: u32
    * kiai_mode: bool
}

entity "CollectionDB" as cdb {
    * version: u32
}

entity "Collection" as coll {
    * name: String
}

entity "ScoresDB" as sdb {
    * version: u32
}

entity "BeatmapScores" as bscores {
    * beatmap_hash: String
}

entity "Scores" as scores {
    * score: u32
    * accuracy: f32
    * max_combo: u16
    * timestamp: DateTime
    * perfect_combo: bool
}

entity "Replay" as replay {
}

entity "ReplayFrame" as rframe {
    * delta_time: i32
    * x: f32
    * y: f32
}

osu ||--o{ beatmap : contains
beatmap ||--|| star : "std_rating"
beatmap ||--|| star : "taiko_rating"
beatmap ||--|| star : "ctb_rating"
beatmap ||--|| star : "mania_rating"
beatmap ||--o{ timing : contains

cdb ||--o{ coll : contains
coll ||--o{ beatmap : references

sdb ||--o{ bscores : contains
bscores ||--o{ scores : contains
scores ||--o| beatmap : references
scores ||--o| replay : contains
replay ||--o{ rframe : contains

@enduml
```

---

## 使用说明

这些图表可以通过以下方式查看：

### 在线查看
1. 使用 [PlantUML Online Editor](http://plantuml.com/plantuml/uml/):
   - 复制上面的代码到编辑器
   - 点击"Render"查看生成的图表

### 本地查看
1. **VS Code 扩展**: 
   - 安装 "PlantUML" 扩展
   - 右键点击代码块，选择"Preview"

2. **命令行**:
   ```bash
   plantuml plantuml.md -o output/
   ```

3. **在 Markdown 中渲染**:
   - 使用支持 PlantUML 的 Markdown 渲染器，如 GitHub Pages

### 注意
- 这些图表展示了系统的核心架构和组件
- 随着项目的演进，请保持图表的更新
- 更多信息请参考 [PlantUML 文档](http://plantuml.com/)
