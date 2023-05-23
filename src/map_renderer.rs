use std::cmp::{max, min};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::sys::Window;
use crate::game_context::GameContext;
use crate::wad::{Linedef, MapData, Point, Vertex};

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
}

impl MapRenderer {
    pub fn new(width: i32, height: i32, map_data: MapData) -> MapRenderer {
        let map_bounds = MapBounds {
            min_x: map_data.vertexes.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
            max_x: map_data.vertexes.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
            min_y: map_data.vertexes.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
            max_y: map_data.vertexes.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
        };

        let mut map_renderer = Self {
            map_data,
            map_bounds
        };

        map_renderer.remap_vertexes(width, height);
        map_renderer
    }

    pub fn draw(&mut self, canvas: &WindowCanvas, context: &GameContext) {
        self.draw_linedefs(canvas);
        self.draw_vertexes(canvas);
        self.draw_player_pos(canvas, context);
    }

    fn draw_player_pos(&self, canvas: &WindowCanvas, context: &GameContext) {
        let player = &context.player;

        let x = self.remap_x(player.position.x, canvas.viewport().w, 30) as i16;
        let y = self.remap_y(player.position.y, canvas.viewport().h, 30) as i16;

        canvas.filled_circle(x, y, 8, Color::BLUE);
    }

    fn draw_linedefs(&self, canvas: &WindowCanvas) {
        for linedef in &self.map_data.linedefs {
            let p1 = self.map_data.vertexes[linedef.start_vertex_id as usize];
            let p2 = self.map_data.vertexes[linedef.end_vertex_id as usize];

            canvas.thick_line(p1.x, p1.y, p2.x, p2.y, 3, Color::YELLOW);
        }
    }

    fn draw_vertexes(&mut self, canvas: &WindowCanvas) {
        for (i, vertex) in self.map_data.vertexes.iter().enumerate() {
            canvas.filled_circle(vertex.x, vertex.y, 4, Color::WHITE);
        }
    }

    fn remap_x(&self, x: i16, width: i32, padding: i32) -> i32 {
        let min_x = self.map_bounds.min_x as i32;
        let max_x = self.map_bounds.max_x as i32;
        let out_min = padding;
        let out_max = width - padding;

        (max(min_x, min(x as i32, max_x)) - min_x) *
            (out_max - out_min) / (max_x - min_x) + out_min
    }

    fn remap_y(&self, y: i16, height: i32, padding: i32) -> i32 {
        let min_y = self.map_bounds.min_y as i32;
        let max_y = self.map_bounds.max_y as i32;
        let out_min = padding;
        let out_max = height - padding;

        height - (max(min_y, min(y as i32, max_y)) - min_y) *
            (out_max - out_min) / (max_y - min_y) - out_min
    }

    fn remap_vertexes(&mut self, width: i32, height: i32) {
        let vertexes = self.map_data.vertexes.iter().map(|v| {
            Vertex {
                x: self.remap_x(v.x, width, 30) as i16,
                y: self.remap_y(v.y, height, 30) as i16
            }
        });

        self.map_data.vertexes = vertexes.collect();
    }
}