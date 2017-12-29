extern crate image;

use image::png;
use image::ImageDecoder;
use image::png::PNGDecoder;
use image::ColorType;
use image::DecodingResult;

use std::ops::Add;

use std::env;

use std::thread;

use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;

const MAXH: usize = 1920;
const MAXW: usize = 1200;


fn pixel(x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) -> String {
    if a == 0 {
        return String::new();
    }
    if a < 255 {
        return format!("PX {} {} {:x}{:x}{:x}{:x}\n", x, y, r, g, b, a);
    }
    return format!("PX {} {} {:x}{:x}{:x}\n", x, y, r, g, b);
}


fn gen (image: &Vec<u8>, image_type: ColorType, w: u32, h: u32, x: u32, y: u32, xoffset: u32, yoffset: u32) -> String {
    let (r, g, b, a);

    match image_type {
        ColorType::RGB(_) => {
            r = image[((y * w + x) * 3 + 0) as usize];
            g = image[((y * w + x) * 3 + 1) as usize];
            b = image[((y * w + x) * 3 + 2) as usize];
            a = 255;
        },
        ColorType::RGBA(_) => {
            r = image[((y * w + x) * 4 + 0) as usize];
            g = image[((y * w + x) * 4 + 1) as usize];
            b = image[((y * w + x) * 4 + 2) as usize];
            a = image[((y * w + x) * 4 + 3) as usize];

        },
        _ => panic!("Zut zut zut!")
    }

    return pixel(x + xoffset, y + yoffset, r, g, b, a);
}


fn compute (output_file: &mut File, image: &mut PNGDecoder<File>, xoffset: u32, yoffset: u32, w: u32, h: u32) {
    let image_type = image.colortype().unwrap();
    match image.read_image().unwrap() {
        DecodingResult::U8(image_buffer_input) => {
            let mut buffer = String::new();

            for y in 0..h {
                for x in 0..w {
                    buffer = buffer.add(&gen(&image_buffer_input, image_type, w, h, x, h - y - 1, xoffset, yoffset));
                }
            }

            output_file.write(buffer.as_bytes());
        },
        _ => panic!("Zot zot zot!")
    }
}




fn main () {


    let mut args = env::args();
    let image_input_path = args.nth(1).unwrap();
    let image_output_path = "temp";
    let xoffset = args.next().unwrap().parse::<u32>().unwrap();
    let yoffset = args.next().unwrap().parse::<u32>().unwrap();
    let iterations = args.next().unwrap().parse::<usize>().unwrap();


    let mut image = png::PNGDecoder::new(File::open(image_input_path).unwrap());
    let mut output_file = File::create(image_output_path).unwrap ();;


    let (w, h) = image.dimensions ().unwrap();
    for i in 0..iterations {
        compute (&mut output_file, &mut image, xoffset, yoffset, w, h);
    }
}




