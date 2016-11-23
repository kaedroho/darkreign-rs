#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TeamHandle(u8);


#[derive(Debug)]
pub enum TeamColour {
    Orange,
    Red,
    // TODO
}


#[derive(Debug)]
pub enum Faction {
    Civilian,
    FreedomGuard,
    Imperium,
    Togran,
}


impl Default for Faction {
    fn default() -> Faction {
        Faction::Civilian
    }
}


#[derive(Debug)]
pub struct Team {
    handle: TeamHandle,
    name: String,
    faction: Faction,
    colour: TeamColour,
}
