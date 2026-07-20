mod framebuffer;
mod line;
mod bmp;
mod polygon;

use crate::framebuffer::Framebuffer;
use crate::bmp::WriteBmp;
use crate::polygon::FillPolygon;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Poligono 1
    let poligono1: [(usize, usize); 10] = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    // Poligono 2
    let poligono2: [(usize, usize); 4] = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    // Poligono 3
    let poligono3: [(usize, usize); 3] = [
        (377, 249), (411, 197), (436, 249),
    ];

    // Poligono 4
    let poligono4: [(usize, usize); 18] = [
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    // Poligono 5
    let poligono5: [(usize, usize); 4] = [
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    // Poligono 1: relleno 
    framebuffer.fill_polygon(&poligono1, 0x00FFFF, 0xFF00FF);

    // Poligono 2: relleno 
    framebuffer.fill_polygon(&poligono2, 0x00FFFF, 0xFF00F0);

    // Poligono 3: relleno 
    framebuffer.fill_polygon(&poligono3, 0x00FFF0, 0xFFF000);

    // Poligono 4: relleno amarillo, borde rojo, con el poligono 5 como agujero
    framebuffer.fill_polygon_with_hole(
        &poligono4, 0xFF0000,
        &poligono5, 0xFF0000,
        0xFFFFFF,
    );

    let _ = framebuffer.render_buffer("imagen.bmp");

    println!("Framebuffer renderizada en imagen.bmp");
}