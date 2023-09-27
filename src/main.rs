use std::fs::File;

use pakify::{PakError, PakFile};

fn main() {
    const PAK_FILE_PATH: &str = "res/foo.pak";
    const TEST_ENTRY_PATH: &str = "folder/foo.txt";

    match pak_test(PAK_FILE_PATH, TEST_ENTRY_PATH) {
        Ok(_) => println!("Successfully loaded the pak file at {PAK_FILE_PATH}"),
        Err(error) => println!("Failed to load the pak file due to: {error}"),
    }
}

fn pak_test(file_path: &str, entry_path: &str) -> Result<(), PakError> {
    let file = File::open(file_path)
        .expect(format!("pak file should exist at: {file_path}").as_str());
    let mut pak_file = PakFile::load(file)?;
    let entry = pak_file.load_entry(entry_path)?;

    println!("Contents of the '{}' entry: {:#?}", entry_path, entry.escape_ascii().to_string());

    Ok(())
}