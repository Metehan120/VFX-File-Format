use std::io::Read;
use std::fs::File;
use image::{self, DynamicImage, RgbaImage, Rgba};
use zstd::stream::Decoder;

fn decode_with_zstd(compressed_data: &[u8]) -> Vec<u8> {
    println!("Çözme işlemi başlatıldı. Sıkıştırılmış veri boyutu: {}", compressed_data.len());
    let mut decompressed_data = Vec::new();
    let mut decoder = Decoder::new(compressed_data).expect("Çözücü başlatılamadı");
    decoder.read_to_end(&mut decompressed_data).expect("Çözme hatası");
    println!("Çözme sonrası veri boyutu: {}", decompressed_data.len());
    decompressed_data
}

pub fn decode(file_path: &str) -> DynamicImage {
    let mut file = File::open(format!("{}.vfx", file_path.trim())).unwrap();
    let mut compressed_data = Vec::new();
    file.read_to_end(&mut compressed_data).unwrap();

    let raw_data = decode_with_zstd(&compressed_data);

    let data_str = String::from_utf8_lossy(&raw_data);
    let width: u32 = data_str.lines()
        .find(|line| line.contains("Width:")).unwrap()
        .split(':').nth(1).unwrap().trim().parse().unwrap();
    let height: u32 = data_str.lines()
        .find(|line| line.contains("Height:")).unwrap()
        .split(':').nth(1).unwrap().trim().parse().unwrap();

    let img_data = &raw_data[..raw_data.len() - (width.to_string().len() + height.to_string().len() + 16)];
    let mut img_pixels = Vec::new();

    for chunk in img_data.chunks(4) {
        if chunk.len() == 4 {
            img_pixels.push(Rgba([chunk[0], chunk[1], chunk[2], chunk[3]]));
        }
    }

    let img = RgbaImage::from_raw(width, height, img_pixels.into_iter()
        .flat_map(|p| p.0.to_vec())
        .collect())
        .unwrap_or_else(|| panic!("Görsel oluşturulamadı."));

    DynamicImage::ImageRgba8(img)
}