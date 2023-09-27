use std::{fs::File, mem, os::windows::prelude::FileExt};

use crate::pak_error::PakError;

#[derive(Debug)]
pub struct PakFileEntry {
    name: String,
    offset: u32,
    size: u32,
}

impl PakFileEntry {
    pub const SIZE: usize = 64;

    const NAME_LENGTH: usize = 56;

    pub fn load_entries(
        file: &File,
        length: usize,
        offset: u32,
    ) -> Result<Vec<PakFileEntry>, PakError> {
        const NAME_SIZE: usize = PakFileEntry::NAME_LENGTH;
        const OFFSET_SIZE: usize = 4;

        let mut entries = Vec::with_capacity(length);
        let mut name_buffer = [0u8; NAME_SIZE];
        let mut offset_size_buffer = [0u8; OFFSET_SIZE];

        for i in 0..length {
            // Read the name
            let position = (offset as usize + i * mem::size_of::<PakFileEntry>()) as u64;
            let bytes_read = file.seek_read(&mut name_buffer, position)?;
            let name = if bytes_read != NAME_SIZE {
                return Err(PakError::UnexpectedEof);
            } else {
                // Map the bytes to a NULL-terminated UTF-8 string
                name_buffer
                    .iter()
                    .take_while(|&&byte| byte != b'\0')
                    .map(|&byte| byte as char)
                    .collect()
            };

            // Read the offset
            let position = position + NAME_SIZE as u64;
            let bytes_read = file.seek_read(&mut offset_size_buffer, position)?;
            let offset = if bytes_read != OFFSET_SIZE {
                return Err(PakError::UnexpectedEof);
            } else {
                u32::from_le_bytes(offset_size_buffer)
            };

            // Read the size
            let position = position + mem::size_of_val(&offset) as u64;
            let bytes_read = file.seek_read(&mut offset_size_buffer, position)?;
            let size = if bytes_read != OFFSET_SIZE {
                return Err(PakError::UnexpectedEof);
            } else {
                u32::from_le_bytes(offset_size_buffer)
            };

            entries.push(PakFileEntry { name, offset, size });
        }

        Ok(entries)
    }
}
