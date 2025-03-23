use crate::Args;
use indicatif::ProgressBar;
use num::Complex;
use rand::Rng;
use rayon::prelude::*;

#[derive(Copy, Clone)]
struct Pixel {
    x: u32,
    y: u32,
}

pub struct SampleResult {
    pub x_res: u32,
    pub y_res: u32,
    pub grid: Vec<Vec<Option<f64>>>,
}

pub fn sample_grid(args: &Args, progress_bar: &ProgressBar) -> SampleResult {
    let offset = Complex::new(args.real_offset, args.complex_offset);
    let center = (Complex::new(args.x_res as f64, args.y_res as f64) / args.zoom) / 2f64;
    let mut result = vec![vec![None; args.y_res as usize]; args.x_res as usize];

    result.par_iter_mut().enumerate().for_each(|(x, column)| {
        for y in 0..args.y_res {
            let sample = sample_pixel(args, offset, center, x as u32, y);
            column[y as usize] = sample;
            if y % 100 == 0 {
                progress_bar.inc(100);
            }
        }
    });
    progress_bar.finish();

    SampleResult {
        x_res: args.x_res,
        y_res: args.y_res,
        grid: result,
    }
}

fn sample_pixel(args: &Args, offset: Complex<f64>, center: Complex<f64>, x: u32, y: u32) -> Option<f64> {
    let location: Complex<f64> = pixel_to_complex(Pixel { x, y }, center, offset, args.zoom);
    let half_pixel = (1.0 / args.zoom) / 2f64;
    super_sample_mandelbrot(args.samples, half_pixel, location, args.threshold, args.max_iterations)
}

/// Convert a pixel location to a location on the complex plane.
fn pixel_to_complex(location: Pixel, center: Complex<f64>, offset: Complex<f64>, zoom: f64) -> Complex<f64> {
    let sample = Complex::new(location.x as f64, location.y as f64) / zoom;
    sample + offset - center
}

/// Returns the average of multiple samples within a given range
/// https://en.wikipedia.org/wiki/Supersampling.
fn super_sample_mandelbrot(samples: u32, range: f64, c: Complex<f64>, threshold: f64,
                           max_iterations: u32) -> Option<f64> {
    let mut sum = 0f64;
    let mut diverged_samples = 0;

    for _ in 0..samples - 1 {
        let sample = sample_mandelbrot(super_sample(c, range), threshold, max_iterations);
        if sample.is_some() {
            sum += sample.unwrap();
            diverged_samples += 1
        }
    }

    if diverged_samples > 0 {
        Some(sum / diverged_samples as f64)
    } else {
        None
    }
}

fn super_sample(c: Complex<f64>, range: f64) -> Complex<f64> {
    let mut rng = rand::rng();
    let re = rng.random_range(-range..range);
    let im = rng.random_range(-range..range);
    c + Complex::new(re, im)
}

/// Sample the mandelbrot set at the given location.
/// Returns num iterations before the sequence diverged, or None if the sequence did not diverge.
fn sample_mandelbrot(c: Complex<f64>, threshold: f64, max_iterations: u32) -> Option<f64> {
    let mut z = Complex::new(0.0, 0.0);
    for iteration in 0..max_iterations {
        z = (z * z) + c;
        if f64::hypot(z.re, z.im) > threshold {
            // return Some((iteration + 1) as f64)
            return Some(smooth_iteration(iteration, z));
        }
    }
    None
}

fn smooth_iteration(iteration: u32, z: Complex<f64>) -> f64 {
    iteration as f64 + 1.0 - ((z.norm().ln() / 2.0_f64.ln()).ln() / 2.0_f64.ln())
}
