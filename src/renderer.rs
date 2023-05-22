use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use anyhow::Result;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use crate::game_context::GameContext;
use crate::map_renderer::MapRenderer;

pub struct Renderer {
    canvas: WindowCanvas
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer> {
        let canvas = sdl2::render::CanvasBuilder::new(window)
            .accelerated()
            .present_vsync()
            .build()?;

        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self, context: &mut GameContext) -> Result<()> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let mut map_renderer = MapRenderer::new(&context.map_data);
        map_renderer.draw(&self.canvas);

        self.canvas.present();
        Ok(())
    }
}