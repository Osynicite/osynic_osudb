use osynic_osudb::entity::osu::osudb::OsuDB;

fn main() {
    // Load the listing to memory
    // let mut osudb = OsuDB::from_file("D:\\ProgramUnsigned\\Games\\OSU\\osu!.db").unwrap();
    match OsuDB::from_file("archive\\osu!.db") {
        // match OsuDB::from_file("osu!.db") {
        Ok(mut osudb) => {
            // 使用 osudb

            for (index, beatmap) in osudb.beatmaps.iter_mut().take(1).enumerate() {
                println!("beatmap {}: {:?}", index, beatmap);
            }
        }
        Err(e) => {
            println!("读取文件时发生错误: {:?}", e);
            // 读取文件时发生错误: Parse error: Tag
        }
    }
}
