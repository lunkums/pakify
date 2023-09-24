# pakify

Explore (Quake) .pak files in Rust.

# Usage

```rs
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
```

# Future Plans

- Replace all uses of `seek_read` with a cross-platform function
- Implement additional validation
- Add the ability to load data from the .pak file's virtual file system
- Implement a test suite
- Incorporate this into a game project
