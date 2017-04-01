extern crate piston_window;

extern crate darkreign;

use std::fs::File;
use std::io::Read;

use piston_window::*;

use darkreign::palette::BARREN_PALETTE;
use darkreign::tileset::TileSet;
use darkreign::map::Map;


fn main() {
    let mut file = File::open("BARREN.TIL").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let ts = TileSet::read(contents.into_boxed_slice(), BARREN_PALETTE).unwrap();

    let mut file = File::open("8burnout.map").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let map = Map::read(contents.into_boxed_slice());

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            let mut i = 0;
            for map_tile in map.tiles.iter() {
                let column = i % map.width as usize;
                let row = i / map.height as usize;

                if column > 32 || row > 32 {
                    i += 1;
                    continue;
                }

                let tile = ts.tiles.get((map_tile.kind as usize * 8 + map_tile.variant as usize)).unwrap();

                for px in 0..24 {
                    for py in 0..24 {
                        let v = tile.data[py * 24 + px] as usize;
                        let red = ts.palette[v * 4 + 0] as f32 / 255.0;
                        let green = ts.palette[v * 4 + 1] as f32 / 255.0;
                        let blue = ts.palette[v * 4 + 2] as f32 / 255.0;

                        rectangle([red, green, blue, 1.0f32],
                                [(px + column * 24) as f64, (py + row * 24) as f64, 1.0, 1.0],
                                c.transform, g);
                    }
                }
                i += 1;
            }

        });
    }
}
