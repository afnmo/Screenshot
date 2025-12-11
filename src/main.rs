use image::{DynamicImage, ImageOutputFormat};
use std::fs::File;
use zip::write::FileOptions;
use std::process::Command;

fn main() {
    let temp_path = "temp.png";
    let output_image = "output.jpg";
    let output_zip = "screenshot.zip";

    Command::new("screencapture")
        .args(&["-x", temp_path])
        .status()
        .expect("Failed to capture screenshot");

    let img = image::open(temp_path).expect("Failed to load screenshot");
    let mut gray = img.to_luma8();
    image::imageops::flip_vertical_in_place(&mut gray);

    let mut image_file = File::create(output_image).expect("Failed to create output image");
    DynamicImage::ImageLuma8(gray)
        .write_to(&mut image_file, ImageOutputFormat::Jpeg(70))
        .expect("Failed to save output image");

    let zip_file = File::create(output_zip).expect("Failed to create zip file");
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file(output_image, options).expect("Failed to add file to zip");
    let mut f = File::open(output_image).expect("Failed to open image for zipping");
    std::io::copy(&mut f, &mut zip).expect("Failed to write image into zip");
    zip.finish().expect("Failed to finish zip");

    println!("Saved {} containing {}", output_zip, output_image);
}
