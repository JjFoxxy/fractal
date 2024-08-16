use pico_args::Arguments;
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

fn draw_mandelbrot(image: &mut Image, range_x: Range<f64>, range_y: Range<f64>, colored: bool) {
    for x in 0..image.width {
        for y in 0..image.height {
            let z_n =
                image_to_real_transform((x, y), (&range_x, &range_y), (image.width, image.height));

            let _ = if !colored {
                image.set_pixel(x, y, belongs_to_mandelbrot(z_n))
            } else {
                image.set_pixel(x, y, belongs_to_mandelbrot_colored(z_n))
            };
        }
    }
}

fn parse_floating(s: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|_| "not a number".to_string())
}

#[derive(Debug)]
struct Args {
    width: i32,
    height: i32,
    colored: bool,
    center_x: f64,
    center_y: f64,
    real_width: f64,
    real_height: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();
    let args = Args {
        width: args.value_from_str("--width")?.unwrap_or(1024),
        height: args.value_from_str("--height")?.unwrap_or(768),
        colored: args.contains(["-c", "--colored"]),
        center_x: args
            .value_from_fn("--center_x", parse_floating)?
            .unwrap_or(0.),
        center_y: args
            .value_from_fn("--center_y", parse_floating)?
            .unwrap_or(0.),
        real_width: args
            .value_from_fn("--real_width", parse_floating)?
            .unwrap_or(1.),
        real_height: args
            .value_from_fn("--real_height", parse_floating)?
            .unwrap_or(1.),
    };

    println!("Runnig with arguments: {:?}", args);

    let mut image = Image::blank(args.width, args.height);

    let range_x = Range {
        start: args.center_x - args.real_width / 2.0f64,
        end: args.center_x + args.real_width / 2.0f64,
    };
    let range_y = Range {
        start: args.center_y - args.real_height / 2.0f64,
        end: args.center_y + args.real_height / 2.0f64,
    };

    draw_mandelbrot(&mut image, range_x, range_y, args.colored);

    let _ = raster::save(&image, "test.png");

    println!("Done!");

    Ok(())
}
