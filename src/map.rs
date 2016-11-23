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
