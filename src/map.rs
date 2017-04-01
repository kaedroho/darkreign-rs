use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};


#[derive(Debug, PartialEq)]
pub struct TileLocation {
    pub x: u16,
    pub y: u16,
}


#[derive(Debug, PartialEq)]
pub struct PreciseLocation {
    pub x: u32,
    pub y: u32,
}


impl PreciseLocation {
    #[inline]
    pub fn tile(&self) -> TileLocation {
        TileLocation {
            x: (self.x / 256) as u16,
            y: (self.y / 256) as u16,
        }
    }
}


pub struct MapTile {
    pub kind: u8,  // u4
    pub variant: u8,  // u3
    //elevation: u8,  // u4
    //blend_mask: u8,  // u2
    // shadow?
}


pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<MapTile>,
}


impl Map {
    pub fn read(data: Box<[u8]>) -> Map {
        let mut cursor = Cursor::new(&data);

        // Check magic number
        let mut magic_number: [u8; 4] = [0; 4];
        cursor.read_exact(&mut magic_number).unwrap();

        if &magic_number != b"MAP_" {
            panic!("unrecognised magic number!");
        }

        // Check version
        let version = cursor.read_u32::<LittleEndian>().unwrap();
        if version != 0x300 {
            panic!("unsupported version!");
        }

        // Width and height
        let width = cursor.read_u32::<LittleEndian>().unwrap();
        let height = cursor.read_u32::<LittleEndian>().unwrap();

        let _ = cursor.read_u32::<LittleEndian>().unwrap();

        let mut tiles = Vec::with_capacity(width as usize * height as usize);

        for _ in 0..(width * height) {
            let tileinfo = cursor.read_u16::<LittleEndian>().unwrap();
            let _ = cursor.read_u32::<LittleEndian>().unwrap();

            tiles.push(MapTile {
                kind: (tileinfo & 0xF) as u8,
                variant: ((tileinfo & 0x1C0) >> 6) as u8,
            });
        }

        Map {
            width: width,
            height: height,
            tiles: tiles,
        }
    }
}