use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};


pub struct PackFile {
    index_offset: u32,
    file_count: u32,
    data: Box<[u8]>,
}


impl PackFile {
    pub fn new(data: Box<[u8]>) -> Result<PackFile, String> {
        let (index_offset, file_count) = {
            let mut cursor = Cursor::new(&data);

            // Check magic number
            let magic_number = cursor.read_u32::<LittleEndian>().unwrap();
            if magic_number != 0x47544f42 {
                return Err("Not a valid pack file (invalid magic number)".into());
            }

            // Index offset and file count
            let index_offset = cursor.read_u32::<LittleEndian>().unwrap();
            let file_count = cursor.read_u32::<LittleEndian>().unwrap();

            (index_offset, file_count)
        };

        Ok(PackFile {
            data: data,
            index_offset: index_offset,
            file_count: file_count,
        })
    }

    pub fn iter_entries<'a>(&'a self) -> PackFileEntriesIterator<'a> {
        let mut cursor = Cursor::new(&self.data);
        cursor.set_position(self.index_offset as u64);

        PackFileEntriesIterator {
            archive: self,
            cursor: cursor,
            files_remaining: self.file_count,
        }
    }
}


pub struct PackFileEntry<'a> {
    name: &'a [u8],
    data: &'a [u8],
}


impl<'a> PackFileEntry<'a> {
    pub fn name(&self) -> &'a [u8] {
        self.name
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}


pub struct PackFileEntriesIterator<'a> {
    archive: &'a PackFile,
    cursor: Cursor<&'a Box<[u8]>>,
    files_remaining: u32,
}


impl<'a> Iterator for PackFileEntriesIterator<'a> {
    type Item = PackFileEntry<'a>;

    fn next(&mut self) -> Option<PackFileEntry<'a>> {
        if self.files_remaining > 0 {
            self.files_remaining -= 1;

            // Read name
            let name_start = self.cursor.position() as usize;
            let mut name_end = name_start + 28;
            self.cursor.seek(SeekFrom::Current(28)).unwrap();

            // Strip zero-padding from name
            for (i, c) in self.archive.data[name_start..name_end].iter().enumerate() {
                if *c == 0 {
                    name_end = name_start + i;
                    break;
                }
            }

            // Get data slice
            let data_start = self.cursor.read_u32::<LittleEndian>().unwrap() as usize;
            let data_end = data_start + self.cursor.read_u32::<LittleEndian>().unwrap() as usize;

            Some(PackFileEntry {
                name: &self.archive.data[name_start..name_end],
                data: &self.archive.data[data_start..data_end],
            })
        } else {
            None
        }
    }
}
