use std::path::Path;
use std::fs::write;

const FILE_BASE:&str = r#"{
"top":[],
"jng":[],
"mid":[],
"bot":[],
"sup":[]
}"#;

pub fn create_player(folder: &Path, name:&str) -> Result<(),String>{
    let t = folder.join(format!("{}.json",name));
    write(&t,FILE_BASE).map_err(|e| format!("Failed to write {} because {}",t.to_string_lossy(), e.to_string()))
}
