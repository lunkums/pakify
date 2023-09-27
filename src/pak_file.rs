use std::fs::File;

use crate::{pak_error::PakError, pak_file_entry::PakFileEntry, pak_header::PakHeader};

#[derive(Debug)]
pub struct PakFile {
    source: File,
    header: PakHeader,
    entries: Vec<PakFileEntry>,
}

impl PakFile {
    pub fn source(&self) -> &File {
        &self.source
    }

    pub fn header(&self) -> &PakHeader {
        &self.header
    }

    pub fn entries(&self) -> &Vec<PakFileEntry> {
        &self.entries
    }

    pub fn load(source: File) -> Result<PakFile, PakError> {
        let mut source = source;
        let header = PakHeader::load(&mut source)?;
        let entries =
            PakFileEntry::load_entries(&mut source, header.num_entries(), header.offset())?;
        Ok(PakFile {
            source,
            header,
            entries,
        })
    }

    pub fn load_entry(&mut self, path: &str) -> Result<Vec<u8>, PakError> {
        for entry in self.entries.iter() {
            if path == entry.name() {
                return entry.load(&mut self.source);
            }
        }
        Err(PakError::MissingEntry(path.to_string()))
    }
}
