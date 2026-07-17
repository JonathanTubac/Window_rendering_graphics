mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;

fn draw_scene(framebuffer: &mut Framebuffer) {
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::GREEN);
    line(
        framebuffer,
        Vector2::new(50.0, 50.0),
        Vector2::new(350.0, 350.0),
    );

    framebuffer.set_current_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(350.0, 50.0),
        Vector2::new(50.0, 350.0),
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

    let mut framebuffer = Framebuffer::new(window_width as usize, window_height as usize);
    draw_scene(&mut framebuffer);

    while !window.window_should_close() {
        if window.is_window_resized() {
            let new_width = window.get_screen_width() as usize;
            let new_height = window.get_screen_height() as usize;
            framebuffer = Framebuffer::new(new_width, new_height);
            draw_scene(&mut framebuffer);
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
