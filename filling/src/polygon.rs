use crate::framebuffer::Framebuffer;
use crate::line::Line;

type Edge = (f64, f64, f64, f64);

fn edges_from_points(points: &[(usize, usize)]) -> Vec<Edge> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n);

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        edges.push((x1 as f64, y1 as f64, x2 as f64, y2 as f64));
    }

    edges
}

/// Llena las esquinas usando el algoritmo scanline.
fn scanline_fill(fb: &mut Framebuffer, edges: &[Edge], fill_color: u32) {
    if edges.is_empty() {
        return;
    }

    let y_min = edges
        .iter()
        .fold(f64::MAX, |acc, e| acc.min(e.1).min(e.3))
        .floor() as i32;
    let y_max = edges
        .iter()
        .fold(f64::MIN, |acc, e| acc.max(e.1).max(e.3))
        .ceil() as i32;

    fb.set_current_color(fill_color);

    for y in y_min..=y_max {
        if y < 0 {
            continue;
        }

        let y_sample = y as f64 + 0.5;

        let mut intersections: Vec<f64> = Vec::new();

        for &(x1, y1, x2, y2) in edges {
            if (y1 - y2).abs() < f64::EPSILON {
         
                continue;
            }

            let (y_lo, y_hi, x_lo, x_hi) = if y1 < y2 {
                (y1, y2, x1, x2)
            } else {
                (y2, y1, x2, x1)
            };

            if y_sample >= y_lo && y_sample < y_hi {
                let t = (y_sample - y_lo) / (y_hi - y_lo);
                let x = x_lo + t * (x_hi - x_lo);
                intersections.push(x);
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut i = 0;
        while i + 1 < intersections.len() {
            let x_start = intersections[i].round().max(0.0) as i32;
            let x_end = intersections[i + 1].round().max(0.0) as i32;

            for x in x_start..=x_end {
                fb.point(x as usize, y as usize);
            }

            i += 2;
        }
    }
}

fn draw_outline(fb: &mut Framebuffer, points: &[(usize, usize)], line_color: u32) {
    fb.set_current_color(line_color);

    let n = points.len();
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        fb.line(x1, y1, x2, y2);
    }
}

pub trait FillPolygon {

    fn fill_polygon(&mut self, points: &[(usize, usize)], fill_color: u32, line_color: u32);

    fn fill_polygon_with_hole(
        &mut self,
        outer: &[(usize, usize)],
        outer_line_color: u32,
        hole: &[(usize, usize)],
        hole_line_color: u32,
        fill_color: u32,
    );
}

impl FillPolygon for Framebuffer {
    fn fill_polygon(&mut self, points: &[(usize, usize)], fill_color: u32, line_color: u32) {
        let edges = edges_from_points(points);
        scanline_fill(self, &edges, fill_color);
        draw_outline(self, points, line_color);
    }

    fn fill_polygon_with_hole(
        &mut self,
        outer: &[(usize, usize)],
        outer_line_color: u32,
        hole: &[(usize, usize)],
        hole_line_color: u32,
        fill_color: u32,
    ) {
        let mut edges = edges_from_points(outer);
        edges.extend(edges_from_points(hole));

        scanline_fill(self, &edges, fill_color);
        draw_outline(self, outer, outer_line_color);
        draw_outline(self, hole, hole_line_color);
    }
}