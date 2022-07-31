//! An example of opening an image.
extern crate image;
extern crate ndarray;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{GenericImageView, ImageFormat};


fn image_to_array(img: &image::DynamicImage) -> ndarray::Array2<u8> {
    let (width, height) = img.dimensions();
    println!("{}x{}", width, height);
    let mut array = ndarray::Array2::zeros( (width as usize,height as usize) );

    for y in 0..height-1 {
        for x in 0..width-1 {
            let pixel = img.get_pixel(x, y);
            //println!("{}", pixel);
            array[[x as usize, y as usize]] = pixel[0];

            //array[y][x] = pixel[0];
        }
    }

    array
}


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
    let array = image_to_array(&im);
    println!("{}", array);


    // The color method returns the image's ColorType
    println!("{:?}", im.color());

    let fout = &mut File::create(&Path::new(&format!("{}new.png", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    im.write_to(fout, ImageFormat::Png).unwrap();
}