use image::{ExtendedColorType, ImageEncoder, ImageResult, codecs::png::PngEncoder};
use std::{fs::File, io::Read};

use crate::Normalization;

const BUFFER_SIZE: usize = 4194304;

pub fn plot(mut input: File, mut output: File, normalization: &Normalization) -> ImageResult<()> {
    let mut raw = [0u32; 256 * 256];
    let mut buf = [0u8; BUFFER_SIZE];

    loop {
        if input.read(&mut buf)? < 2 {
            break;
        }
        for pair in buf.chunks_exact(2) {
            let x = pair[1] as usize;
            let y = pair[0] as usize;
            raw[(x << 8) | y] += 1;
        }
    }
    let max = *raw.iter().max().unwrap() as f32;
    let data = match normalization {
        Normalization::Max => {
            let mul = 255.0 / max;
            raw.map(|val| (val as f32 * mul).round() as u8)
        }
        Normalization::MinMax => {
            let min = *raw.iter().min().unwrap() as f32;
            let mul = 255.0 / (max - min);
            raw.map(|val| ((val as f32 - min) * mul).round() as u8)
        }
    };

    PngEncoder::new(&mut output).write_image(&data, 256, 256, ExtendedColorType::L8)
}
