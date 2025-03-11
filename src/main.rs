use image::{imageops::FilterType, open, DynamicImage, GenericImageView};



fn resize_image(path: &str, width: u32, height: u32) -> DynamicImage {
    let img = open(path).expect("failed to load image");
    img.resize(width, height, FilterType::Nearest)
}

fn save_image(image: &DynamicImage, output_path: &str) {

    image.save_with_format(output_path, image::ImageFormat::Png).expect("failed to save image")

}

pub fn main(){

    let img: DynamicImage = open("D:\\coding\\rust-image-processing\\image-processing\\cat.jpg").expect("Could not open image");
    let (width, height) = img.dimensions();
    println!("Image dimensions: {} x {}", width, height);

    println!("Starting converting image png...");
    img.save_with_format("D:\\coding\\rust-image-processing\\image-processing\\cat.png", image::ImageFormat::Png).expect("Could not save image to png");

    println!("Staring converting image webp...");
    img.save_with_format("D:\\coding\\rust-image-processing\\image-processing\\cat.webp", image::ImageFormat::WebP).expect("Could not save iamge to webp");


    let resize_image = resize_image("D:\\coding\\rust-image-processing\\image-processing\\cat.jpg", 512, 512);

    save_image(&resize_image, "D:\\coding\\rust-image-processing\\image-processing\\cat-resized.png");

}