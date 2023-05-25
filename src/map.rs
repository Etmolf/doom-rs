use std::time::Duration;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::sys::{u_int16_t, Window};
use crate::Player;
use crate::wad::{BoundingBox, Linedef, Node, Point, Seg, SubSector, Thing, Vertex};

const SUB_SECTOR_IDENTIFIER: u16 = 0x8000;

#[derive(Debug, Default)]
pub struct MapBounds {
    pub min_x: i16,
    pub max_x: i16,
    pub min_y: i16,
    pub max_y: i16
}

#[derive(Default)]
pub struct GameMap {
    pub map_name: String,
    pub player: Player,
    pub vertexes: Vec<Vertex>,
    pub linedefs: Vec<Linedef>,
    pub nodes: Vec<Node>,
    pub ssectors: Vec<SubSector>,
    pub segs: Vec<Seg>,
    pub things: Vec<Thing>,
    pub bounds: MapBounds,
    pub automap_scale_factor: i16,
    render_w: u32,
    render_h: u32
}

impl GameMap {
    pub fn new(map_name: &str, player: Player, canvas: &WindowCanvas) -> Self {

        let (mut render_w, mut render_h) = canvas.logical_size();

        Self {
            map_name: map_name.to_string(),
            player,
            automap_scale_factor: 15,
            render_w,
            render_h,
            ..Default::default()
        }
    }

    pub fn calc_map_bounds(&mut self) {
        self.bounds.min_x = self.vertexes.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        self.bounds.max_x = self.vertexes.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        self.bounds.min_y = self.vertexes.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        self.bounds.max_y = self.vertexes.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    }

    fn is_point_on_left_side(&self, point: Point, node_id: usize) -> bool {
        let dx: i32 = (point.x - self.nodes[node_id].x_partition) as i32;
        let dy: i32 = (point.y - self.nodes[node_id].y_partition) as i32;

        ((dx * self.nodes[node_id].dy_partition as i32) - (dy * self.nodes[node_id].dx_partition as i32)) <= 0
    }

    fn render_bsp_node(&self, canvas: &mut WindowCanvas, node_id: u16) {
        if node_id >= SUB_SECTOR_IDENTIFIER {
            let sub_sector_id = (node_id - SUB_SECTOR_IDENTIFIER) as usize;
            self.render_sub_sector(canvas, sub_sector_id);
            return;
        }

        let node = self.nodes[node_id as usize];

        if self.is_point_on_left_side(self.player.position, node_id as usize) {
            self.render_bsp_node(canvas, node.left_child_id);
            self.render_bsp_node(canvas, node.right_child_id);
        } else {
            self.render_bsp_node(canvas, node.right_child_id);
            self.render_bsp_node(canvas, node.left_child_id);
        }
    }

    fn render_sub_sector(&self, canvas: &mut WindowCanvas, sub_sector_id: usize) {
        let sub_sector = self.ssectors[sub_sector_id];

        for i in 0..sub_sector.seg_count {
            let seg_id = (sub_sector.first_seg_id + i) as usize;
            self.render_seg(canvas, seg_id, sub_sector_id);
        }
    }

    fn render_seg(&self, canvas: &mut WindowCanvas, seg_id: usize, sub_sector_id: usize) {
        let seg = self.segs[seg_id];
        let sub_sector = self.ssectors[sub_sector_id];

        let v1 = self.vertexes[seg.start_vertex_id as usize];
        let v2 = self.vertexes[seg.end_vertex_id as usize];

        canvas.line(
            self.remap_x(v1.x),
            self.remap_y(v1.y),
            self.remap_x(v2.x),
            self.remap_y(v2.y),
            self.get_color(sub_sector_id as u64)
        );
    }

    fn get_color(&self, seed: u64) -> Color {
        let mut rng = Pcg32::seed_from_u64(seed);

        let r: u8 = rng.gen_range(100..=255);
        let g: u8 = rng.gen_range(100..=255);
        let b: u8 = rng.gen_range(100..=255);

        Color::RGB(r, g, b)
    }

    fn remap_x(&self, x: i16) -> i16 {
        (x + (-self.bounds.min_x)) / self.automap_scale_factor
    }

    fn remap_y(&self, y: i16) -> i16 {
        self.render_h as i16 - (y + (-self.bounds.min_y)) / self.automap_scale_factor
    }

    pub fn render_automap(&self, canvas: &mut WindowCanvas) {
        self.render_automap_player(canvas);
        self.render_automap_walls(canvas);

        let root_node_id = (self.nodes.len() - 1) as usize;
        self.render_bsp_node(canvas, root_node_id as u16);
    }

    fn render_automap_player(&self, canvas: &WindowCanvas) {
        canvas.filled_circle(
            self.remap_x(self.player.position.x),
            self.remap_y(self.player.position.y),
            2,
            Color::RED
        );
    }

    fn render_automap_walls(&self, canvas: &WindowCanvas) {
        for linedef in &self.linedefs {
            let start = self.vertexes[linedef.start_vertex_id as usize];
            let end = self.vertexes[linedef.end_vertex_id as usize];

            canvas.line(
                self.remap_x(start.x),
                self.remap_y(start.y),
                self.remap_x(end.x),
                self.remap_y(end.y),
                Color::RGB(70, 70, 70)
            );
        }
    }

    fn render_automap_node(&self, canvas: &WindowCanvas, root_node_id: usize) {
        let node = self.nodes[root_node_id];

        self.render_bbox(canvas, node.bbox_left, Color::RED);
        self.render_bbox(canvas, node.bbox_right, Color::GREEN);

        canvas.line(
            self.remap_x(node.x_partition),
            self.remap_y(node.y_partition),
            self.remap_x(node.x_partition + node.dx_partition),
            self.remap_y(node.y_partition + node.dy_partition),
            Color::BLUE
        );
    }

    fn render_bbox(&self, canvas: &WindowCanvas, bbox: BoundingBox, color: Color) {
        let x = self.remap_x(bbox.left);
        let y = self.remap_y(bbox.top);
        let w = self.remap_x(bbox.right);
        let h = self.remap_y(bbox.bottom);

        canvas.rectangle(x, y, w, h, color);
    }
}