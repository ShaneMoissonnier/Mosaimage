use crate::buffer;
use crate::color;
use crate::constants::SQUARE_SIZE;
use crate::parser::Arguments;
use image::{ImageBuffer, Rgb};
use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Square {
    x: u32,
    y: u32,
    color: Rgb<u8>,
}

#[derive(Debug)]
pub struct PixelImage {
    pub dimensions: (u32, u32),
    pub squares: Vec<Square>,
}

fn create_square(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, square_size: u32, x: u32, y: u32) -> Square {
    let square_width = square_size.min(img.width() - x);
    let square_height = square_size.min(img.height() - y);
    let mut square_img = ImageBuffer::new(square_width, square_height);

    for sy in 0..square_height {
        for sx in 0..square_width {
            let pixel = img.get_pixel(x + sx, y + sy);
            square_img.put_pixel(sx, sy, *pixel);
        }
    }

    Square {
        color: color::calculate_rgb_mean(&square_img),
        x: x / square_size,
        y: y / square_size,
    }
}

fn divide_image_into_squares(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, square_size: u32) -> PixelImage {
    let width = img.width();
    let height = img.height();

    let mut squares: Vec<Square> = Vec::new();

    for y in (0..height).step_by(square_size as usize) {
        for x in (0..width).step_by(square_size as usize) {
            let square = create_square(img, square_size, x, y);
            squares.push(square);
        }
    }

    PixelImage {
        squares,
        dimensions: (
            (0..width).step_by(square_size as usize).len() as u32,
            (0..height).step_by(square_size as usize).len() as u32,
        ),
    }
}

fn calculate_source_images_means(source_path_folder: String) -> HashMap<String, Rgb<u8>> {
    let mut hash_map = HashMap::<String, Rgb<u8>>::new();

    for file in fs::read_dir(source_path_folder).unwrap() {
        let file_path = file.unwrap().path().display().to_string();
        let file_img = ::image::open(file_path.clone()).unwrap();

        hash_map.insert(file_path, color::calculate_rgb_mean(&file_img.into_rgb8()));
    }

    hash_map
}

fn find_closest_source_image(hash_map: &HashMap<String, Rgb<u8>>, color: Rgb<u8>) -> String {
    let mut minimum_distance: f64 = f64::MAX;
    let mut key_value: String = Default::default();

    hash_map.iter().for_each(|(key, value)| {
        let map_color = *value;
        let distance = color::calculate_rgb_euclidean_distance(color, map_color);

        if distance <= minimum_distance {
            key_value = key.to_string();
            minimum_distance = distance;
        }
    });

    key_value
}

fn match_source_images(
    img_pixels: &PixelImage,
    source_image_means: &HashMap<String, Rgb<u8>>,
    output_image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let mut source_image_buffers = HashMap::<String, ImageBuffer<Rgb<u8>, Vec<u8>>>::new();

    for square in &img_pixels.squares {
        let closest_image_path = find_closest_source_image(source_image_means, square.color);

        let source_image_buffer = source_image_buffers
            .entry(closest_image_path.clone())
            .or_insert_with(|| buffer::open_and_resize_image(&closest_image_path, SQUARE_SIZE));

        buffer::write_image_into_buffer(
            source_image_buffer,
            output_image_buffer,
            square.x * SQUARE_SIZE,
            square.y * SQUARE_SIZE,
        );
    }
}

pub fn generate_mosaic(args: Arguments) {
    // The result of this call could easily be cached for optimization
    let hash_map = calculate_source_images_means(args.source_image_path);

    let input_img = image::open(args.input_image_path).unwrap();

    let img_pixels = divide_image_into_squares(&input_img.to_rgb8(), SQUARE_SIZE);

    let mut image_buffer_output = ImageBuffer::new(
        img_pixels.dimensions.0 * SQUARE_SIZE,
        img_pixels.dimensions.1 * SQUARE_SIZE,
    );

    match_source_images(&img_pixels, &hash_map, &mut image_buffer_output);

    image_buffer_output.save(args.output_image_path).unwrap();
}
