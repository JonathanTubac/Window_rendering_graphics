mod framebuffer;
mod line;
mod polygon;
mod game_of_life;
mod patterns;

use std::thread;
use std::time::Duration;

use raylib::prelude::*;
use framebuffer::Framebuffer;

const CELL_SIZE: usize = 150;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .resizable()
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut current = Framebuffer::new(CELL_SIZE, CELL_SIZE);
    current.set_background_color(game_of_life::DEAD);
    current.clear();

    patterns::spawn_glider(&mut current, 10, 10);
    patterns::spawn_blinker(&mut current, 40, 20);
    patterns::spawn_toad(&mut current, 10, 40);
    patterns::spawn_pulsar(&mut current, 60, 60);
    patterns::spawn_beacon(&mut current, 100, 10);
    patterns::spawn_beehive(&mut current, 100, 25);
    patterns::spawn_block(&mut current, 100, 35);
    patterns::spawn_r_pentomino(&mut current, 20, 100);
    patterns::spawn_lwss(&mut current, 5, 120);
    patterns::spawn_gosper_glider_gun(&mut current, 5, 60);
    patterns::spawn_pentadecathlon(&mut current, 120, 100);
    patterns::spawn_acorn(&mut current, 120, 135);

    let mut next = Framebuffer::new(CELL_SIZE, CELL_SIZE);
    next.set_background_color(game_of_life::DEAD);

    while !window.window_should_close() {
        game_of_life::step(&current, &mut next);
        std::mem::swap(&mut current, &mut next);

        current.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(100));
    }
}
