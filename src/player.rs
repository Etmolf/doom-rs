use crate::game_context::GameContext;
use crate::wad::{Point, Thing};

pub struct Player {
    thing: Thing,
    pub position: Point,
    pub angle: i16
}

impl Player {
    pub fn new(thing: Thing) -> Self {
        Self {
            thing,
            position: thing.position,
            angle: thing.angle
        }
    }

    pub fn update(&mut self, delta_time: f64) {

    }
}