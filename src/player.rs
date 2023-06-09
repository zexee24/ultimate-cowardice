use crate::position::Position;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub champion: String,
}
