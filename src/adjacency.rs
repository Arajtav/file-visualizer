use std::fs::File;

use image::{ExtendedColorType, ImageEncoder, ImageResult, codecs::png::PngEncoder};

use crate::{Normalization, utils::normalize_2d};

pub fn adjacency(
    input: File,
    mut output: File,
    normalization: Normalization,
    ignore_most_frequent: bool,
) -> ImageResult<()> {
    let mut input = crate::utils::File::new(input);

    let mut raw = [0u32; 256 * 256];

    if let Some(mut prev) = input.next() {
        for b in input {
            let x = b as usize;
            let y = prev as usize;
            raw[(x << 8) | y] += 1;
            prev = b;
        }
    }

    PngEncoder::new(&mut output).write_image(
        &normalize_2d(raw, normalization, ignore_most_frequent),
        256,
        256,
        ExtendedColorType::L8,
    )
}
