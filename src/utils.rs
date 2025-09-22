use std::io::Read;

use crate::Normalization;

pub fn normalize_2d<const L: usize>(
    mut raw: [u32; L],
    mode: Normalization,
    ignore_most_frequent: bool,
) -> [u8; L] {
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
    match mode {
        Normalization::Max => {
            let mul = 255.0 / max;
            raw.map(|val| (val as f32 * mul).round() as u8)
        }
        Normalization::MinMax => {
            let min = *raw.iter().min().unwrap() as f32;
            let mul = 255.0 / (max - min);
            raw.map(|val| ((val as f32 - min) * mul).round() as u8)
        }
    }
}

pub struct File {
    file: std::fs::File,
    buf: Box<[u8; 4194304]>,
    pos: usize,
    end: usize,
}

impl File {
    pub fn new(file: std::fs::File) -> Self {
        Self {
            file,
            buf: Box::new([0; _]),
            pos: 0,
            end: 0,
        }
    }
}

impl Iterator for File {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.end {
            match self.file.read(self.buf.as_mut_slice()) {
                Ok(0) => return None,
                Ok(n) => {
                    self.pos = 0;
                    self.end = n;
                }
                Err(_) => return None,
            }
        }
        let byte = self.buf[self.pos];
        self.pos += 1;
        Some(byte)
    }
}
