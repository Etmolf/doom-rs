use std::path::PathBuf;
use anyhow::Result;
use crate::map::GameMap;
use crate::wad::{Linedef, LumpIndices, Node, Seg, SubSector, Thing, Vertex};
use crate::wad::reader::{Reader, ReadFromBytes, ReadLumpData};

pub struct Loader {
    reader: Reader
}

impl Loader {
    pub fn new(wad_file_path: PathBuf) -> Result<Self> {
        let reader = Reader::new(wad_file_path)?;

        Ok(
            Self {
                reader
            }
        )
    }

    pub fn load_map_data(&mut self, map: &mut GameMap) -> Result<()> {
        let map_index = self.reader.get_lump_index(&map.map_name).unwrap();

        map.vertexes = self.reader.read_lump(
            map_index + LumpIndices::VERTEXES as usize,
            4,
            None
        )?;

        map.calc_map_bounds();

        map.linedefs = self.reader.read_lump(
            map_index + LumpIndices::LINEDEFS as usize,
            14,
            None
        )?;

        map.nodes = self.reader.read_lump(
            map_index + LumpIndices::NODES as usize,
            28,
            None
        )?;

        map.ssectors = self.reader.read_lump(
            map_index + LumpIndices::SSECTORS as usize,
            4,
            None
        )?;

        map.segs = self.reader.read_lump(
            map_index + LumpIndices::SEGS as usize,
            12,
            None
        )?;

        map.things = self.reader.read_lump(
            map_index + LumpIndices::THINGS as usize,
            10,
            None
        )?;

        if map.player.id > 0 {
            if let Some(thing) = map.things.get(map.player.id - 1) {
                map.player.position = thing.position;
                map.player.angle = thing.angle;
            }
        }

        Ok(())
    }
}