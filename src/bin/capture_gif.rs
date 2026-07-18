#[path = "../framebuffer.rs"]
mod framebuffer;
#[path = "../game_of_life.rs"]
mod game_of_life;
#[path = "../patterns.rs"]
mod patterns;

use framebuffer::Framebuffer;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, RgbaImage};
use std::fs::File;
use std::time::Duration;

const CELL_SIZE: usize = 150;
const SCALE: u32 = 4;
const GENERATIONS: usize = 200;
const FRAME_STEP: usize = 2;

fn framebuffer_to_frame(fb: &Framebuffer) -> Frame {
    let width = fb.width as u32 * SCALE;
    let height = fb.height as u32 * SCALE;
    let mut img = RgbaImage::new(width, height);

    for y in 0..fb.height {
        for x in 0..fb.width {
            let c = fb.get_color(x as i32, y as i32);
            let pixel = image::Rgba([c.r, c.g, c.b, 255]);
            for sy in 0..SCALE {
                for sx in 0..SCALE {
                    img.put_pixel(x as u32 * SCALE + sx, y as u32 * SCALE + sy, pixel);
                }
            }
        }
    }

    Frame::from_parts(img, 0, 0, Delay::from_saturating_duration(Duration::from_millis(100)))
}

fn main() {
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

    let mut frames = Vec::new();
    frames.push(framebuffer_to_frame(&current));

    for i in 0..GENERATIONS {
        game_of_life::step(&current, &mut next);
        std::mem::swap(&mut current, &mut next);

        if i % FRAME_STEP == 0 {
            frames.push(framebuffer_to_frame(&current));
        }
    }

    let frame_count = frames.len();

    let file = File::create("demo.gif").expect("could not create demo.gif");
    let mut encoder = GifEncoder::new(file);
    encoder.set_repeat(Repeat::Infinite).expect("could not set gif repeat");
    encoder
        .encode_frames(frames.into_iter())
        .expect("could not encode gif");

    println!("Wrote demo.gif with {} frames", frame_count);
}
