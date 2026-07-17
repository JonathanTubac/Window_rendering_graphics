mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;

fn draw_scene(
    framebuffer: &mut Framebuffer,
    translate_x: f32,
    translate_y: f32,
) {
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::GREEN);
    line(
        framebuffer,
        Vector2::new(50.0 + translate_x, 50.0 + translate_y),
        Vector2::new(350.0 + translate_x, 350.0 + translate_y),
    );

    framebuffer.set_current_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(350.0 + translate_x, 50.0 + translate_y),
        Vector2::new(50.0 + translate_x, 350.0 + translate_y),
    );
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Example")
        .resizable()
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut translate_x = 0.0;
    let mut translate_y = 0.0;

    let mut framebuffer = Framebuffer::new(window_width as usize, window_height as usize);
    draw_scene(&mut framebuffer, translate_x, translate_y);

    

    while !window.window_should_close() {
        translate_x += 1.0;
        translate_y += 1.0;

        if window.is_window_resized() {
            let new_width = window.get_screen_width() as usize;
            let new_height = window.get_screen_height() as usize;
            framebuffer = Framebuffer::new(new_width, new_height);
        }

        draw_scene(&mut framebuffer, translate_x, translate_y);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
