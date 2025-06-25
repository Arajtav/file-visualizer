use image::{ExtendedColorType, ImageEncoder, ImageResult, codecs::png::PngEncoder};
use std::{fs::File, io::Read};

use crate::Normalization;

const BUFFER_SIZE: usize = 4194304;

pub fn plot(
    mut input: File,
    mut output: File,
    normalization: &Normalization,
    ignore_most_frequent: bool,
) -> ImageResult<()> {
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

    let max = if ignore_most_frequent {
        let mut first_max = 0u32;
        let mut second_max = 0u32;
        for &val in raw.iter() {
            if val > first_max {
                second_max = first_max;
                first_max = val;
                continue;
            }
            if val < first_max && val > second_max {
                second_max = val;
            }
        }
        for val in raw.iter_mut() {
            if *val == first_max {
                *val = second_max;
            }
        }
        second_max
    } else {
        *raw.iter().max().unwrap()
    } as f32;
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
