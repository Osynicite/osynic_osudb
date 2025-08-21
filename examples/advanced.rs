use osynic_osudb::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let osudb = OsuDB::from_file("osu!.db")?;

    // 筛选特定条件的谱面
    let hard_beatmaps: Vec<_> = osudb
        .beatmaps
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
