//! An example of opening an image.
extern crate image;
extern crate ndarray;

use std::env;

use std::path::Path;

use images;

use image::GenericImageView;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
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
    println!("{:?}", im.color());

    let file_extension = images::extract_file_extension(&file);
    let file_name = images::extract_file_name(&file);
    // println!(
    //     "File name: {:?}, Extension: {:?} ",
    //     file_name, file_extension,
    // );

    let new_path = Path::new(&file).with_file_name(format!("new_{}.{}", file_name, file_extension));

    images::write_img(&im, &new_path, &file_extension);
}
