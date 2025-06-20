use image::{ExtendedColorType, ImageEncoder, ImageResult, codecs::png::PngEncoder};
use std::{fs::File, io::Read};

const BUFFER_SIZE: usize = 4194304;

pub fn plot(mut input: File, mut output: File) -> ImageResult<()> {
    let mut raw = [[0u32; 256]; 256];
    let mut buf = [0u8; BUFFER_SIZE];

    loop {
        let len = input.read(&mut buf)? & !1;
        if len == 0 {
            break;
        }
        for pair in buf[..len].chunks(2) {
            raw[pair[0] as usize][pair[1] as usize] += 1;
        }
    }

    let max = *raw.iter().flatten().max().unwrap() as f64;
    let data: Vec<u8> = raw
        .iter()
        .flatten()
        .map(|&val| (val as f64 * 255.0 / max).round() as u8)
        .collect();

    PngEncoder::new(&mut output).write_image(&data, 256, 256, ExtendedColorType::L8)
}
