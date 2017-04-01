use std::io::{Cursor, Read, Seek, SeekFrom, Error};

use byteorder::{LittleEndian, ReadBytesExt};


pub struct Tile {
    pub data: [u8; 24 * 24],
}


pub struct TileSet {
    pub palette: &'static [u8],
    pub tiles: Vec<Tile>,
}


impl TileSet {
    pub fn read(data: Box<[u8]>, palette: &'static [u8]) -> Result<TileSet, Error> {
        let mut tiles = Vec::with_capacity(8 * 16 + 8 * 8 + 16 * 4);
        let mut cursor = Cursor::new(&data);

        // Check magic number
        let mut magic_number: [u8; 4] = [0; 4];
        cursor.read_exact(&mut magic_number)?;

        if &magic_number != b"TILE" {
            panic!("unrecognised magic number!");
        }

        // Check version
        let version = cursor.read_u32::<LittleEndian>()?;
        if version != 0x240 {
            panic!("unsupported version!");
        }

        // Basic tilesets
        for _ts in 0..16 {
            for _tile in 0..8 {
                // Skip 1 byte
                // TODO: Find out why there's an extra byte here
                cursor.seek(SeekFrom::Current(1))?;

                let mut data: [u8; 24 * 24] = [0; 24 * 24];
                cursor.read_exact(&mut data)?;
                tiles.push(Tile {
                    data: data,
                });
            }
        }

        // Water animation
        for _ts in 0..8 {
            for _tile in 0..8 {
                let mut data: [u8; 24 * 24] = [0; 24 * 24];
                cursor.read_exact(&mut data)?;
                tiles.push(Tile {
                    data: data,
                });
            }
        }

        // Land-sea blending tiles
        for _ts in 0..4 {
            for _tile in 0..16 {
                // Skip 1 byte
                // TODO: Find out why there's an extra byte here
                cursor.seek(SeekFrom::Current(1))?;

                let mut data: [u8; 24 * 24] = [0; 24 * 24];
                cursor.read_exact(&mut data)?;
                tiles.push(Tile {
                    data: data,
                });
            }
        }

        Ok(TileSet {
            palette: palette,
            tiles: tiles,
        })
    }
}
