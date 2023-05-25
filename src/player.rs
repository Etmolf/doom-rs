use crate::wad::Point;

#[derive(Debug, Default, Copy, Clone)]
pub struct Player {
    pub id: usize,
    pub position: Point,
    pub angle: i16
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}