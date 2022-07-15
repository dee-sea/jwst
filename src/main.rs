use rand::seq::IteratorRandom;
use rand::Rng;
use reqwest::blocking::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

mod files;
mod path;

fn download_files(files: Vec<String>) {
    let names = files::get_names();
    for (counter, file) in files.into_iter().enumerate() {
        let filepath = format!("{}/{}", path::get_path(), names[counter]);
        let path = Path::new(&filepath);
        if !path.exists() {
            println!("Downloading {}", names[counter]);
            let url = file.clone();
            let mut resp = get(url).expect("request failed");
            let mut out = File::create(filepath).expect("failed to create file");
            io::copy(&mut resp, &mut out).expect("failed to copy content");
        }
    }
}

fn main() {
    let mut width = 1920;
    let mut height = 1080;
    let images = path::get_path();
    let mut output = "wp.png";
    let mut set_wp = true;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <width> <height> <output_file>", args[0]);
        return;
    } else if args.len() == 4 {
        output = &args[3];
        set_wp = false;
    }
    width = args[1].parse::<u32>().unwrap_or(width);
    height = args[2].parse::<u32>().unwrap_or(height);
    let mut rng = rand::thread_rng();
    /* check if directory images exists */
    let path = Path::new(&images);
    if !path.is_dir() {
        /* if not, create it */
        let res = fs::create_dir_all(images.clone());
        if res.is_err() {
            println!("Failed to create directory {}", images);
            return;
        }
    }
    download_files(files::get_links());
    let files = fs::read_dir(images);
    let files = match files {
        Ok(files) => files,
        Err(_) => {
            println!("Unable to find the images directory");
            return;
        }
    };
    let file = files.choose(&mut rng);
    let file = match file {
        Some(file) => file,
        None => {
            println!("Unable to read files in the images directory");
            return;
        }
    };
    let file = match file {
        Ok(file) => file,
        Err(_) => {
            println!("Unable to read files in the images director");
            return;
        }
    };
    let file = format!("{}", file.path().display());
    let img = image::io::Reader::open(file.clone()).expect("file not found");
    println!("Decoding Image...");
    let img = img.decode();
    let img = match img {
        Ok(img) => img,
        Err(_) => {
            println!("Unable to decode image");
            return;
        }
    };
    let h = img.height();
    let w = img.width();

    let min_resizefactor = if (w as f32 / h as f32) < (width as f32 / height as f32) {
        w / width
    } else {
        h / height
    };
    let rescale_percent: u32 =
        rng.gen::<u32>() % (101 - (100 / min_resizefactor)) + (100 / min_resizefactor);
    let rescale_factor = (rescale_percent as f32 + 5.0) / 100.0;

    println!("Rescaling...");
    let mut rescaled = if min_resizefactor >= 1 {
        image::imageops::resize(
            &img,
            (w as f32 * rescale_factor) as u32,
            (h as f32 * rescale_factor) as u32,
            image::imageops::FilterType::Lanczos3,
        )
    } else {
        image::imageops::resize(
            &img,
            (w as f32) as u32,
            (h as f32) as u32,
            image::imageops::FilterType::Lanczos3,
        )
    };
    let rh = rescaled.height();
    let rw = rescaled.width();
    let mut rng = rand::thread_rng();

    let rand_x: u32 = rng.gen::<u32>() % (rw - width);
    let rand_y: u32 = rng.gen::<u32>() % (rh - height);
    println!("Cropping...");
    let crop = image::imageops::crop(&mut rescaled, rand_x, rand_y, 1920, 1080).to_image();
    crop.save(output).unwrap_or_else(|_| {
        panic!(
            "could not save image: {}
            original size {}x{}
            Rescaled size {}x{}
            rescale factor {}
            crop x {} y {},
            ",
            file, w, h, rw, rh, rescale_factor, rand_x, rand_y
        )
    });
    if set_wp {
        let _ = wallpaper::set_from_path(output);
    }
    let filename = file.split('/').last();
    let filename = match filename {
        Some(filename) => filename,
        None => {
            println!("Unable to get filename");
            return;
        }
    };
    let name = filename.split('.').next();
    let name = match name {
        Some(name) => name,
        None => {
            println!("Unable to get name");
            return;
        }
    };
    let object = name.split('-').next();
    let object = match object {
        Some(object) => object,
        None => {
            println!("Unable to get object");
            return;
        }
    };
    let object = object.replace('_', " ");
    let instrument = name.split('-').nth(1);
    let mut instrument = match instrument {
        Some(instrument) => instrument,
        None => {
            println!("Unable to get instrument");
            return;
        }
    };
    if instrument == "composite" {
        instrument = "nircam + miri";
    }
    println!();
    println!("    Object: {}", object);
    println!("Instrument: {}", instrument);
    println!("Zoom Level: {}%", rescale_percent);
    println!(
        "  Position: X = {} 
            Y = {}",
        rand_x + (width / 2),
        rand_y + (height / 2)
    );
}
