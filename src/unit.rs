use std::rc::Rc;

use map::{TileLocation, PreciseLocation};
use team::{TeamHandle, Faction};
use building::BuildingHandle;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitHandle(u32);


#[derive(Debug)]
pub struct UnitClass {
    name: String,
    faction: Faction,
}


#[derive(Debug)]
pub enum UnitCommand {
    Stand,
    Scout,
    Harass,
    SearchAndDestroy,
    MoveTo(TileLocation),
    AttackUnit(UnitHandle),
    AttackBuilding(BuildingHandle),
    AttackGround(PreciseLocation),
}


impl Default for UnitCommand {
    fn default() -> UnitCommand {
        UnitCommand::Stand
    }
}


#[derive(Debug)]
pub struct UnitBehaviour {
    pursuit_range: u8,
    damage_tolerance: u8,
    independance: u8,
}


#[derive(Debug)]
pub struct Unit {
    handle: UnitHandle,
    class: Rc<UnitClass>,
    team: TeamHandle,
    location: PreciseLocation,
    command: UnitCommand,
    behaviour: UnitBehaviour,
}
