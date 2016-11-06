use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt};
use image::ColorType;
use image::png::PNGEncoder;

use palette::SNOW_PALETTE;


pub struct TileSetFile {
    data: Box<[u8]>,
}


impl TileSetFile {
    pub fn open(data: Box<[u8]>) -> TileSetFile {
        {
            let mut cursor = Cursor::new(&data);

            // Check magic number
            let mut magic_number: [u8; 4] = [0; 4];
            cursor.read_exact(&mut magic_number).unwrap();

            if &magic_number != b"TILE" {
                panic!("unrecognised magic number!");
            }

            // Check version
            let version = cursor.read_u32::<LittleEndian>().unwrap();
            if version != 0x240 {
                panic!("unsupported version!");
            }
        }

        TileSetFile {
            data: data,
        }
    }

    pub fn write_png<W: Write>(&self, w: W) {
        let out_size_x = 24 as usize;
        let out_size_y = self.data.len() / out_size_x;
        let mut pixel_data = vec![0; out_size_x * (out_size_y + 1) * 4];

        let mut cursor = Cursor::new(&self.data);
        cursor.set_position(8);

        let mut bytes = cursor.bytes();

        // Basic tilesets
        for tileset in 0..16 {
            for tile in 0..8 {
                let _ = bytes.next().unwrap().unwrap();
                let sy = (tileset * 8 + tile) * 24;

                for l in 0..24 {
                    for r in 0..24 {
                        let v = bytes.next().unwrap().unwrap();
                        let px = (sy + l) * 24 + r;
                        pixel_data[px * 4 + 0] = SNOW_PALETTE[v as usize * 4 + 0];
                        pixel_data[px * 4 + 1] = SNOW_PALETTE[v as usize * 4 + 1];
                        pixel_data[px * 4 + 2] = SNOW_PALETTE[v as usize * 4 + 2];
                        pixel_data[px * 4 + 3] = 255;
                    }
                }
            }
        }

        // Water animation
        for tileset in 0..8 {
            for tile in 0..8 {
                let sy = ((16 + tileset) * 8 + tile) * 24;

                for l in 0..24 {
                    for r in 0..24 {
                        let v = bytes.next().unwrap().unwrap();
                        let px = (sy + l) * 24 + r;
                        pixel_data[px * 4 + 0] = SNOW_PALETTE[v as usize * 4 + 0];
                        pixel_data[px * 4 + 1] = SNOW_PALETTE[v as usize * 4 + 1];
                        pixel_data[px * 4 + 2] = SNOW_PALETTE[v as usize * 4 + 2];
                        pixel_data[px * 4 + 3] = 255;
                    }
                }
            }
        }

        // Land-sea blending tiles
        for tileset in 0..4 {
            for tile in 0..16 {
                let _ = bytes.next().unwrap().unwrap();
                let sy = ((tileset) * 17 + 24 * 8 + 5 + tile) * 24;

                for l in 0..24 {
                    for r in 0..24 {
                        let v = bytes.next().unwrap().unwrap();
                        let px = (sy + l) * 24 + r;
                        pixel_data[px * 4 + 0] = SNOW_PALETTE[v as usize * 4 + 0];
                        pixel_data[px * 4 + 1] = SNOW_PALETTE[v as usize * 4 + 1];
                        pixel_data[px * 4 + 2] = SNOW_PALETTE[v as usize * 4 + 2];
                        pixel_data[px * 4 + 3] = 255;
                    }
                }
            }
        }

        // TODO masks


/*
        let mut px = 0;
        let mut skip = 0;


        for b in cursor.bytes() {
            let v = b.unwrap();
            if skip > 0 {
                skip -= 1;
                continue;
            }
            pixel_data[px * 4 + 0] = SNOW_PALETTE[v as usize * 4 + 0];
            pixel_data[px * 4 + 1] = SNOW_PALETTE[v as usize * 4 + 1];
            pixel_data[px * 4 + 2] = SNOW_PALETTE[v as usize * 4 + 2];
            pixel_data[px * 4 + 3] = 255;

            if px % (24 * 24) == 0 {
                skip = 1;
            }

            px += 1;
        }
*/

        // Encode
        let mut encoder = PNGEncoder::new(w);
        encoder.encode(&pixel_data, out_size_x as u32, out_size_y as u32, ColorType::RGBA(8));
    }
}
