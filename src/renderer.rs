use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use anyhow::Result;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use crate::game_context::GameContext;
use crate::map_renderer::MapRenderer;

#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

pub struct Renderer {
    canvas: WindowCanvas,
    map_renderer: MapRenderer
}

impl Renderer {
    pub fn new(window: Window, context: &GameContext) -> Result<Renderer> {
        let canvas = sdl2::render::CanvasBuilder::new(window)
            .accelerated()
            .present_vsync()
            .build()?;

        let width = canvas.viewport().w as i32;
        let height = canvas.viewport().h as i32;

        let viewport = Viewport {
            x: 30,
            y: 30,
            w: width - 30,
            h: height - 30
        };

        let map_renderer = MapRenderer::new(viewport, context.map_data.to_owned());

        Ok(Renderer {
            canvas,
            map_renderer
        })
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<()> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.map_renderer.draw(&mut self.canvas, &context);

        self.canvas.present();
        Ok(())
    }
}