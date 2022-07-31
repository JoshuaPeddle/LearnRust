use std::fs::File;
use std::path::Path;

use image::{GenericImageView, ImageFormat};

pub fn extract_file_extension(file: &String) -> String {
    file.split('/')
        .last()
        .unwrap()
        .split('.')
        .last()
        .unwrap()
        .to_string()
}

pub fn extract_file_name(file: &String) -> String {
    file.split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string()
}

pub struct RGBAImage {
    pub width: u32,
    pub height: u32,
    pub red: ndarray::Array2<u8>,
    pub green: ndarray::Array2<u8>,
    pub blue: ndarray::Array2<u8>,
    pub alpha: ndarray::Array2<u8>,
}

/** 
 * Convert DynamicImage to RGBAImage
 * # Arguments
 * * `img` - The DynamicImage to convert
 * 
 * # Return 
 * * RGBAImage
 * 
 * @see
 * 
 * https://docs.rs/image/0.9.0/image/
 * 
 * https://docs.rs/ndarray/0.12.0/ndarray/
 * 

 */
pub fn image_to_array(img: &image::DynamicImage) -> RGBAImage {
    let (width, height) = img.dimensions();
    println!("{}x{}", width, height);
    let mut array = ndarray::Array2::zeros((width as usize, height as usize));
    let mut array1 = ndarray::Array2::zeros((width as usize, height as usize));
    let mut array2 = ndarray::Array2::zeros((width as usize, height as usize));
    let mut array3 = ndarray::Array2::zeros((width as usize, height as usize));

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let pixel = img.get_pixel(x, y);
            //println!("{}", pixel);
            array[[x as usize, y as usize]] = pixel[0];
            array1[[x as usize, y as usize]] = pixel[1];
            array2[[x as usize, y as usize]] = pixel[2];
            array3[[x as usize, y as usize]] = pixel[3];
            //array[y][x] = pixel[0];
        }
    }

    let rgba = RGBAImage {
        width,
        height,
        red: array,
        green: array1,
        blue: array2,
        alpha: array3,
    };

    rgba
}

/**
 * Write the contents of this image to the Writer in PNG format.
 *
 * # Arguments
 * * `im` - The image to write.
 * * `new_path` - The path to write the image to.
 * * `file_extension` - The file extension of the image.
 */
pub fn write_img(im: &image::DynamicImage, new_path: &Path, file_extension: &String) {
    let fout = &mut File::create(new_path).unwrap();
    if file_extension == "png" {
        im.write_to(fout, ImageFormat::Png).unwrap();
    }
    if file_extension == "jpeg" {
        im.write_to(fout, ImageFormat::Jpeg).unwrap();
    }
}
