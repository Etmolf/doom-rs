#[derive(Debug)]
pub struct Header {
    pub wad_type: String,
    pub num_lumps: usize,
    pub info_table_offset: usize
}

#[derive(Debug, Clone)]
pub struct LumpInfo {
    pub offset: usize,
    pub size: usize,
    pub name: String
}

#[derive(Copy, Clone)]
pub enum LumpIndices {
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
pub struct Vertex {
    pub x: i16,
    pub y: i16
}

#[derive(Debug, Copy, Clone)]
pub struct Linedef {
    pub start_vertex_id: u16,
    pub end_vertex_id: u16,
    pub flags: u16,
    pub line_type: u16,
    pub sector_tag: u16,
    pub front_sidedef_id: u16,
    pub back_sidedef_id: u16
}

#[derive(Debug, Default, Copy, Clone)]
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
    pub right_child_id: u16,
    pub left_child_id: u16
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