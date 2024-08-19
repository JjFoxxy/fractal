use crate::utils::*;
use raster::{Color, Image};
use std::ops::Range;

fn mandelbrot_map(x: (f64, f64), x0: (f64, f64)) -> (f64, f64) {
    (x.0 * x.0 - x.1 * x.1 + x0.0, 2. * x.0 * x.1 + x0.1)
}

fn belongs_to_mandelbrot(x0: (f64, f64)) -> Color {
    let mut x = x0;

    for _ in 0..200 {
        x = mandelbrot_map(x, x0);
        if x.0 * x.0 + x.1 * x.1 > 4. {
            return Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            };
        }
    }
    Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }
}

fn belongs_to_mandelbrot_colored(x0: (f64, f64)) -> Color {
    let mut x = x0;

    for n in 0..255 {
        x = mandelbrot_map(x, x0);
        if x.0 * x.0 + x.1 * x.1 > 4. {
            let rgb = Color::to_rgb(((n as f64) * (n as f64).log2()) as u16, 100., 50.);
            return Color {
                r: rgb.0,
                g: rgb.1,
                b: rgb.2,
                a: 255,
            };
        }
    }
    Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }
}

pub fn draw_mandelbrot(
    image: &mut Image,
    range_x: &Range<f64>,
    range_y: &Range<f64>,
    colored: bool,
) {
    for x in 0..image.width {
        for y in 0..image.height {
            let z_n =
                image_to_real_transform((x, y), (range_x, range_y), (image.width, image.height));

            let _ = if !colored {
                image.set_pixel(x, y, belongs_to_mandelbrot(z_n))
            } else {
                image.set_pixel(x, y, belongs_to_mandelbrot_colored(z_n))
            };
        }
    }
}
