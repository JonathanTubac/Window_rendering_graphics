use crate::framebuffer::Framebuffer;
use crate::game_of_life::ALIVE;

pub fn spawn_glider(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [
        (1, 0),
        (2, 1),
        (0, 2), (1, 2), (2, 2),
    ];

    framebuffer.set_current_color(ALIVE);
    for (dx, dy) in cells {
        framebuffer.set_pixel(x + dx, y + dy);
    }
}

pub fn spawn_blinker(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [(0, 0), (1, 0), (2, 0)];

    framebuffer.set_current_color(ALIVE);
    for (dx, dy) in cells {
        framebuffer.set_pixel(x + dx, y + dy);
    }
}

pub fn spawn_toad(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [
        (1, 0), (2, 0), (3, 0),
        (0, 1), (1, 1), (2, 1),
    ];

    framebuffer.set_current_color(ALIVE);
    for (dx, dy) in cells {
        framebuffer.set_pixel(x + dx, y + dy);
    }
}
