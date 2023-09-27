use std::fs::File;

use crate::{pak_error::PakError, pak_file_entry::PakFileEntry, pak_header::PakHeader};

#[derive(Debug)]
pub struct PakFile {
    pub header: PakHeader,
    pub entries: Vec<PakFileEntry>,
}

impl PakFile {
    pub fn load(file: &File) -> Result<PakFile, PakError> {
        let header = PakHeader::load(file)?;
        let entries = PakFileEntry::load_entries(file, header.num_entries(), header.offset())?;
        Ok(PakFile { header, entries })
    }
}
