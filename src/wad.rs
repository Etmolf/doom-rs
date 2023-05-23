use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Index;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug)]
pub struct Header {
    wad_type: String,
    num_lumps: usize,
    info_table_offset: usize
}

#[derive(Debug, Clone)]
pub struct LumpInfo {
    offset: usize,
    size: usize,
    name: String
}

#[derive(Copy, Clone)]
enum LumpIndices {
    THINGS = 1,
    LINEDEFS = 2,
    SIDEDEFS = 3,
    VERTEXES = 4,
    SEGS = 5,
    SSECTORS = 6,
    NODES = 7,
    SECTORS = 8,
    REJECT = 9,
    BLOCKMAP = 10
}

#[derive(Debug, Copy, Clone)]
pub struct Linedef {
    pub start_vertex_id: i16,
    pub end_vertex_id: i16,
    pub flags: i16,
    pub line_type: i16,
    pub sector_tag: i16,
    pub front_sidedef_id: i16,
    pub back_sidedef_id: i16
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16
}

#[derive(Debug, Copy, Clone)]
pub struct Thing {
    pub position: Point,
    pub angle: i16,
    pub ed_type: i16,
    pub flags: i16
}

#[derive(Debug, Copy, Clone)]
pub struct Seg {
    pub start_vertex_id: i16,
    pub end_vertex_id: i16,
    pub angle: i16,
    pub linedef_id: i16,
    pub direction: i16,
    pub offset: i16
}

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub x_partition: i16,
    pub y_partition: i16,
    pub dx_partition: i16,
    pub dy_partition: i16,
    pub bbox_right: BoundingBox,
    pub bbox_left: BoundingBox,
    pub right_child_id: i16,
    pub left_child_id: i16
}

#[derive(Debug, Copy, Clone)]
pub struct SubSector {
    pub seg_count: i16,
    pub first_seg_id: i16
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub top: i16,
    pub bottom: i16,
    pub left: i16,
    pub right: i16
}

#[derive(Debug, Clone)]
pub struct MapData {
    pub map_index: usize,
    pub vertexes: Vec<Vertex>,
    pub linedefs: Vec<Linedef>,
    pub nodes: Vec<Node>,
    pub ssectors: Vec<SubSector>,
    pub segs: Vec<Seg>,
    pub things: Vec<Thing>
}

impl MapData {
    pub fn new(wad_path: PathBuf, map_name: &str) -> Result<Self> {
        let mut reader = Reader::new(wad_path)?;
        let map_index = reader.get_lump_index(map_name).unwrap();

        let vertexes: Vec<Vertex> = reader.read_lump(
            map_index + LumpIndices::VERTEXES as usize,
            4,
            None
        )?;

        let linedefs: Vec<Linedef> = reader.read_lump(
            map_index + LumpIndices::LINEDEFS as usize,
            14,
            None
        )?;

        let nodes: Vec<Node> = reader.read_lump(
           map_index + LumpIndices::NODES as usize,
            28,
            None
        )?;

        let ssectors: Vec<SubSector> = reader.read_lump(
           map_index + LumpIndices::SSECTORS as usize,
            4,
            None
        )?;

        let segs: Vec<Seg> = reader.read_lump(
           map_index + LumpIndices::SEGS as usize,
            12,
            None
        )?;

        let things: Vec<Thing> = reader.read_lump(
            map_index + LumpIndices::THINGS as usize,
            10,
            None
        )?;

        Ok(Self {
            map_index,
            vertexes,
            linedefs,
            nodes,
            ssectors,
            segs,
            things
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: i16,
    pub y: i16
}

pub struct Reader {
    file: File,
    pub header: Option<Header>,
    pub directory: Vec<LumpInfo>
}

impl Reader {
    pub fn new(file_path: PathBuf) -> Result<Reader> {
        let file = File::open(file_path)?;

        let mut reader = Self {
            file,
            header: None,
            directory: Vec::new()
        };

        reader.load()?;

        Ok(reader)
    }

    pub fn load(&mut self) -> Result<()> {
        self.header = Some(self.read_header()?);
        self.directory = self.read_directory()?;

        Ok(())
    }

    pub fn read_directory(&mut self) -> Result<Vec<LumpInfo>> {
        let header = self.read_header()?;
        let mut directory: Vec<LumpInfo> = Vec::new();

        for i in 0..header.num_lumps {
            let offset: usize = header.info_table_offset + i * 16;

            let lump = LumpInfo {
                offset: self.read(offset, 4)?,
                size: self.read(offset + 4, 4)?,
                name: self.read(offset + 8, 8)?,
            };

            directory.push(lump);
        }

        Ok(directory)
    }

    pub fn read_header(&mut self) -> Result<Header> {
        let header = Header {
            wad_type: self.read(0, 4)?,
            num_lumps: self.read(4, 4)?,
            info_table_offset: self.read(8, 4)?,
        };

        Ok(header)
    }

    pub fn get_lump_index(&self, lump_name: &str) -> Option<usize> {
        self.directory.iter().position(|info| info.name == lump_name)
    }

    fn read_bytes(&mut self, offset: usize, num_bytes: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; num_bytes];

        self.file.seek(SeekFrom::Start(offset as u64))?;
        self.file.read(buffer.as_mut_slice())?;

        Ok(buffer.to_owned())
    }
}

pub trait ReadFromBytes<T> {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<T>;
}

impl ReadFromBytes<i32> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<i32> {
        let mut bytes = self.read_bytes(offset, num_bytes)?;
        Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl ReadFromBytes<i16> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<i16> {
        let mut bytes = self.read_bytes(offset, num_bytes)?;
        Ok(i16::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl ReadFromBytes<usize> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<usize> {
        let bytes = self.read_bytes(offset, num_bytes)?;
        Ok(i32::from_le_bytes(bytes.try_into().unwrap()) as usize)
    }
}

impl ReadFromBytes<String> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<String> {
        let bytes = self.read_bytes(offset, num_bytes)?;

        let string = String::from_utf8(bytes)?
            .trim_matches(char::from(0))
            .to_string();

        Ok(string)
    }
}

impl ReadFromBytes<Vertex> for Reader {
    fn read(&mut self, offset: usize, _num_bytes: usize) -> Result<Vertex> {
        let x: i16 = self.read(offset, 2)?;
        let y: i16 = self.read(offset + 2, 2)?;
        Ok(Vertex { x, y })
    }
}

impl ReadFromBytes<Linedef> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<Linedef> {
        Ok(Linedef {
            start_vertex_id: self.read(offset, 2)?,
            end_vertex_id: self.read(offset + 2, 2)?,
            flags: self.read(offset + 4, 2)?,
            line_type: self.read(offset + 6, 2)?,
            sector_tag: self.read(offset + 8, 2)?,
            front_sidedef_id: self.read(offset + 10, 2)?,
            back_sidedef_id: self.read(offset + 12, 2)?
        })
    }
}

impl ReadFromBytes<Node> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<Node> {
        Ok(Node {
            x_partition: self.read(offset, 2)?,
            y_partition: self.read(offset + 2, 2)?,
            dx_partition: self.read(offset + 4, 2)?,
            dy_partition: self.read(offset + 6, 2)?,
            bbox_right: BoundingBox {
                top: self.read(offset + 8, 2)?,
                bottom: self.read(offset + 10, 2)?,
                left: self.read(offset + 12, 2)?,
                right: self.read(offset + 14, 2)?,
            },
            bbox_left: BoundingBox {
                top: self.read(offset + 16, 2)?,
                bottom: self.read(offset + 18, 2)?,
                left: self.read(offset + 20, 2)?,
                right: self.read(offset + 22, 2)?,
            },
            right_child_id: self.read(offset + 24, 2)?,
            left_child_id: self.read(offset + 26, 2)?
        })
    }
}

impl ReadFromBytes<SubSector> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<SubSector> {
        Ok(SubSector {
            seg_count: self.read(offset, 2)?,
            first_seg_id: self.read(offset + 2, 2)?,
        })
    }
}

impl ReadFromBytes<Seg> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<Seg> {
        Ok(Seg {
            start_vertex_id: self.read(offset, 2)?,
            end_vertex_id: self.read(offset + 2, 2)?,
            angle: self.read(offset + 4, 2)?,
            linedef_id: self.read(offset + 6, 2)?,
            direction: self.read(offset + 8, 2)?,
            offset: self.read(offset + 10, 2)?,
        })
    }
}

impl ReadFromBytes<Thing> for Reader {
    fn read(&mut self, offset: usize, num_bytes: usize) -> Result<Thing> {
        Ok(Thing {
            position: Point {
                x: self.read(offset, 2)?,
                y: self.read(offset + 2, 2)?
            },
            angle: self.read(offset + 4, 2)?,
            ed_type: self.read(offset + 6, 2)?,
            flags: self.read(offset + 8, 2)?,
        })
    }
}

pub trait ReadLumpData<T> {
    fn read_lump(&mut self, lump_index: usize, num_bytes: usize, header_length: Option<usize>) -> Result<T>;
}

impl ReadLumpData<Vec<Vertex>> for Reader {
    fn read_lump(&mut self, lump_index: usize, num_bytes: usize, header_length: Option<usize>) -> Result<Vec<Vertex>> {
        let lump_info = self.directory.get(lump_index).unwrap().clone();
        let mut vertexes: Vec<Vertex> = Vec::new();

        let total_count = lump_info.size / num_bytes;

        for i in 0..total_count {
            let offset = lump_info.offset + i * num_bytes + header_length.unwrap_or_default();
            let vertex: Vertex = self.read(offset, num_bytes)?;
            vertexes.push(vertex);
        }

        Ok(vertexes)
    }
}

impl ReadLumpData<Vec<Linedef>> for Reader {
    fn read_lump(&mut self, lump_index: usize, num_bytes: usize, header_length: Option<usize>) -> Result<Vec<Linedef>> {
        let lump_info = self.directory.get(lump_index).unwrap().clone();
        let mut linedefs: Vec<Linedef> = Vec::new();

        let total_count = lump_info.size / num_bytes;

        for i in 0..total_count {
            let offset = lump_info.offset + i * num_bytes + header_length.unwrap_or_default();
            let linedef: Linedef = self.read(offset, 12)?;
            linedefs.push(linedef);
        }

        Ok(linedefs)
    }
}

impl ReadLumpData<Vec<Node>> for Reader {
    fn read_lump(&mut self, lump_index: usize, num_bytes: usize, header_length: Option<usize>) -> Result<Vec<Node>> {
        let lump_info = self.directory.get(lump_index).unwrap().clone();
        let mut nodes: Vec<Node> = Vec::new();

        let total_count = lump_info.size / num_bytes;

        for i in 0..total_count {
            let offset = lump_info.offset + i * num_bytes + header_length.unwrap_or_default();
            let node: Node = self.read(offset, 24)?;
            nodes.push(node);
        }

        Ok(nodes)
    }
}

impl ReadLumpData<Vec<SubSector>> for Reader {
    fn read_lump(&mut self, lump_index: usize, num_bytes: usize, header_length: Option<usize>) -> Result<Vec<SubSector>> {
        let lump_info = self.directory.get(lump_index).unwrap().clone();
        let mut ssectors: Vec<SubSector> = Vec::new();

        let total_count = lump_info.size / num_bytes;

        for i in 0..total_count {
            let offset = lump_info.offset + i * num_bytes + header_length.unwrap_or_default();
            let ssector: SubSector = self.read(offset, 4)?;
            ssectors.push(ssector);
        }

        Ok(ssectors)
    }
}

impl ReadLumpData<Vec<Seg>> for Reader {
    fn read_lump(&mut self, lump_index: usize, num_bytes: usize, header_length: Option<usize>) -> Result<Vec<Seg>> {
        let lump_info = self.directory.get(lump_index).unwrap().clone();
        let mut segs: Vec<Seg> = Vec::new();

        let total_count = lump_info.size / num_bytes;

        for i in 0..total_count {
            let offset = lump_info.offset + i * num_bytes + header_length.unwrap_or_default();
            let seg: Seg = self.read(offset, 4)?;
            segs.push(seg);
        }

        Ok(segs)
    }
}

impl ReadLumpData<Vec<Thing>> for Reader {
    fn read_lump(&mut self, lump_index: usize, num_bytes: usize, header_length: Option<usize>) -> Result<Vec<Thing>> {
        let lump_info = self.directory.get(lump_index).unwrap().clone();
        let mut things: Vec<Thing> = Vec::new();

        let total_count = lump_info.size / num_bytes;

        for i in 0..total_count {
            let offset = lump_info.offset + i * num_bytes + header_length.unwrap_or_default();
            let thing: Thing = self.read(offset, 10)?;
            things.push(thing);
        }

        Ok(things)
    }
}