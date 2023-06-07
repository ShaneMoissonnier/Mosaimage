use image::{ImageBuffer, Rgb};

pub fn calculate_rgb_euclidean_distance(color: Rgb<u8>, other_color: Rgb<u8>) -> f64 {
    let r_diff = color[0] as f64 - other_color[0] as f64;
    let g_diff = color[1] as f64 - other_color[1] as f64;
    let b_diff = color[2] as f64 - other_color[2] as f64;

    let squared_distance = r_diff * r_diff + g_diff * g_diff + b_diff * b_diff;

    squared_distance.sqrt()
}

pub fn calculate_rgb_mean(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Rgb<u8> {
    let (sum_red, sum_green, sum_blue) = image.pixels().fold((0, 0, 0), |acc, pixel| {
        let rgb = pixel.0;
        (
            acc.0 + rgb[0] as u32,
            acc.1 + rgb[1] as u32,
            acc.2 + rgb[2] as u32,
        )
    });

    let total_pixels = image.pixels().count() as u32;

    let mean_red = (sum_red / total_pixels) as u8;
    let mean_green = (sum_green / total_pixels) as u8;
    let mean_blue = (sum_blue / total_pixels) as u8;

    Rgb([mean_red, mean_green, mean_blue])
}
