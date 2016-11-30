use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt};
use image::ColorType;
use image::png::PNGEncoder;

use palette::SNOW_PALETTE;


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


pub struct MapFile {
    data: Box<[u8]>,
}


impl MapFile {
    pub fn open(data: Box<[u8]>) -> MapFile {
        {
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

            println!("{}, {}", width, height);

            let foo = cursor.read_u32::<LittleEndian>().unwrap();
            println!("{}", foo);

            // Then read 6*width*height bytes assume this is map tile data


            // 9400 bytes left. no idea what it is
        }

        MapFile {
            data: data,
        }
    }

    pub fn write_png<W: Write>(&self, w: W) {
        let out_size_x = 112 as usize;
        let out_size_y = 6 * 112 as usize;
        let mut pixel_data = vec![0; out_size_x * (out_size_y + 1) * 4];

        {
            let mut cursor = Cursor::new(&self.data);
            cursor.set_position(20);

            let mut bytes = cursor.bytes();

            for i in 0..(112*112) {
                let a = bytes.next().unwrap().unwrap();
                let b = bytes.next().unwrap().unwrap();
                let c = bytes.next().unwrap().unwrap();
                let d = bytes.next().unwrap().unwrap();
                let e = bytes.next().unwrap().unwrap();
                let f = bytes.next().unwrap().unwrap();
    /*
                pixel_data[4 * 112 * 112 * 0 + i * 4 + 0] = a;
                pixel_data[4 * 112 * 112 * 0 + i * 4 + 1] = a;
                pixel_data[4 * 112 * 112 * 0 + i * 4 + 2] = a;
                pixel_data[4 * 112 * 112 * 0 + i * 4 + 3] = 255;

                pixel_data[4 * 112 * 112 * 1 + i * 4 + 0] = b * 64;
                pixel_data[4 * 112 * 112 * 1 + i * 4 + 1] = b * 64;
                pixel_data[4 * 112 * 112 * 1 + i * 4 + 2] = b * 64;
                pixel_data[4 * 112 * 112 * 1 + i * 4 + 3] = 255;

                pixel_data[4 * 112 * 112 * 2 + i * 4 + 0] = c * 16;
                pixel_data[4 * 112 * 112 * 2 + i * 4 + 1] = c * 16;
                pixel_data[4 * 112 * 112 * 2 + i * 4 + 2] = c * 16;
                pixel_data[4 * 112 * 112 * 2 + i * 4 + 3] = 255;

                pixel_data[4 * 112 * 112 * 3 + i * 4 + 0] = d;
                pixel_data[4 * 112 * 112 * 3 + i * 4 + 1] = d;
                pixel_data[4 * 112 * 112 * 3 + i * 4 + 2] = d;
                pixel_data[4 * 112 * 112 * 3 + i * 4 + 3] = 255;

                pixel_data[4 * 112 * 112 * 4 + i * 4 + 0] = e;
                pixel_data[4 * 112 * 112 * 4 + i * 4 + 1] = e;
                pixel_data[4 * 112 * 112 * 4 + i * 4 + 2] = e;
                pixel_data[4 * 112 * 112 * 4 + i * 4 + 3] = 255;

                pixel_data[4 * 112 * 112 * 5 + i * 4 + 0] = f;
                pixel_data[4 * 112 * 112 * 5 + i * 4 + 1] = f;
                pixel_data[4 * 112 * 112 * 5 + i * 4 + 2] = f;
                pixel_data[4 * 112 * 112 * 5 + i * 4 + 3] = 255;
    */
            }
        }

        let mut cursor = Cursor::new(&self.data);
        cursor.set_position(20 + 112 * 112 * 6 + 16);

        let mut csum = 0;

        for i in 0..1173 {
            let a = cursor.read_u16::<LittleEndian>().unwrap();
            let b = cursor.read_u16::<LittleEndian>().unwrap();
            let c = cursor.read_u16::<LittleEndian>().unwrap();
            let d = cursor.read_u16::<LittleEndian>().unwrap();

            csum += c as u32;
            println!("{} {} {} {}", a, b, c, d);
        }

        println!("{:?}", csum);




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
