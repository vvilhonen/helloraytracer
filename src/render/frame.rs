use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Frame {
    pub nx: usize,
    pub ny: usize,
    pub data: Vec<u8>,
}

impl Frame {
    pub fn new(nx: usize, ny: usize, data: Vec<u8>) -> Self {
        Self { nx, ny, data }
    }

    pub fn write_to_png<P: AsRef<Path>>(&self, path: P) {
        let f = File::create(path).unwrap();
        let w = BufWriter::new(f);
        let mut encoder = png::Encoder::new(w, self.nx as u32, self.ny as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let mut data_flipped_y = self.data.clone();
        for y in 0..self.ny {
            for x in 0..self.nx {
                let idx_orig = calc_idx(x, self.nx, y, 3);
                let idx_new = calc_idx(x, self.nx, self.ny - y - 1, 3);
                for field in 0..3 {
                    data_flipped_y[idx_new + field] = self.data[idx_orig + field];
                }
            }
        }
        writer.write_image_data(data_flipped_y.as_slice()).unwrap();
    }

    pub fn fill_u32_buf(&self, buf: &mut Vec<u32>) {
        for y in 0..self.ny {
            for x in 0..self.nx {
                let idx = calc_idx(x, self.nx, y, 3);
                let pixel = (self.data[idx + 0] as u32) << 16
                    | (self.data[idx + 1] as u32) << 8
                    | (self.data[idx + 2] as u32);
                buf[calc_idx(x, self.nx, self.ny - y - 1, 1)] = pixel;
            }
        }
    }
}

fn calc_idx(x: usize, nx: usize, y: usize, n: usize) -> usize {
    (y * nx + x) * n
}
