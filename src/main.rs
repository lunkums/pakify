use std::fs::File;

use pak_rs::PakFile;

fn main() {
    const PAK_FILE_PATH: &str = "res/foo.pak";
    let file = File::open(PAK_FILE_PATH);

    match file {
        Ok(file) => {
            println!("Opened {}", PAK_FILE_PATH);
            match PakFile::from_file(&file) {
                Ok(pak_file) => println!("{:#?}", pak_file),
                Err(error) => println!("Failed to read {PAK_FILE_PATH} due to: {error}"),
            }
        }
        Err(error) => println!("Failed to open the file due to: {error}"),
    }
}
