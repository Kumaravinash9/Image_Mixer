use image::{io::Reader, DynamicImage, ImageFormat};

fn get_nth_arg(n: usize) -> String {
    return std::env::args().nth(n).unwrap();
}

// pub makes the public visibility
#[derive(Debug)]
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String,
}

#[derive(Debug)]
pub enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

impl Args {
    pub fn new() -> Self {
        Args {
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}

pub fn find_image_by_path(path: String) -> Result<(DynamicImage, ImageFormat), std::io::Error> {
    let image_reader = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
    Ok((image, image_format))
}
