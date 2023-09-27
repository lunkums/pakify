use std::{
    fmt,
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem,
};

use crate::{pak_error::PakError, pak_file_entry::PakFileEntry};

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
        self.size as usize / PakFileEntry::SIZE
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    fn id(&self) -> String {
        self.id
            .to_le_bytes()
            .iter()
            .map(|byte| *byte as char)
            .collect()
    }

    pub fn load(file: &mut File) -> Result<PakHeader, PakError> {
        const VALID_PAK_ID: u32 = 1262698832; // Equal to the UTF-8 string "PACK"
        const SIZE: usize = 4;

        let mut buffer = [0u8; SIZE];

        // Read the id
        let position = 0;
        file.seek(SeekFrom::Start(position))?;
        let bytes_read = file.read(&mut buffer)?;
        let id = if bytes_read != SIZE {
            return Err(PakError::UnexpectedEof);
        } else {
            u32::from_le_bytes(buffer)
        };

        // Validate the id
        if id != VALID_PAK_ID {
            return Err(PakError::InvalidField("header.id"));
        }

        // Read the offset
        let position = position + mem::size_of_val(&id) as u64;
        file.seek(SeekFrom::Start(position))?;
        let bytes_read = file.read(&mut buffer)?;
        let offset = if bytes_read != SIZE {
            return Err(PakError::UnexpectedEof);
        } else {
            u32::from_le_bytes(buffer)
        };

        // Validate the offset
        if (offset as usize) < mem::size_of::<PakHeader>() {
            return Err(PakError::InvalidField("header.offset"));
        }

        // Read the size
        let position = position + mem::size_of_val(&offset) as u64;
        file.seek(SeekFrom::Start(position))?;
        let bytes_read = file.read(&mut buffer)?;
        let size = if bytes_read != SIZE {
            return Err(PakError::UnexpectedEof);
        } else {
            u32::from_le_bytes(buffer)
        };

        // Validate the size
        if (size as usize) % PakFileEntry::SIZE != 0 {
            return Err(PakError::InvalidField("header.size"));
        }

        Ok(PakHeader { id, offset, size })
    }
}
