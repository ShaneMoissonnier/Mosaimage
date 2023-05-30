use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Rgb};
use std::fs;

const SQUARE_SIZE: u32 = 32;

struct Square {
    x: u32,
    y: u32,
    color: Rgb<u8>,
}

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

    dest_img.save("test2.png").unwrap();
}

fn min_dimension(dynamic_image: &DynamicImage) -> u32 {
    dynamic_image
        .dimensions()
        .0
        .min(dynamic_image.dimensions().1)
}

fn main() {
    let me_img = image::open("assets/input_images/astronaut.jpg").unwrap();
    let pixel_img = divide_image_into_squares(&me_img.into_rgb8(), SQUARE_SIZE);

    for file in fs::read_dir("assets/source_images/").unwrap() {
        let file_path = file.unwrap().path().display().to_string();
        let file_img = image::open(file_path).unwrap();

        println!("{:?}", calculate_rgb_mean(&file_img.into_rgb8()));
    }
}
