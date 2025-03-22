use clap::{arg, Parser};
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use num::Complex;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file path.
    #[arg(short, long)]
    output: Option<String>,

    /// Width of the generated image.
    #[arg(short, long, default_value_t = 1000)]
    x_res: u32,

    /// Height of the generated image.
    #[arg(short, long, default_value_t = 1000)]
    y_res: u32,

    /// Center location on the real axis.
    #[arg(short, long, default_value_t = -1.0)]
    real_offset: f64,

    /// Center location on the imaginary axis.
    #[arg(short, long, default_value_t = 0.0)]
    complex_offset: f64,

    /// Zoom factor (pixels per unit distance on complex plane).
    #[arg(short, long, default_value_t = 250.0)]
    zoom: f64,

    /// Threshold past which the sequence is assumed to diverge.
    #[arg(short, long, default_value_t = 2.0)]
    threshold: f64,

    /// Number of iterations before assuming sequence does not diverge.
    #[arg(short, long, default_value_t = 100)]
    max_iterations: u32,

    /// Number of worker threads to run the calculation on.
    #[arg(short, long, default_value_t = 1)]
    workers: usize,

    /// Percentile after which to consider pixels as having reached the end of the color palette.
    /// Avoids a small number of extreme values throwing off the color scale.
    #[arg(short, long, default_value_t = 1.0)]
    palette_clamp: f64,
}

#[derive(Copy, Clone)]
struct PixelLocation {
    x: u32,
    y: u32,
}

fn main() {
    // black/white
    // let palette = vec!("#FFFFFF", "#000000");

    // virdis
    let palette = vec!("#000000", "#440c54", "#47337e", "#365c8d", "#277f8e", "#1ea187", "#49c26c", "#9eda3a", "#9eda3a");

    // aurora
    // let palette = vec![
    //     "#001a33", "#003d66", "#007f7f", "#00b34d", "#b3ef00",
    //     "#ffd966", "#ff6600", "#99004d", "#330033"
    // ];


    let palette_rgb = palette.iter()
        .map(|color| convert_hex_to_rgb(color))
        .collect::<Vec<[u8; 3]>>();
    let did_not_converge = Rgb(convert_hex_to_rgb("#000000"));

    let now = Instant::now();
    let args = Args::parse();
    let offset = Complex::new(args.real_offset, args.complex_offset);
    let center = (Complex::new(args.x_res as f64, args.y_res as f64) / args.zoom) / 2f64;

    let progress_bar = build_progress_bar((args.x_res * args.y_res) as u64);
    progress_bar.set_message("Sampling Mandelbrot");

    let mut min = args.max_iterations as f64;
    let mut max = 0f64;

    let mut mandelbrot_data = vec![vec![None; args.y_res as usize]; args.x_res as usize];
    let mut flattened = Vec::new();

    for x in 0..args.x_res {
        for y in 0..args.y_res {
            let complex_location: Complex<f64> = pixel_to_complex(PixelLocation { x, y }, center, offset, args.zoom);
            let sample = sample_mandelbrot(complex_location, args.threshold, args.max_iterations);
            mandelbrot_data[x as usize][y as usize] = sample;
            if sample.is_some() {
                flattened.push(sample.unwrap());
                if sample.unwrap() < min {
                    min = sample.unwrap()
                }
                if sample.unwrap() > max {
                    max = sample.unwrap()
                }
            }
            progress_bar.inc(1);
        }
    }

    flattened.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q3 = *flattened.get(((flattened.len() - 1) as f32 * 0.995) as usize).unwrap();
    println!("min: {min}, max: {max}, percentile: {q3}");

    let output_path = args.output.unwrap_or_else(|| "mandelbrot.png".to_string());
    progress_bar.set_message(format!("Saving image as {}", output_path));

    let mut image = RgbImage::new(args.x_res, args.y_res);
    let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(args.workers).build().unwrap();
    thread_pool.install(|| {
        image.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
            let color = match mandelbrot_data[x as usize][y as usize] {
                Some(iterations) => {
                    let scaled = scale_value(iterations, min, q3);
                    sample_palette(&palette_rgb, scaled)
                },
                None => did_not_converge
            };
            *pixel = color;
            progress_bar.inc(1);
        });
    });

    match image.save(output_path) {
        Err(e) => {
            println!("Error saving image {}", e)
        }
        _ => {}
    }

    progress_bar.set_message("Done");
    progress_bar.finish();

    let elapsed = now.elapsed().as_millis();
    println!("Elapsed time: {}ms", elapsed);
}

fn scale_value(value: f64, min: f64, max: f64) -> f64 {
    if min == max {
        return 0.0
    }
    (num::clamp(value, min, max) - min) / (max - min)
}

/// Construct a progress bar with a custom style.
fn build_progress_bar(len: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(len);
    progress_bar.set_style(
        ProgressStyle::with_template("{msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len}")
            .unwrap()
            .progress_chars("#>-")
    );
    progress_bar
}

/// Convert a pixel location to a location on the complex plane.
fn pixel_to_complex(location: PixelLocation, center: Complex<f64>, offset: Complex<f64>, zoom: f64) -> Complex<f64> {
    let sample = Complex::new(location.x as f64, location.y as f64) / zoom;
    sample + offset - center
}

fn sample_palette(palette: &Vec<[u8; 3]>, t: f64) -> Rgb<u8> {
    let scaled_t = t * (palette.len() as f64 - 1.0);
    let index = scaled_t.floor() as usize;
    let next_index = (index + 1).min(palette.len() - 1);
    let local_t = scaled_t - index as f64;

    interpolate_rgb(
        Rgb(palette[index]),
        Rgb(palette[next_index]),
        local_t
    )
}

fn interpolate_rgb(a: Rgb<u8>, b: Rgb<u8>, t: f64) -> Rgb<u8> {
    Rgb([
        interpolate(a[0], b[0], t) as u8,
        interpolate(a[1], b[1], t) as u8,
        interpolate(a[2], b[2], t) as u8,
    ])
}

fn interpolate(a: u8, b: u8, t: f64) -> f64 {
    a as f64 + (b as f64 - a as f64) * t
}

fn convert_hex_to_rgb(hex: &str) -> [u8; 3] {
    let trimmed = hex.trim_start_matches('#');
    [
        u8::from_str_radix(&trimmed[0..2], 16).unwrap(),
        u8::from_str_radix(&trimmed[2..4], 16).unwrap(),
        u8::from_str_radix(&trimmed[4..6], 16).unwrap()
    ]
}

/// Sample the mandelbrot set at the given location.
/// Returns num iterations before the sequence diverged, or None if the sequence did not diverge.
fn sample_mandelbrot(c: Complex<f64>, threshold: f64, max_iterations: u32) -> Option<f64> {
    let mut z = Complex::new(0.0, 0.0);
    for iteration in 0..max_iterations {
        z = (z * z) + c;
        if f64::hypot(z.re, z.im) > threshold {
            // return Some((iteration + 1) as f64)
            return Some(smooth_color(iteration, z));
        }
    }
    None
}

fn smooth_color(iteration: u32, z: Complex<f64>) -> f64 {
    iteration as f64 + 1.0 - ((z.norm().ln() / 2.0_f64.ln()).ln() / 2.0_f64.ln())
}
