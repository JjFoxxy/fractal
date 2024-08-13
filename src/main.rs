use raster::{Color, Image};
use std::ops::Range;

fn image_to_real_transform(coord: f64, real_range: f64, image_size: f64, offset: f64) -> f64 {
    coord * real_range / image_size - offset
}

fn mandelbrot_map(x: (f64, f64), x0: (f64, f64)) -> (f64, f64) {
    (x.0 * x.0 - x.1 * x.1 + x0.0, 2. * x.0 * x.1 + x0.1)
}

fn belongs_to_mandelbrot(x0: (f64, f64)) -> bool {
    let mut x = x0;
    for _ in 0..200 {
        x = mandelbrot_map(x, x0);
        if x.0 * x.0 + x.1 * x.1 > 4. {
            return false;
        }
    }
    true
}

fn draw_mandelbrot(image: &mut Image, range_x: Range<f64>, range_y: Range<f64>) {
    for x in 0..image.width {
        for y in 0..image.height {
            let z_n = (
                image_to_real_transform(
                    x as f64,
                    range_x.end - range_x.start,
                    image.width as f64,
                    (range_x.end - range_x.start) / 2.,
                ),
                image_to_real_transform(
                    y as f64,
                    range_y.end - range_y.start,
                    image.height as f64,
                    (range_y.end - range_y.start) / 2.,
                ),
            );
            if belongs_to_mandelbrot(z_n) {
                let _ = image.set_pixel(x, y, Color::rgba(0, 0, 0, 255));
            } else {
                let _ = image.set_pixel(x, y, Color::rgba(255, 255, 255, 255));
            }
        }
    }
}

fn main() {
    let mut image = Image::blank(1024, 1024);

    draw_mandelbrot(&mut image, -2.0..1.0, -1.0..1.0);

    let _ = raster::save(&image, "test.png");

    println!("Done!");
}
