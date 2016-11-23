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
    FreedomGuard,
    Imperium,
    Togran,
}


#[derive(Debug)]
pub struct Team {
    handle: TeamHandle,
    name: String,
    faction: Faction,
    colour: TeamColour,
}
