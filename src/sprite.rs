use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt};
use image::ColorType;
use image::png::PNGEncoder;

use palette::SNOW_PALETTE;


#[derive(Debug, PartialEq)]
pub enum SpriteType {
    L,
    R,
    Shadow,
}


pub struct SpriteFile<'a> {
    data: &'a [u8],
    sprite_type: SpriteType,
    total_frames: u32,
    num_rotations: u32,
    size_x: u32,
    size_y: u32,
    total_pics: u32,  // total_frames * num_rotations
    num_anims: u32,
    header_size: u32,
}


impl<'a> SpriteFile<'a> {
    pub fn open(data: &'a [u8]) -> SpriteFile<'a> {
        let mut cursor = Cursor::new(&data);

        // Check magic number
        let mut magic_number: [u8; 4] = [0; 4];
        cursor.read_exact(&mut magic_number).unwrap();
        let sprite_type = match &magic_number {
            b"LSPR" => SpriteType::L,
            b"RSPR" => SpriteType::R,
            b"SSPR" => SpriteType::Shadow,
            _ => {
                panic!("unrecognised magic number!");
            }
        };

        // Check version
        let version = cursor.read_u32::<LittleEndian>().unwrap();
        if version != 0x200 && version != 0x210 {
            panic!("unsupported version!");
        }

        // Read rest of header
        let total_frames = cursor.read_u32::<LittleEndian>().unwrap();
        let num_rotations = cursor.read_u32::<LittleEndian>().unwrap();
        let size_x = cursor.read_u32::<LittleEndian>().unwrap();
        let size_y = cursor.read_u32::<LittleEndian>().unwrap();
        let total_pics = cursor.read_u32::<LittleEndian>().unwrap();
        let num_anims = cursor.read_u32::<LittleEndian>().unwrap();

        SpriteFile {
            data: data,
            sprite_type: sprite_type,
            total_frames: total_frames,
            num_rotations: num_rotations,
            size_x: size_x,
            size_y: size_y,
            total_pics: total_pics,
            num_anims: num_anims,
            header_size: cursor.position() as u32,
        }
    }

    fn lookup_pic_offset(&self, pic_index: u32) -> u32 {
        let mut cursor = Cursor::new(&self.data);

        let picindextable_offset = self.header_size;
        let picoffsettable_offset = self.header_size + 4 * self.total_frames * self.num_rotations + 16 * self.num_anims + 4 * self.total_frames;

        cursor.set_position((self.header_size + 4 * pic_index) as u64);
        let pic = cursor.read_u32::<LittleEndian>().unwrap();

        cursor.set_position((picoffsettable_offset + 8 * pic) as u64);
        let offset = cursor.read_u32::<LittleEndian>().unwrap();

        offset
    }

    pub fn write_png<W: Write>(&self, w: W) {
        let out_size_x = self.size_x * self.num_rotations;
        let out_size_y = self.size_y * self.total_frames;
        let mut pixel_data = vec![0; (out_size_x * out_size_y * 4) as usize];

        for rotation in 0..self.num_rotations {
            for frame in 0..self.total_frames {
                let pic_index = frame * self.num_rotations + rotation;
                let pic_offset = self.lookup_pic_offset(pic_index) as u64;

                let picdata_offset = (self.header_size + 4 * self.total_frames * self.num_rotations + 16 * self.num_anims + 4 * self.total_frames + 8 * self.total_pics + 4) as u64;

                let mut cursor = Cursor::new(&self.data);
                cursor.set_position(picdata_offset + pic_offset);
                let mut bytes = cursor.bytes();

                let pic_pos_x = rotation * self.size_x;
                let pic_pos_y = frame * self.size_y;

                for line in 0..self.size_y {
                    let mut step = 0;
                    let mut currx = 0;
                    let mut i = 0;

                    while currx < self.size_x {
                        let mut cnt = bytes.next().unwrap().unwrap() as u32;

                        if step & 1 == 1 {
                            cnt &= 0x7f;

                            match self.sprite_type {
                                SpriteType::L | SpriteType::R => {
                                    for i in 0..cnt {
                                        let px = pic_pos_x + currx + i;
                                        let py = pic_pos_y + line;
                                        let pixel_index = (py * out_size_x + px) as usize;
                                        let pixel_value = bytes.next().unwrap().unwrap() as usize;

                                        pixel_data[pixel_index * 4 + 0] = SNOW_PALETTE[pixel_value * 4 + 0];
                                        pixel_data[pixel_index * 4 + 1] = SNOW_PALETTE[pixel_value * 4 + 1];
                                        pixel_data[pixel_index * 4 + 2] = SNOW_PALETTE[pixel_value * 4 + 2];
                                        pixel_data[pixel_index * 4 + 3] = 255;
                                    }
                                }
                                SpriteType::Shadow => {
                                    for i in 0..cnt {
                                        let px = pic_pos_x + currx + i;
                                        let py = pic_pos_y + line;
                                        let pixel_index = (py * out_size_x + px) as usize;

                                        pixel_data[pixel_index * 4 + 0] = 0;
                                        pixel_data[pixel_index * 4 + 1] = 0;
                                        pixel_data[pixel_index * 4 + 2] = 0;
                                        pixel_data[pixel_index * 4 + 3] = 128;
                                    }
                                }
                            }
                        }

                        currx += cnt;
                        step += 1;
                    }

                    if currx != self.size_x {
                        // Invalid image
                        return;
                    }
                }
            }
        }

        // Encode
        let mut encoder = PNGEncoder::new(w);
        encoder.encode(&pixel_data, out_size_x, out_size_y, ColorType::RGBA(8));
    }
}
