use std::io::Read;
use std::fs::File;
use image::{self, DynamicImage, RgbaImage, Rgba};
use zstd::stream::Decoder;

fn decode_with_zstd(compressed_data: &[u8]) -> Vec<u8> {
    let mut decompressed_data = Vec::new();
    let mut decoder = Decoder::new(compressed_data).expect("Failed to initialize decoder");
    decoder.read_to_end(&mut decompressed_data).expect("Decompression error");
    decompressed_data
}

pub fn decode(file_path: &str) -> DynamicImage {
    let mut file = File::open(format!("{}", file_path.trim())).unwrap();
    let mut compressed_data = Vec::new();
    let height_hex = hex::encode("Height");
    let width_hex: String = hex::encode("Width");
    file.read_to_end(&mut compressed_data).unwrap();

    let raw_data = decode_with_zstd(&compressed_data);

    let data_str = String::from_utf8_lossy(&raw_data);
    let width: u32 = data_str.lines()
        .find(|line| line.contains(&width_hex)).unwrap()
        .split(':').nth(1).unwrap().trim().parse().unwrap();
    let height: u32 = data_str.lines()
        .find(|line| line.contains(&height_hex)).unwrap()
        .split(':').nth(1).unwrap().trim().parse().unwrap();
    let signature = data_str.lines()
        .find(|line| line.contains("0x56-0x46-0x58"))
        .and_then(|line| line.split(':').nth(1))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| panic!("Signature verification error"));

    if signature != "0x03" {
        println!("The file version is old. Please consider updating.");
    }

    let img_data = &raw_data[..raw_data.len() - (width.to_string().len() + &width.to_string().len() + 16)];
    let mut img_pixels = Vec::new();

    for chunk in img_data.chunks(4) {
        if chunk.len() == 4 {
            img_pixels.push(Rgba([chunk[0], chunk[1], chunk[2], chunk[3]]));
        }
    }

    let img = RgbaImage::from_raw(width, height, img_pixels.into_iter()
        .flat_map(|p| p.0.to_vec())
        .collect())
        .unwrap_or_else(|| panic!("Failed to create image."));

    DynamicImage::ImageRgba8(img)
}
