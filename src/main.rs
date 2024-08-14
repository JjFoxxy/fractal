use raster::{Color, Image};
use std::ops::Range;

fn image_to_real_transform(
    x: (i32, i32),
    real_range: (&Range<f64>, &Range<f64>),
    image_size: (i32, i32),
) -> (f64, f64) {
    let x = (x.0 as f64, x.1 as f64);
    let image_size = (image_size.0 as f64, image_size.1 as f64);
    (
        real_range.0.start + x.0 * (real_range.0.end - real_range.0.start) / image_size.0,
        real_range.1.end - x.1 * (real_range.1.end - real_range.1.start) / image_size.1,
    )
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
            let z_n =
                image_to_real_transform((x, y), (&range_x, &range_y), (image.width, image.height));
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
