use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use crate::mandelbrot::palette::{sample_palette, PresetPalette};
use crate::mandelbrot::sample::SampleResult;

const DID_NOT_CONVERGE: Rgb<u8> = Rgb([0, 0, 0]);

pub fn create_image(palette: PresetPalette, palette_clamp: f64, data: SampleResult, progress_bar: &ProgressBar) -> RgbImage {
    let mut flattened: Vec<f64> = data.grid.iter()
        .flatten()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    flattened.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let palette_min = *flattened.get(0)
        .unwrap_or(&0f64);
    let palette_max = *flattened.get(((flattened.len() - 1) as f64 * palette_clamp) as usize)
        .unwrap_or(&0f64);

    let mut image = RgbImage::new(data.x_res, data.y_res);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = match data.grid[x as usize][y as usize] {
            Some(iterations) => {
                let scaled = scale_value(iterations, palette_min, palette_max);
                sample_palette(&palette, scaled)
            },
            None => DID_NOT_CONVERGE
        };
        *pixel = color;
        if y % 100 == 0 {
            progress_bar.inc(100);
        }
    }
    image
}

fn scale_value(value: f64, min: f64, max: f64) -> f64 {
    if min == max {
        return 0.0
    }
    (num::clamp(value, min, max) - min) / (max - min)
}
