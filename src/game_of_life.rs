use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub const ALIVE: Color = Color::WHITE;
pub const DEAD: Color = Color::BLACK;

fn is_alive(framebuffer: &Framebuffer, x: i32, y: i32) -> bool {
    framebuffer.get_color(x, y) == ALIVE
}

fn count_alive_neighbors(framebuffer: &Framebuffer, x: i32, y: i32) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if is_alive(framebuffer, x + dx, y + dy) {
                count += 1;
            }
        }
    }
    count
}

pub fn step(current: &Framebuffer, next: &mut Framebuffer) {
    for y in 0..current.height as i32 {
        for x in 0..current.width as i32 {
            let alive = is_alive(current, x, y);
            let neighbors = count_alive_neighbors(current, x, y);

            let will_be_alive = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            next.set_current_color(if will_be_alive { ALIVE } else { DEAD });
            next.set_pixel(x, y);
        }
    }
}
