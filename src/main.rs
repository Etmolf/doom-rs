#![allow(unused, dead_code)]

use std::path::PathBuf;
use std::time::Instant;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use doom::wad;
use doom::renderer::Renderer;
use doom::game_context::GameContext;
use anyhow::Result;

const DOOM_W: u32 = 320;
const DOOM_H: u32 = 200;
const SCALE: u32 = 5;

const SCREEN_WIDTH: u32 = DOOM_W * SCALE;
const SCREEN_HEIGHT: u32 = DOOM_H * SCALE;

fn main() -> Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Doom", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()?;

    let mut context = GameContext::new(PathBuf::from("wad/DOOM1.WAD"), "E1M1")?;
    let mut renderer = Renderer::new(window, &context)?;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_frame = Instant::now();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'main,
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_time = delta.as_secs_f64();
        last_frame = now;

        context.update(delta_time);

        renderer.draw(&context)?;
    }

    Ok(())
}
