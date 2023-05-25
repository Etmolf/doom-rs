#![allow(unused, dead_code)]

use std::path::PathBuf;
use std::time::Instant;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use anyhow::Result;

use doom::DoomEngine;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 1000;

fn main() -> Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Doom", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()?;

    let mut engine = DoomEngine::new(window)?;

    engine.init();

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

        engine.update(delta_time);
        engine.render();
    }

    Ok(())
}
