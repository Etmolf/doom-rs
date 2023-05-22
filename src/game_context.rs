use crate::wad::MapData;
use anyhow::Result;

pub struct GameContext {
    pub map_data: MapData
}

impl GameContext {
    pub fn new(map_data: MapData) -> Result<Self> {
        Ok(Self { map_data })
    }

    pub fn update(&mut self, delta_time: f64) {

    }
}