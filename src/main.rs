// it is basically import the args
mod args ;

use args::{Args, ImageDataErrors};
use image::{imageops::FilterType::Triangle, DynamicImage, GenericImageView};

use crate::args::find_image_by_path;

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:?}", args);
    let (_image1, image1_format) =
        find_image_by_path(args.image_1).expect("given path might be wrong");
    let (_image2, image2_format) =
        find_image_by_path(args.image_2).expect("given path might be wrong");
    if image1_format != image2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    let (_image1, _image2) = standarized_size(_image1, _image2);
    let mut output_image = FloatingImage::new(_image1.width(), _image1.height(), args.output);
    let data = combine_images(_image1, _image2);
    output_image.set_data(data)?;
    let _ = image::save_buffer_with_format(
        output_image.name,
        &output_image.data,
        output_image.width,
        output_image.height,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    );
    println!("Hello, world!");
    Ok(())
}

fn get_smallest_dimensions(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let pix1 = dim1.0 * dim1.1;
    let pix2 = dim2.0 * dim2.1;
    return if pix1 < pix2 { dim1 } else { dim2 };
}

fn standarized_size(image1: DynamicImage, image2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image1.dimensions(), image2.dimensions());
    println!("width : {} & height : {}  /n", width, height);
    if image2.dimensions() > (width, height) {
        image2.resize_exact(width, height, Triangle);
    }
    if image1.dimensions() > (width, height) {
        image1.resize_exact(width, height, Triangle);
    }

    (image1, image2)
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = width * height * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width: width,
            height: height,
            data: buffer,
            name: name,
        }
    }
    fn set_data(&mut self, buffer: Vec<u8>) -> Result<(), ImageDataErrors> {
        if buffer.len() > self.data.capacity() {
            println!("something wrong happend");
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = buffer;
        Ok(())
    }
}

fn combine_images(image1: DynamicImage, image2: DynamicImage) -> Vec<u8> {
    let vec_1 = image1.to_rgb8().into_vec();
    let vec_2 = image2.to_rgb8().into_vec();
    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    let len = my_thing(vec1.len()).unwrap() as u8;
    println!("length= {}", len);
    let mut combined_data = vec![0u8, len];
    let mut i = 0;
    while i + 3 < vec1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec2, i, i + 3));
        }
        i += 4;
    }
    return combined_data;
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        if i == vec.len() {
            break;
        }
        // important
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bound"),
        };
        rgba.push(val)
    }
    rgba
}

fn my_thing(a: usize) -> Result<u32, ImageDataErrors> {
    Ok(a as u32)
}

//type conversion
///StackOverflow : https://stackoverflow.com/questions/28273169/how-do-i-convert-between-numeric-types-safely-and-idiomatically
fn _example(v: i32) -> Option<i8> {
    if v > std::i8::MAX as i32 {
        None
    } else {
        Some(v as i8)
    }
}
