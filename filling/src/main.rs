mod framebuffer;
mod line;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::line::Line;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x00FF00);
    framebuffer.clear();

    framebuffer.set_current_color(0x0000FF);

    framebuffer.line(100, 100, 300, 500);
    framebuffer.line(300, 500, 700, 100);
    framebuffer.line(700, 100, 100, 100);
    framebuffer.point(0,0);

    let _ = framebuffer.render_buffer("imagen.bmp");

    println!("Framebuffer renderizada en imagen.bmp");
} 