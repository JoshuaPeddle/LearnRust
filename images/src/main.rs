//! An example of opening an image.

extern crate image;

extern crate ndarray;
//https://docs.rs/ndarray/0.12.1/ndarray/doc/ndarray_for_numpy_users/index.html

use std::env;

use std::path::Path;

use images;

use image::GenericImageView;

fn _debug_suite() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        //panic!("Please enter a file")
        "images/index.png".to_string()
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new(&file)).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", im.dimensions());

    // Print a pixel at (0, 0)
    println!("pixel at (0, 0) {:?}", im.get_pixel(128, 128));

    // Convert the dynamic image to an array of pixels
    let _array = images::image_to_array(&im);
    // println!(
    //     "red{:#?} green {:#?} blue {:#?} alpha {:#?} height x width {} x {}",
    //     array.red.dim(),
    //     array.green.dim(),
    //     array.blue.dim(),
    //     array.alpha.dim(),
    //     array.height,
    //     array.width
    // );

    // The color method returns the image's ColorType

    let blured_im = im.blur(1 as f32);

    println!("{:?}", im.color());

    let file_extension = images::extract_file_extension(&file);
    let file_name = images::extract_file_name(&file);
    // println!(
    //     "File name: {:?}, Extension: {:?} ",
    //     file_name, file_extension,
    // );

    let new_path = Path::new(&file).with_file_name(format!("new_{}.{}", file_name, file_extension));

    images::write_img(&blured_im, &new_path, &file_extension);
}

fn main() {
    //_debug_suite();

    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        //panic!("Please enter a file")
        "images/index.png".to_string()
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new(&file)).unwrap();

    let mut target = im.clone();

    images::fill_dynamic_image(&mut target, image::Rgba([123, 123, 0, 255]));

    //let black_pixel = image::Rgba([0, 0, 0, 255]);
    //for pixel in im.pixels() {
    //println!("{:?}", pixel);
    //println!("{:?}", pixel.0);
    //    target.put_pixel(pixel.0, pixel.1, black_pixel);
    //}
    //                   WRITE IMAGE

    let file_extension = images::extract_file_extension(&file);
    let file_name = images::extract_file_name(&file);
    // println!(
    //     "File name: {:?}, Extension: {:?} ",
    //     file_name, file_extension,
    // );

    let new_path = Path::new(&file).with_file_name(format!("new_{}.{}", file_name, file_extension));

    images::write_img(&target, &new_path, &file_extension);
}
