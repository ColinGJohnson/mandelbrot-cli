mod mandelbrot;

use crate::mandelbrot::palette::PresetPalette::Viridis;
use crate::mandelbrot::render::create_image;
use crate::mandelbrot::sample::sample_grid;
use clap::{arg, Parser};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file path.
    #[arg(short, long, default_value_t)]
    output: String,

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
    #[arg(short, long, default_value_t = 300.0)]
    zoom: f64,

    /// Threshold past which the sequence is assumed to diverge.
    #[arg(short, long, default_value_t = 2.0)]
    threshold: f64,

    /// Number of iterations before assuming sequence does not diverge.
    #[arg(short, long, default_value_t = 100)]
    max_iterations: u32,

    /// Number of samples taken within each pixel, i.e. super-sampling.
    #[arg(short, long, default_value_t = 4)]
    samples: u32,

    /// Number of worker threads to run the calculation on.
    #[arg(short, long, default_value_t = 1)]
    workers: usize,

    /// Percentile after which to consider pixels as having reached the end of the color palette.
    /// Avoids a small number of extreme values throwing off the color scale.
    #[arg(short, long, default_value_t = 1.0)]
    palette_clamp: f64,
}

fn main() {
    let args = Args::parse();
    let progress_bar = build_progress_bar((args.x_res * args.y_res) as u64);

    progress_bar.set_message("Sampling Mandelbrot");
    let data = sample_grid(&args, &progress_bar);

    progress_bar.set_message("Converting to image".to_string());
    let image = create_image(Viridis, args.x_res, args.y_res, data);

    progress_bar.set_message(format!("Saving image as {}", args.output));
    match image.save(args.output) {
        Err(e) => {
            println!("Error saving image {}", e)
        }
        _ => {}
    }

    progress_bar.set_message("Done");
    progress_bar.finish();
}

fn build_progress_bar(len: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(len);
    let template = "{msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len}";
    progress_bar.set_style(
        ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar
}
