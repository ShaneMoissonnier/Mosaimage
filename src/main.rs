use clap::Parser;
use image::{imageops, ImageBuffer, Rgb};
use std::{collections::HashMap, fs};

const SQUARE_SIZE: u32 = 32;

#[derive(Debug)]
struct Square {
    x: u32,
    y: u32,
    color: Rgb<u8>,
}

#[derive(Debug)]
struct PixelImage {
    dimensions: (u32, u32),
    squares: Vec<Square>,
}

fn calculate_rgb_mean(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Rgb<u8> {
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
        color: calculate_rgb_mean(&square_img),
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

#[allow(dead_code)]
fn create_pixelate_image(filename: String, mut img_pixels: PixelImage) {
    let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(
        img_pixels.dimensions.0 * SQUARE_SIZE,
        img_pixels.dimensions.1 * SQUARE_SIZE,
    );

    img_pixels.squares.iter_mut().for_each(|square| {
        let x = square.x * SQUARE_SIZE;
        let y = square.y * SQUARE_SIZE;

        for i in x..x + SQUARE_SIZE {
            for j in y..y + SQUARE_SIZE {
                image_buffer.put_pixel(i, j, square.color);
            }
        }
    });

    image_buffer.save(filename).unwrap();
}

fn write_image(
    src_img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    dest_img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u32,
    y: u32,
) {
    src_img.enumerate_pixels().for_each(|pixel| {
        dest_img.put_pixel(x + pixel.0, y + pixel.1, *pixel.2);
    });
}

fn calculate_source_images_means(source_path_folder: String) -> HashMap<String, Rgb<u8>> {
    let mut hash_map = HashMap::<String, Rgb<u8>>::new();

    for file in fs::read_dir(source_path_folder).unwrap() {
        let file_path = file.unwrap().path().display().to_string();
        let file_img = image::open(file_path.clone()).unwrap();

        hash_map.insert(file_path, calculate_rgb_mean(&file_img.into_rgb8()));
    }

    hash_map
}

pub fn euclidean_distance(color: Rgb<u8>, other_color: Rgb<u8>) -> f64 {
    let r_diff = color[0] as f64 - other_color[0] as f64;
    let g_diff = color[1] as f64 - other_color[1] as f64;
    let b_diff = color[2] as f64 - other_color[2] as f64;

    let squared_distance = r_diff * r_diff + g_diff * g_diff + b_diff * b_diff;

    squared_distance.sqrt()
}

fn find_closest_source_image(hash_map: &HashMap<String, Rgb<u8>>, color: Rgb<u8>) -> String {
    let mut minimum_distance: f64 = f64::MAX;
    let mut key_value: String = Default::default();

    hash_map.iter().for_each(|(key, value)| {
        let map_color = *value;
        let distance = euclidean_distance(color, map_color);

        if distance <= minimum_distance {
            key_value = key.to_string();
            minimum_distance = distance;
        }
    });

    key_value
}

fn open_and_resize_image(image_path: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let source_image = image::open(image_path).unwrap().to_rgb8();
    imageops::resize(
        &source_image,
        SQUARE_SIZE,
        SQUARE_SIZE,
        imageops::FilterType::Gaussian,
    )
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
            .or_insert_with(|| open_and_resize_image(&closest_image_path));

        write_image(
            source_image_buffer,
            output_image_buffer,
            square.x * SQUARE_SIZE,
            square.y * SQUARE_SIZE,
        );
    }
}

#[derive(Parser, Default, Debug)]
#[clap(author, version, about)]
struct Arguments {
    #[arg(short, long)]
    input_image_path: String,

    #[arg(short, long)]
    source_image_path: String,

    #[arg(short, long)]
    output_image_path: String,
}

fn main() {
    let args = Arguments::parse();

    let hash_map = calculate_source_images_means(args.source_image_path);

    let input_img = image::open(args.input_image_path).unwrap();

    let img_pixels = divide_image_into_squares(&input_img.to_rgb8(), SQUARE_SIZE);

    let mut image_buffer_output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(
        img_pixels.dimensions.0 * SQUARE_SIZE,
        img_pixels.dimensions.1 * SQUARE_SIZE,
    );

    match_source_images(&img_pixels, &hash_map, &mut image_buffer_output);

    image_buffer_output.save(args.output_image_path).unwrap();
}
