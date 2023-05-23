use std::path::PathBuf;
use crate::wad::MapData;
use anyhow::Result;
use crate::bsp::BinarySpacePartitioning;
use crate::player::Player;

pub struct GameContext {
    pub map_data: MapData,
    pub player: Player,
    pub bsp: BinarySpacePartitioning
}

impl GameContext {
    pub fn new(wad_path: PathBuf, map_name: &str) -> Result<Self> {
        let map_data = MapData::new(wad_path, map_name)?;

        let player = Player::new(map_data.things[0]);
        let bsp = BinarySpacePartitioning::new(&map_data);

        let mut context = Self {
            map_data,
            player,
            bsp
        };

        Ok(context)
    }

    pub fn update(&mut self, delta_time: f64) {
        self.player.update(delta_time);
    }
}