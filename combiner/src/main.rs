mod args;
use args::Args;

mod floatingImage;
use floatingImage::FloatingImage;
// extern crate image;
use image::{ io::Reader, DynamicImage, ImageFormat };
use image::GenericImageView;
use std::io::BufReader;
use std::fs::File;
use image::imageops::Triangle;

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:#?}", args);

    let (image_1, image_1_format) = find_image_from_path(args.image_1);
    let (image_2, image_2_format) = find_image_from_path(args.image_2);

    if image_1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardize_images(image_1, image_2);
    let image_dim = image_1.dimensions();
    let mut output = FloatingImage::new(image_dim.0, image_dim.1, args.output);
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn find_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let area_1 = dim_1.0 * dim_1.1;
    let area_2 = dim_2.0 * dim_2.1;

    if area_1 > area_2 {
        dim_2
    } else {
        dim_1
    }
}

fn standardize_images(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = find_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, height: {}\n", width, height);
     
    if image_1.dimensions() == (width, height) {
        (image_1, image_2.resize_exact(width, height, Triangle))
    } else {
        (image_1.resize_exact(width, height, Triangle), image_2)
    }
}