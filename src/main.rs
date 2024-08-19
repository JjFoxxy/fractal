use crate::mandelbrot::*;
use pico_args::Arguments;
use raster::{editor, Color, Image};
use rossler::draw_rossler;
use std::ops::Range;

mod mandelbrot;
mod rossler;
mod utils;

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

    draw_mandelbrot(&mut image, &range_x, &range_y, args.colored);

    let _ = raster::save(&image, "test.png");

    println!("Done!");

    // Temporary
    let mut image = Image::blank(args.width, args.height);
    editor::fill(&mut image, Color::white()).unwrap();
    draw_rossler(&mut image, &range_x, &range_y);

    let _ = raster::save(&image, "rossler.png");

    Ok(())
}
