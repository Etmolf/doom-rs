use std::path::PathBuf;
use sdl2::render::WindowCanvas;
use sdl2::sys::SDL_Renderer;
use anyhow::Result;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::video::Window;

use crate::wad::Loader;
use crate::map::GameMap;
use crate::Player;

const DOOM_W: u32 = 320;
const DOOM_H: u32 = 200;

pub struct DoomEngine {
    canvas: WindowCanvas,
    player: Player,
    map: GameMap,
    loader: Loader,
    is_over: bool
}

impl DoomEngine {
    pub fn new(window: Window) -> Result<Self> {
        let mut canvas = sdl2::render::CanvasBuilder::new(window)
            .software()
            .present_vsync()
            .build()?;

        canvas.set_logical_size(DOOM_W, DOOM_H);

        let player = Player::new(1);
        let map = GameMap::new("E1M1", player, &canvas);
        let loader = Loader::new(PathBuf::from("wad/DOOM1.wad")).unwrap();

        Ok(
            Self {
                canvas,
                map,
                player,
                loader,
                is_over: false
            }
        )
    }

    pub fn init(&mut self) -> Result<()> {
        self.loader.load_map_data(&mut self.map);

        Ok(())
    }

    pub fn update(&mut self, delta_time: f64) {

    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.map.render_automap(&mut self.canvas);

        self.canvas.present();
    }
}