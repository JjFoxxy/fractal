use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(128, 128);

    for i in 0..128 {
        let result = image.set_pixel(64, i, Color::rgba(255, 0, 0, 255));
    }

    let result = raster::save(&image, "test.png");

    println!("Hello, world!");
}
