use image::{ExtendedColorType, ImageEncoder, ImageResult, codecs::png::PngEncoder};
use std::{fs::File, io::Read};

const BUFFER_SIZE: usize = 4194304;

pub fn plot(mut input: File, mut output: File) -> ImageResult<()> {
    let mut raw = [0u32; 256 * 256];
    let mut buf = [0u8; BUFFER_SIZE];

    loop {
        let len = input.read(&mut buf)? & !1;
        if len == 0 {
            break;
        }
        for pair in buf[..len].chunks(2) {
            let x = pair[1] as usize;
            let y = pair[0] as usize;
            raw[(x << 8) + y] += 1;
        }
    }

    let max = *raw.iter().max().unwrap() as f32;
    let data = raw.map(|val| (val as f32 * 255.0 / max).round() as u8);

    PngEncoder::new(&mut output).write_image(&data, 256, 256, ExtendedColorType::L8)
}
