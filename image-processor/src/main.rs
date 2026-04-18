use clap::Parser;
use image::ImageReader;
use std::{
    ffi::CString,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::plugin_loader::Plugin;

mod plugin_loader;

#[derive(Parser)]
#[command(name = "image-processor", about = "Image processor cli", version)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(long)]
    plugin: String,
    #[arg(long)]
    params: String,
    #[arg(long)]
    plugin_path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let Cli {
        input,
        output,
        plugin,
        params,
        plugin_path,
    } = cli;

    let plugin_path = plugin_path.unwrap_or("target/debug".into());

    println!(
        "Params: {} {} {} {} {}",
        input, output, plugin, params, plugin_path
    );

    let img = ImageReader::open("bg.png").unwrap().decode().unwrap();
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let mut raw = rgba.into_raw();

    let params = read_params(&params).unwrap_or(String::new());

    let c_params = CString::new(params).unwrap();
    let plugin = Plugin::new(&format!("{}/{}", plugin_path, plugin)).unwrap();
    let plugin = plugin.interface().unwrap();
    (plugin.process_image)(width, height, raw.as_mut_ptr(), c_params.as_ptr());

    let result = image::RgbaImage::from_raw(width, height, raw).unwrap();
    result.save(output).unwrap();
}

fn read_params(from_file: &str) -> Option<String> {
    let file = match File::open(from_file) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();

    let params = line.trim();
    if params.is_empty() {
        None
    } else {
        Some(params.to_string())
    }
}
