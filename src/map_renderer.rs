use std::cmp::{max, min};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::sys::Window;
use crate::wad::{Linedef, MapData, Vertex};

#[derive(Debug)]
struct MapBounds {
    pub min_x: i16,
    pub max_x: i16,
    pub min_y: i16,
    pub max_y: i16
}

pub struct MapRenderer {
    vertexes: Vec<Vertex>,
    linedefs: Vec<Linedef>,
    map_bounds: MapBounds
}

impl MapRenderer {
    pub fn new(map_data: &MapData) -> Self {
        let vertexes = map_data.vertexes.to_owned();
        let linedefs = map_data.linedefs.to_owned();

        let map_bounds = MapBounds {
            min_x: vertexes.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
            max_x: vertexes.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
            min_y: vertexes.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
            max_y: vertexes.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
        };

        Self {
            vertexes,
            linedefs,
            map_bounds
        }
    }

    pub fn draw(&mut self, canvas: &WindowCanvas) {
        let width = canvas.viewport().w as i32;
        let height = canvas.viewport().h as i32;

        self.remap_vertexes(width, height);
        self.draw_linedefs(canvas);
        self.draw_vertexes(canvas);
    }

    fn draw_linedefs(&self, canvas: &WindowCanvas) {
        for linedef in &self.linedefs {
            let p1 = self.vertexes[linedef.start_vertex_id as usize];
            let p2 = self.vertexes[linedef.end_vertex_id as usize];

            canvas.thick_line(p1.x, p1.y, p2.x, p2.y, 3, Color::YELLOW);
        }
    }

    fn draw_vertexes(&mut self, canvas: &WindowCanvas) {
        for (i, vertex) in self.vertexes.iter().enumerate() {
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
        let vertexes = self.vertexes.iter().map(|v| {
            Vertex {
                x: self.remap_x(v.x, width, 30) as i16,
                y: self.remap_y(v.y, height, 30) as i16
            }
        });

        self.vertexes = vertexes.collect();
    }
}