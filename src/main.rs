use pakify::{PakError, PakFile};

const PAK_FILE_PATH: &str = "res/foo.pak";
const TEST_ENTRY_PATH: &str = "folder/foo.txt";
const MISSING_ENTRY_PATH: &str = "nothing.txt";

fn main() {
    match pak_test() {
        Ok(_) => println!("Successfully loaded the pak file at {PAK_FILE_PATH}"),
        Err(error) => println!("Failed to load the pak file due to: {error}"),
    }
}

fn pak_test() -> Result<(), PakError> {
    let file = std::fs::File::open(PAK_FILE_PATH)
        .expect(format!("pak file should exist at: {}", PAK_FILE_PATH).as_str());
    let mut pak_file = PakFile::load(file)?;

    println!("The pak file: {:#?}", pak_file);

    let entry = pak_file.load_entry(TEST_ENTRY_PATH)?;

    println!(
        "Contents of the '{}' entry: {:#?}",
        TEST_ENTRY_PATH,
        entry.escape_ascii().to_string()
    );

    let error = pak_file
        .load_entry(MISSING_ENTRY_PATH)
        .expect_err(format!("Shouldn't have been able to load '{}'", MISSING_ENTRY_PATH).as_str());

    println!("Expected error: {}", error);

    Ok(())
}
