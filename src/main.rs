extern crate leptess;

use std::path::Path;

// use leptess::{leptonica, tesseract};

use image_convert::{ImageResource, PNGConfig, to_png};

fn main() {
    let source_image_path = Path::new("./tests/58877.png");

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.png");

    let mut config = PNGConfig::new();
    config.width = 1920;
    // config.

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_png(&mut output, &input, &config).unwrap();

    // let mut api = tesseract::TessApi::new(None, "eng").unwrap();
    //
    // let pix = leptonica::pix_read(target_image_path.as_path()).unwrap();
    // api.set_image(&pix);
    //
    // let text = api.get_utf8_text();
    // println!("{}", text.unwrap());
}