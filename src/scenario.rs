use std::collections::HashMap;

use team::{TeamHandle, Team};
use building::{BuildingHandle, Building};
use unit::{UnitHandle, Unit};


#[derive(Debug)]
pub struct Scenario {
    teams: HashMap<TeamHandle, Team>,
    buildings: HashMap<BuildingHandle, Building>,
    units: HashMap<UnitHandle, Unit>,
}
