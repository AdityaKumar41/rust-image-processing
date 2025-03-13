use image::{imageops::{crop_imm, FilterType}, open, DynamicImage, GenericImageView};

fn resize_image(path: &str, width: u32, height: u32) -> DynamicImage {
    let img = open(path).expect("failed to load image");
    img.resize(width, height, FilterType::Lanczos3) //you can use Nearest
}

fn save_image(image: &DynamicImage, output_path: &str) {

    image.save_with_format(output_path, image::ImageFormat::Png).expect("failed to save image")

}

fn crop_image(img: DynamicImage, x: u32, y:u32, width: u32, height: u32) -> DynamicImage {
    let crooped_img = crop_imm(&img, x, y, width, height);
    DynamicImage::ImageRgba8(crooped_img.to_image())
}

fn rotate_image(path: &str, degree:u32) -> DynamicImage {
    let img = open(path).expect("failed to load image");
    match degree {
        90=> img.rotate90(),
        180=> img.rotate180(),
        270=> img.rotate270(),
        _ =>{
            eprintln!("unsupported rotation angle support 90, 180, 270");
            img // return original image for unsupported angles
        }
    }

}

fn resize_image_maintaning_ratio(path: &str, new_width: Option<u32>, new_height: Option<u32>) -> DynamicImage {
    let img = open(path).expect("failed to load iamge");
    let (width, height) = img.dimensions();

    // calculate the ratio 
    let ratio = width as u32 / height as u32 ;

    let (resize_height, resize_width) = match (new_height, new_width) {

        (Some(w), None)=> (w, (w as f32 / ratio as f32).round() as u32),
        (None, Some(h))=> ((h as f32 * ratio as f32).round() as u32, h),
        (Some(w), Some(h)) => (w, h), // get the new ration
        (None, None) => (width, height) // if the width and height not given take default

    };

    img.resize(resize_width, resize_height, FilterType::Lanczos3)
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


    // rotate image
    let rotated_img = rotate_image("D:\\coding\\rust-image-processing\\image-processing\\cat.jpg", 90);
    save_image(&rotated_img, "D:\\coding\\rust-image-processing\\image-processing\\cat-rotated-image.png");

    // iamge ratio
    let ratio_image = resize_image_maintaning_ratio("D:\\coding\\rust-image-processing\\image-processing\\cat.jpg", Some(512), Some(512));

    save_image(&ratio_image, "D:\\coding\\rust-image-processing\\image-processing\\cat-ration-iamge.jpg");


    let croped_image = crop_image(img, 50, 50, 500, 500);
    save_image(&croped_image, "D:\\coding\\rust-image-processing\\image-processing\\cat-crop-image.png");

} 