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

pub fn spawn_pulsar(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];

    framebuffer.set_current_color(ALIVE);
    for (dx, dy) in cells {
        framebuffer.set_pixel(x + dx, y + dy);
    }
}
