use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem,
};

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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn load_entries(
        file: &mut File,
        length: usize,
        offset: u32,
    ) -> Result<Vec<PakFileEntry>, PakError> {
        const NAME_SIZE: usize = PakFileEntry::NAME_LENGTH;

        let mut entries = Vec::with_capacity(length);
        let mut name_buffer = [0u8; NAME_SIZE];
        let mut offset_size_buffer = [0u8; mem::size_of::<u32>()];

        for i in 0..length {
            // Read the name
            let position = (offset as usize + i * PakFileEntry::SIZE) as u64;
            file.seek(SeekFrom::Start(position))?;
            let bytes_read = file.read(&mut name_buffer)?;
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
            file.seek(SeekFrom::Start(position))?;
            let bytes_read = file.read(&mut offset_size_buffer)?;
            let offset = if bytes_read != mem::size_of::<u32>() {
                return Err(PakError::UnexpectedEof);
            } else {
                u32::from_le_bytes(offset_size_buffer)
            };

            // Read the size
            let position = position + mem::size_of_val(&offset) as u64;
            file.seek(SeekFrom::Start(position))?;
            let bytes_read = file.read(&mut offset_size_buffer)?;
            let size = if bytes_read != mem::size_of::<u32>() {
                return Err(PakError::UnexpectedEof);
            } else {
                u32::from_le_bytes(offset_size_buffer)
            };

            entries.push(PakFileEntry { name, offset, size });
        }

        Ok(entries)
    }

    pub fn load<T: Read + Seek>(&self, source: &mut T) -> Result<Vec<u8>, PakError> {
        let size = self.size as usize;
        let mut buffer = vec![0u8; size];

        source.seek(SeekFrom::Start(self.offset as u64))?;
        let bytes_read = source.read(&mut buffer)?;

        if bytes_read == size {
            Ok(buffer)
        } else {
            Err(PakError::UnexpectedEof)
        }
    }
}
