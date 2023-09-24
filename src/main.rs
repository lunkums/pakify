use std::{
    fs::File,
    mem::{size_of, size_of_val},
    os::windows::prelude::FileExt, fmt,
};

#[derive(Debug)]
struct PakFile {
    header: PakHeader,
    entries: Vec<PakFileEntry>,
}

impl PakFile {
    fn from_file(file: &File) -> Result<PakFile, &str> {
        match PakHeader::from_file(file) {
            Ok(header) => {
                let entries = header.load_entries(file);

                match entries {
                    Ok(entries) => Ok(PakFile { header, entries }),
                    Err(_) => {
                        Err("Invalid file entry")
                    }
                }
            }
            Err(error) => Err(error),
        }
    }
}
struct PakHeader {
    id: u32,
    offset: u32,
    size: u32,
}

impl fmt::Debug for PakHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PakHeader").field("id", &self.id()).field("offset", &self.offset).field("size", &self.size).finish()
    }
}

impl PakHeader {
    fn id(&self) -> String {
        let id_bytes = self.id.to_le_bytes();
        let mut id = String::new();

        for byte in id_bytes {
            id.push(byte as char);
        }

        id
    }

    fn num_entries(&self) -> usize {
        self.size as usize / size_of::<PakFileEntry>()
    }

    fn from_file(file: &File) -> Result<PakHeader, &str> {
        const VALID_PAK_ID: u32 = 1262698832;
        const SIZE: usize = 4;

        let mut buffer = [0u8; SIZE];

        // Read the id
        let position = 0;
        let id = match file.seek_read(&mut buffer, position) {
            Ok(bytes_read) => {
                if bytes_read != SIZE {
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
                if bytes_read != SIZE {
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
                if bytes_read != SIZE {
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

    fn load_entries(&self, file: &File) -> Result<Vec<PakFileEntry>, &str> {
        const NAME_SIZE: usize = PakFileEntry::NAME_LENGTH;
        const OFFSET_SIZE: usize = 4;

        let length = self.num_entries();
        let mut entries = Vec::with_capacity(length);
        let mut name_buffer = [0u8; NAME_SIZE];
        let mut offset_size_buffer = [0u8; OFFSET_SIZE];

        for i in 0..length {
            // Read the name
            let position = (self.offset as usize + i * size_of::<PakFileEntry>()) as u64;
            let name = match file.seek_read(&mut name_buffer, position) {
                Ok(bytes_read) => {
                    if bytes_read != NAME_SIZE {
                        return Err("Failed to read the entire file entry name");
                    } else {
                        name_buffer
                    }
                }
                Err(_) => return Err("Failed to read the file entry name"),
            };

            // Read the offset
            let position = position + NAME_SIZE as u64;
            let offset = match file.seek_read(&mut offset_size_buffer, position) {
                Ok(bytes_read) => {
                    if bytes_read != OFFSET_SIZE {
                        return Err("Failed to read the entire file entry offset");
                    } else {
                        u32::from_le_bytes(offset_size_buffer)
                    }
                }
                Err(_) => return Err("Failed to read the file entry offset"),
            };

            // Read the size
            let position = position + size_of_val(&offset) as u64;
            let size = match file.seek_read(&mut offset_size_buffer, position) {
                Ok(bytes_read) => {
                    if bytes_read != OFFSET_SIZE {
                        return Err("Failed to read the entire file entry offset");
                    } else {
                        u32::from_le_bytes(offset_size_buffer)
                    }
                }
                Err(_) => return Err("Failed to read the file entry offset"),
            };

            entries.push(PakFileEntry { name, offset, size });
        }

        Ok(entries)
    }
}

struct PakFileEntry {
    name: [u8; 56],
    offset: u32,
    size: u32,
}

impl fmt::Debug for PakFileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PakFileEntry").field("name", &self.name()).field("offset", &self.offset).field("size", &self.size).finish()
    }
}

impl PakFileEntry {
    const NAME_LENGTH: usize = 56;

    fn name(&self) -> String {
        let mut name = String::new();

        for character in self.name {
            if character != b'\0' {
                name.push(character as char);
            } else {
                break;
            }
        }

        name
    }
}

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
