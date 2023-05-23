use crate::game_context::GameContext;
use crate::player::Player;
use crate::wad::{MapData, Node, Seg, SubSector};

pub const SUB_SECTOR_IDENTIFIER: usize = 0x8000;

pub struct BinarySpacePartitioning {
    pub nodes: Vec<Node>,
    pub ssectors: Vec<SubSector>,
    pub segs: Vec<Seg>,
    pub root_node_id: usize
}

impl BinarySpacePartitioning {
    pub fn new(map_data: &MapData) -> Self {
        Self {
            nodes: map_data.nodes.to_owned(),
            ssectors: map_data.ssectors.to_owned(),
            segs: map_data.segs.to_owned(),
            root_node_id: map_data.nodes.len() - 1,
        }
    }

    pub fn update(&self, context: &GameContext) {

    }
}