use osynic_osudb::entity::osu::osudb::OsuDB;

fn main() {
    // Load the listing to memory
    // let mut osudb = OsuDB::from_file("D:\\ProgramUnsigned\\Games\\OSU\\osu!.db").unwrap();
    match OsuDB::from_file("archive\\osu!.db") {
        // match OsuDB::from_file("osu!.db") {
        Ok(mut osudb) => {
            // 使用 osudb
            
    // Print info for the first 10 songs
    for (index, beatmap) in osudb.beatmaps.iter_mut().take(3).enumerate() {
        println!("Song {}: {} - {}", index + 1, 
                 beatmap.artist_unicode.as_ref().unwrap(), 
                 beatmap.title_unicode.as_ref().unwrap());
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
        Err(e) => {
            println!("读取文件时发生错误: {:?}", e);
            // 读取文件时发生错误: Parse error: Tag
        }
    }

}
