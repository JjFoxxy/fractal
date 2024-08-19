use std::ops::Range;

pub fn image_to_real_transform(
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

pub fn real_to_image_transform(
    x: (f64, f64),
    real_range: (&Range<f64>, &Range<f64>),
    image_size: (i32, i32),
) -> (i32, i32) {
    (
        ((x.0 - real_range.0.start) * image_size.0 as f64 / (real_range.0.end - real_range.0.start))
            as i32,
        ((x.1 - real_range.1.start) * image_size.1 as f64 / (real_range.1.end - real_range.1.start))
            as i32,
    )
}
