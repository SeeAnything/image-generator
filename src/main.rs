#[macro_use]
extern crate bmp;
extern crate rand;

use std::fs;
use std::path::Path;
use std::vec::Vec;
use std::thread;
use std::time::Duration;

use bmp::{Image, Pixel};
use rand::prelude::*;

fn incr_vec_8bpp(px_vec: &mut Vec<u8>) {

    const MAX: u8 = 255;
    const MIN: u8 = 0;

    let mut idx: usize = 0;

    if px_vec[idx] != MAX {
        px_vec[idx] = px_vec[idx] +1;
        return;
    }

    while idx < px_vec.len() && px_vec[idx] == MAX {

        px_vec[idx] = MIN;
        idx = idx + 1;

        if idx == px_vec.len(){
            break;
        }

        if px_vec[idx] != MAX {
            px_vec[idx] = px_vec[idx] + 1;
            return;
        }
    }


}

fn incr_vec_1bpp(px_vec: &mut Vec<u8>) {

    const MAX: u8 = 255;
    const MIN: u8 = 0;

    let mut idx: usize = 0;

    if px_vec[idx] != MAX {
        px_vec[idx] = MAX;
        return;
    }

    while idx < px_vec.len() && px_vec[idx] != MIN {

        px_vec[idx] = MIN;
        idx = idx + 1;

        if idx == px_vec.len(){
            break;
        }

        if px_vec[idx] == MIN {
            px_vec[idx] = MAX;
            return;
        }
    }
}

fn img_from_vec_mono(w: u32, h: u32, px_vec: &Vec<u8>) -> Image {
    let mut img = Image::new(w, h);

    let mut idx = 0;

    for (x, y) in img.coordinates() {
        if idx == px_vec.len() {
            break;
        }

        let c: u8 = px_vec[idx];

        img.set_pixel(x, y, px!(c, c, c));
        idx = idx + 1;
    }

    img
}

fn generate_images_8bpp(w: u32, h: u32, iter: u32, fname: &str) {
    // Pixel value vector
    let mut px: Vec<u8> = Vec::with_capacity((320 * 240) as usize);

    // Load the most recent image if it exists
    if Path::new(fname).exists() {

        let latest_img: Image = bmp::open(fname).unwrap();

        for (x, y) in latest_img.coordinates() {
            // Use the red channel of the pixel to determine the greyscale value
            let px_value: u8  = latest_img.get_pixel(x, y).r;
            px.push(px_value);
        }
    // If the file does not exist, load the pixel with black
    } else {

        let mut ix = 0;

        while ix < (w * h) {
            px.push(0);
            ix = ix + 1;
        }
    }

    loop {
        // Create the image
        let img = img_from_vec_mono(w, h, &px);

        // Write the image to a temporary file and rename it so the
        // server can find it. These steps are important in order to
        // avoid serving files that are in the process of being written
        let temp_fpath = format!("{}.tmp", fname);
        img.save(temp_fpath.to_string()).unwrap();
        fs::rename(temp_fpath, fname).unwrap();


        // Increment the pixel vector
        let mut incr_number = 0;

        while incr_number < iter {
            incr_vec_8bpp(&mut px);
            incr_number = incr_number + 1;
            thread::sleep(Duration::from_millis(1));
        }
    }
}

fn generate_images_1bpp(w: u32, h: u32, iter: u32, fname: &str) {
    // Pixel value vector
    let mut px: Vec<u8> = Vec::with_capacity((320 * 240) as usize);

    // Load the most recent image if it exists
    if Path::new(fname).exists() {

        let latest_img: Image = bmp::open(fname).unwrap();

        for (x, y) in latest_img.coordinates() {
            // Use the red channel of the pixel to determine the greyscale value
            let px_value: u8  = latest_img.get_pixel(x, y).r;
            px.push(px_value);
        }
        // If the file does not exist, load the pixel with black
    } else {

        let mut ix = 0;

        while ix < (w * h) {
            px.push(0);
            ix = ix + 1;
        }
    }

    loop {
        // Create the image
        let img = img_from_vec_mono(w, h, &px);

        // Write the image to a temporary file and rename it so the
        // server can find it. These steps are important in order to
        // avoid serving files that are in the process of being written
        let temp_fpath = format!("{}.tmp", fname);
        img.save(temp_fpath.to_string()).unwrap();
        fs::rename(temp_fpath, fname).unwrap();


        // Increment the pixel vector
        let mut incr_number = 0;

        while incr_number < iter {
            incr_vec_1bpp(&mut px);
            incr_number = incr_number + 1;
            thread::sleep(Duration::from_millis(1));
        }
    }
}

fn generate_images_rand(w: u32, h: u32, fname: &str) {
    let mut img = Image::new(w, h);
    let mut rng = thread_rng();

    for (x, y) in img.coordinates() {
        let c = rng.gen_range(0, 255);
        img.set_pixel(x, y, px!(c, c, c));
    }

    let temp_fpath = format!("{}.tmp", fname);
    img.save(temp_fpath.to_string()).unwrap();
    fs::rename(temp_fpath, fname).unwrap();
}

fn main() {

    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 240;
    const INCREMENT: u32 = 500;

    const FPATH_8BPP: &str = "/var/www/html/theimage.bmp";
    const FPATH_1BPP: &str = "/var/www/html/theimage_1bpp.bmp";
    const FPATH_RAND: &str = "/var/www/html/theimage_rand.bmp";

    thread::spawn(move || {
        generate_images_8bpp(WIDTH, HEIGHT, INCREMENT, FPATH_8BPP);
    });

    println!("Started 8bpp thread");

    thread::spawn(move || {
        generate_images_1bpp(WIDTH, HEIGHT, INCREMENT, FPATH_1BPP);
    });

    println!("Started 1bpp thread");

    thread::spawn(move || {
        loop {
            generate_images_rand(WIDTH, HEIGHT, FPATH_RAND);
            thread::sleep(Duration::from_millis(500));
        }
    });

    println!("Started random thread");

    loop {
        let min = Duration::from_secs(60);
        thread::sleep(min);
    }

}