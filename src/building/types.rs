use std::rc::Rc;

use map::TileLocation;
use team::{TeamHandle, Faction};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BuildingHandle(u32);


#[derive(Debug)]
pub struct BuildingType {
    name: String,
    faction: Faction,
}


impl BuildingType {
    pub fn new<S: Into<String>>(name: S) -> BuildingType {
        BuildingType {
            name: name.into(),
            faction: Faction::default(),
        }
    }

    pub fn set_faction(&mut self, faction: Faction) {
        self.faction = faction;
    }
}


#[derive(Debug)]
pub struct Building {
    handle: BuildingHandle,
    building_type: Rc<BuildingType>,
    team: TeamHandle,
    location: TileLocation,
}
