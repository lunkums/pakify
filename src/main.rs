use std::{
    fs::File,
    mem::{size_of, size_of_val},
    os::windows::prelude::FileExt,
};

#[derive(Debug)]
struct PakFile {
    header: PakHeader,
}

impl PakFile {
    fn from_file(file: &File) -> Result<PakFile, &str> {
        match PakHeader::from_file(file) {
            Ok(header) => {
                let entries = [PakFileEntry {
                    name: [0u8; 56],
                    offset: 0,
                    size: 0,
                }; 4];
                header.fill_entries(file, &entries);
                Ok(PakFile { header })
            }
            Err(_) => todo!(),
        }
    }
}

#[derive(Debug)]
struct PakHeader {
    id: u32,
    offset: u32,
    size: u32,
}

impl PakHeader {
    fn from_file(file: &File) -> Result<PakHeader, &'static str> {
        const VALID_PAK_ID: u32 = 1262698832;

        let mut buffer = [0u8; 4];

        // Read the id
        let position = 0;
        let id = match file.seek_read(&mut buffer, position) {
            Ok(bytes_read) => {
                if bytes_read != 4 {
                    return Err("Failed to read the entire header id");
                } else {
                    u32::from_le_bytes(buffer)
                }
            }
            Err(_) => return Err("Failed to read the header id"),
        };

        // Validate the id
        if id != VALID_PAK_ID {
            return Err("Invalid header id");
        }

        // Read the offset
        let position = size_of_val(&id) as u64;
        let offset = match file.seek_read(&mut buffer, position) {
            Ok(bytes_read) => {
                if bytes_read != 4 {
                    return Err("Failed to read the entire header offset");
                } else {
                    u32::from_le_bytes(buffer)
                }
            }
            Err(_) => return Err("Failed to read the header offset"),
        };

        // Validate the offset
        if (offset as usize) < size_of::<PakHeader>() {
            return Err("Invalid header offset");
        }

        // Read the size
        let position = position + size_of_val(&offset) as u64;
        let size = match file.seek_read(&mut buffer, position) {
            Ok(bytes_read) => {
                if bytes_read != 4 {
                    return Err("Failed to read the entire header offset");
                } else {
                    u32::from_le_bytes(buffer)
                }
            }
            Err(_) => return Err("Failed to read the header offset"),
        };

        // Validate the size

        Ok(PakHeader { id, offset, size })
    }

    fn fill_entries(&self, file: &File, entries: &[PakFileEntry]) {
        for entry in entries {
            println!("{:?}", entry);
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PakFileEntry {
    name: [u8; 56],
    offset: u32,
    size: u32,
}

fn main() {
    const PAK_FILE_PATH: &str = "res/foo.pak";
    let file = File::open(PAK_FILE_PATH);

    match file {
        Ok(file) => {
            println!("Opened {}", PAK_FILE_PATH);
            match PakFile::from_file(&file) {
                Ok(pak_file) => println!("{:?}", pak_file),
                Err(error) => println!("Failed to read {PAK_FILE_PATH} due to: {error}"),
            }
        }
        Err(error) => println!("Failed to open the file due to: {}", error),
    }
}
