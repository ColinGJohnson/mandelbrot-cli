use image::{Rgb, RgbImage};
use rayon::prelude::*;
use crate::mandelbrot::palette::{sample_palette, PresetPalette};

const DID_NOT_CONVERGE: Rgb<u8> = Rgb([0, 0, 0]);

pub fn create_image(palette: PresetPalette, x_res: u32, y_res: u32,
                    mandelbrot_data: Vec<Vec<Option<f64>>>) -> RgbImage {

    let mut flattened: Vec<f64> = mandelbrot_data.iter()
        .flatten()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    flattened.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = *flattened.get(0).unwrap();
    let max = *flattened.get(((flattened.len() - 1) as f32 * 0.990) as usize).unwrap();

    let mut image = RgbImage::new(x_res, y_res);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = match mandelbrot_data[x as usize][y as usize] {
            Some(iterations) => {
                let scaled = scale_value(iterations, min, max);
                sample_palette(palette, scaled)
            },
            None => DID_NOT_CONVERGE
        };
        *pixel = color;
    }
    image
}

fn scale_value(value: f64, min: f64, max: f64) -> f64 {
    if min == max {
        return 0.0
    }
    (num::clamp(value, min, max) - min) / (max - min)
}
