use std::fs::File;

use crate::{pak_file_entry::PakFileEntry, pak_header::PakHeader};

#[derive(Debug)]
pub struct PakFile {
    pub header: PakHeader,
    pub entries: Vec<PakFileEntry>,
}

impl PakFile {
    pub fn from_file(file: &File) -> Result<PakFile, &str> {
        match PakHeader::from_file(file) {
            Ok(header) => {
                let entries = PakFileEntry::from_file(file, header.num_entries(), header.offset);

                match entries {
                    Ok(entries) => Ok(PakFile { header, entries }),
                    Err(_) => Err("Invalid file entry"),
                }
            }
            Err(error) => Err(error),
        }
    }
}
