use image::{imageops, ImageBuffer, Rgb};

pub fn write_image_into_buffer(
    src_img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    dest_img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u32,
    y: u32,
) {
    src_img.enumerate_pixels().for_each(|pixel| {
        dest_img.put_pixel(x + pixel.0, y + pixel.1, *pixel.2);
    });
}

pub fn open_and_resize_image(image_path: &str, size: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let src_img = image::open(image_path).unwrap().to_rgb8();
    imageops::resize(&src_img, size, size, imageops::FilterType::Gaussian)
}
