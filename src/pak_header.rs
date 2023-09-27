use std::{fmt, fs::File, mem, os::windows::prelude::FileExt};

use crate::pak_file_entry::PakFileEntry;

pub struct PakHeader {
    id: u32,
    offset: u32,
    size: u32,
}

impl fmt::Debug for PakHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PakHeader")
            .field("id", &self.id())
            .field("offset", &self.offset)
            .field("size", &self.size)
            .finish()
    }
}

impl PakHeader {
    pub fn num_entries(&self) -> usize {
        self.size as usize / mem::size_of::<PakFileEntry>()
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    fn id(&self) -> String {
        let id_bytes = self.id.to_le_bytes();
        let mut id = String::new();

        for byte in id_bytes {
            id.push(byte as char);
        }

        id
    }

    pub fn from_file(file: &File) -> Result<PakHeader, &str> {
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
        let position = mem::size_of_val(&id) as u64;
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
        if (offset as usize) < mem::size_of::<PakHeader>() {
            return Err("Invalid header offset");
        }

        // Read the size
        let position = position + mem::size_of_val(&offset) as u64;
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
}
