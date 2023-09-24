use std::fs::File;

pub struct PakFile {
    header: PakHeader,
}
impl PakFile {
    pub fn from_file_handle(file: &File) -> PakFile {
        PakFile {
            header: PakHeader {
                id: [b'P', b'A', b'C', b'K'],
                offset: 0,
                size: 0,
            },
        }
    }
}

pub struct PakHeader {
    id: [u8; 4],
    offset: i32,
    size: i32,
}
