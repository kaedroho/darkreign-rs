use std::rc::Rc;

use map::TileLocation;
use team::{TeamHandle, Faction};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BuildingHandle(u32);


#[derive(Debug)]
pub struct BuildingClass {
    name: String,
    faction: Faction,
}


#[derive(Debug)]
pub struct Building {
    handle: BuildingHandle,
    class: Rc<BuildingClass>,
    team: TeamHandle,
    location: TileLocation,
}
