use std::cmp::{max, min};
use std::time::Duration;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::sys::Window;
use rand_pcg::Pcg32;
use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::bsp::BinarySpacePartitioning;
use crate::bsp::SUB_SECTOR_IDENTIFIER;
use crate::game_context::GameContext;
use crate::renderer::Viewport;
use crate::wad::{BoundingBox, Linedef, MapData, Node, Point, Vertex};

#[derive(Debug)]
struct MapBounds {
    pub min_x: i16,
    pub max_x: i16,
    pub min_y: i16,
    pub max_y: i16
}

pub struct MapRenderer {
    map_data: MapData,
    map_bounds: MapBounds,
    viewport: Viewport
}

impl MapRenderer {
    pub fn new(viewport: Viewport, map_data: MapData) -> MapRenderer {
        let map_bounds = MapBounds {
            min_x: map_data.vertexes.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
            max_x: map_data.vertexes.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
            min_y: map_data.vertexes.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
            max_y: map_data.vertexes.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
        };

        let mut map_renderer = Self {
            map_data,
            map_bounds,
            viewport
        };

        map_renderer.remap_vertexes();
        map_renderer
    }

    fn get_color(&self, seed: u64) -> Color {
        let mut rng = Pcg32::seed_from_u64(seed);

        let r: u8 = rng.gen_range(100..=255);
        let g: u8 = rng.gen_range(100..=255);
        let b: u8 = rng.gen_range(100..=255);

        Color::RGB(r, g, b)
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas, context: &GameContext) {
        let bsp_root_node_id = context.bsp.root_node_id;

        self.draw_linedefs(canvas);
        self.draw_player_pos(canvas, context);
        self.draw_node(canvas, bsp_root_node_id);
        //self.render_bsp_node(canvas, context, bsp_root_node_id);
    }

    fn draw_bbox(&self, canvas: &WindowCanvas, bbox: BoundingBox, color: Color) {
        let x = self.remap_x(bbox.left);
        let y = self.remap_y(bbox.top);
        let w = self.remap_x(bbox.right);
        let h = self.remap_y(bbox.bottom);

        canvas.rectangle(x, y, w, h, color);
    }

    fn render_bsp_node(&self, canvas: &mut WindowCanvas, context: &GameContext, node_id: usize) {
        if node_id >= SUB_SECTOR_IDENTIFIER {
            let sub_sector_id = node_id - SUB_SECTOR_IDENTIFIER;
            self.render_sub_sector(canvas, sub_sector_id);
            return;
        }

        let node = &self.map_data.nodes[node_id];

        if self.bsp_node_is_on_left_side(node, context) {
            self.render_bsp_node(canvas, context, node.left_child_id as usize);
            self.render_bsp_node(canvas, context, node.right_child_id as usize);
        } else {
            self.render_bsp_node(canvas, context, node.right_child_id as usize);
            self.render_bsp_node(canvas, context, node.left_child_id as usize);
        }
    }

    fn render_sub_sector(&self, canvas: &mut WindowCanvas, sub_sector_id: usize) {
        let sub_sector = self.map_data.ssectors[sub_sector_id];

        for i in 0..sub_sector.seg_count {
            let seg_id = (sub_sector.first_seg_id + i) as usize;
            self.draw_seg(canvas, seg_id, sub_sector_id);
        }
    }

    fn bsp_node_is_on_left_side(&self, node: &Node, context: &GameContext) -> bool {
        let dx = (context.player.position.x - node.x_partition) as i32;
        let dy = (context.player.position.y - node.y_partition) as i32;

        dx * node.dy_partition as i32 - dy * node.dx_partition as i32 <= 0
    }

    fn draw_node(&self, canvas: &WindowCanvas, node_id: usize) {
        let node = self.map_data.nodes[node_id];

        self.draw_bbox(canvas,node.bbox_right, Color::GREEN);
        self.draw_bbox(canvas, node.bbox_left, Color::RED);

        let x1 = self.remap_x(node.x_partition);
        let y1 = self.remap_y(node.y_partition);
        let x2 = self.remap_x(node.x_partition + node.dx_partition);
        let y2 = self.remap_y(node.y_partition + node.dy_partition);

        canvas.thick_line(x1, y1, x2, y2, 4, Color::BLUE);
    }

    fn draw_seg(&self, canvas: &mut WindowCanvas, seg_id: usize, sub_sector_id: usize) {
        let seg = self.map_data.segs[seg_id];
        let sub_sector = self.map_data.ssectors[sub_sector_id];

        let v1 = self.map_data.vertexes[seg.start_vertex_id as usize];
        let v2 = self.map_data.vertexes[seg.end_vertex_id as usize];

        canvas.thick_line(v1.x, v1.y, v2.x, v2.y, 4, self.get_color(sub_sector_id as u64));
        canvas.present();
        std::thread::sleep(Duration::from_millis(10));
    }

    fn draw_player_pos(&self, canvas: &WindowCanvas, context: &GameContext) {
        let player = &context.player;

        let x = self.remap_x(player.position.x);
        let y = self.remap_y(player.position.y);

        canvas.filled_circle(x, y, 8, Color::RGB(255, 165, 0));
    }

    fn draw_linedefs(&self, canvas: &WindowCanvas) {
        for linedef in &self.map_data.linedefs {
            let p1 = self.map_data.vertexes[linedef.start_vertex_id as usize];
            let p2 = self.map_data.vertexes[linedef.end_vertex_id as usize];

            canvas.thick_line(p1.x, p1.y, p2.x, p2.y, 3, Color::RGB(70, 70, 70));
        }
    }

    fn draw_vertexes(&mut self, canvas: &WindowCanvas) {
        for (i, vertex) in self.map_data.vertexes.iter().enumerate() {
            canvas.filled_circle(vertex.x, vertex.y, 4, Color::WHITE);
        }
    }

    fn remap_x(&self, x: i16) -> i16 {
        let min_x = self.map_bounds.min_x as i32;
        let max_x = self.map_bounds.max_x as i32;
        let out_min = self.viewport.x;
        let out_max = self.viewport.w;

        ((max(min_x, min(x as i32, max_x)) - min_x) *
            (out_max - out_min) / (max_x - min_x) + out_min) as i16
    }

    fn remap_y(&self, y: i16) -> i16 {
        let min_y = self.map_bounds.min_y as i32;
        let max_y = self.map_bounds.max_y as i32;
        let out_min = self.viewport.y;
        let out_max = self.viewport.h;
        let height = self.viewport.y + self.viewport.h;

        (height - (max(min_y, min(y as i32, max_y)) - min_y) *
            (out_max - out_min) / (max_y - min_y) - out_min) as i16
    }

    fn remap_vertexes(&mut self) {
        let vertexes = self.map_data.vertexes.iter().map(|v| {
            Vertex {
                x: self.remap_x(v.x),
                y: self.remap_y(v.y)
            }
        });

        self.map_data.vertexes = vertexes.collect();
    }
}