use std::rc::Rc;

use map::TileLocation;
use team::{TeamHandle, Faction};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BuildingHandle(u32);


#[derive(Debug)]
pub struct BuildingType {
    name: String,
    base_image: String,
    top_image: String,
    menu_image: String,
    description: String,
    faction: Faction,
}


impl BuildingType {
    pub fn new<S: Into<String>>(name: S) -> BuildingType {
        BuildingType {
            name: name.into(),
            base_image: String::new(),
            top_image: String::new(),
            menu_image: String::new(),
            description: String::new(),
            faction: Faction::default(),
        }
    }

    pub fn set_images(&mut self, base: String, top: String, menu: String) {
        self.base_image = base;
        self.top_image = top;
        self.menu_image = menu;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
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
