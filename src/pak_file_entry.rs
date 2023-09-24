use std::{fmt, fs::File, mem, os::windows::prelude::FileExt};

pub struct PakFileEntry {
    name: [u8; 56],
    offset: u32,
    size: u32,
}

impl fmt::Debug for PakFileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PakFileEntry")
            .field("name", &self.name())
            .field("offset", &self.offset)
            .field("size", &self.size)
            .finish()
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

    pub fn from_file(file: &File, length: usize, offset: u32) -> Result<Vec<PakFileEntry>, &str> {
        const NAME_SIZE: usize = PakFileEntry::NAME_LENGTH;
        const OFFSET_SIZE: usize = 4;

        let mut entries = Vec::with_capacity(length);
        let mut name_buffer = [0u8; NAME_SIZE];
        let mut offset_size_buffer = [0u8; OFFSET_SIZE];

        for i in 0..length {
            // Read the name
            let position = (offset as usize + i * mem::size_of::<PakFileEntry>()) as u64;
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
            let position = position + mem::size_of_val(&offset) as u64;
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
