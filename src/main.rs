
use clap::Parser;
use num::Complex;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use show_image::{create_window, event};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file path to use instead of the Image preview window.
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

    /// Threshold past width the sequence is assumed to diverge.
    #[arg(short, long, default_value_t = 2.0)]
    threshold: f64,

    /// Number of iterations before assuming sequence does not diverge.
    #[arg(short, long, default_value_t = 100)]
    max_iterations: u32,

    /// Number of worker threads to run the calculation on.
    #[arg(short, long, default_value_t = 1)]
    workers: u32,
}

#[show_image::main]
fn main() {
    let args = Args::parse();
    let offset = Complex::new(args.real_offset, args.complex_offset);
    let center = Complex::new(args.x_res as f64, args.y_res as f64) / args.zoom / 2f64;

    let progress_bar = build_progress_bar((args.x_res * args.y_res) as u64);
    progress_bar.set_message("Sampling Mandelbrot");

    let mut image = RgbImage::new(args.x_res, args.y_res);
    for x in 0..args.x_res {
        for y in 0..args.y_res {
            let color = get_pixel(&args, offset, center, x, y);
            image.put_pixel(x, y, color);
            progress_bar.inc(1);
        }
    }

    match args.output {
        Some(output) => {
            progress_bar.set_message("Saving image");
            image.save(output).unwrap();
        },
        None => {
            progress_bar.set_message("Displaying image");
            show_image(image).unwrap()
        }
    };

    progress_bar.set_message("Saving image");
    progress_bar.finish();
}

fn get_pixel(args: &Args, offset: Complex<f64>, center: Complex<f64>, x: u32, y: u32) -> Rgb<u8> {
    let location: Complex<f64> = pixel_to_complex((x, y), center, offset, args.zoom);
    let color = match sample_mandelbrot(location, args.threshold, args.max_iterations) {
        Some(iterations) => iterations_to_color(iterations, args.max_iterations),
        None => Rgb([0, 0, 0])
    };
    color
}


fn show_image(image: RgbImage)-> Result<(), Box<dyn std::error::Error>> {
    let window = create_window("Mandelbrot", Default::default())?;
    window.set_image("Mandelbrot", image)?;

    // wait until escape button pressed
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed() {
                break;
            }
        }
    }

    Ok(())
}

fn build_progress_bar(len: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(len);
    progress_bar.set_style(
        ProgressStyle::with_template("{msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len}")
            .unwrap()
            .progress_chars("#>-")
    );
    progress_bar
}

fn pixel_to_complex((x, y): (u32, u32), center: Complex<f64>, offset: Complex<f64>, zoom: f64) -> Complex<f64> {
    let sample = Complex::new(x as f64, y as f64) / zoom;
    sample + offset - center
}

fn iterations_to_color(iterations: u32, max_iterations: u32) -> Rgb<u8> {
    let t = iterations as f64 / max_iterations as f64;
    let color = ((1.0 - (t)) * 255.0) as u8;
    Rgb([color, color, color])
}

fn sample_mandelbrot(c: Complex<f64>, threshold: f64, iterations: u32) -> Option<u32> {
    let threshold_squared = threshold * threshold;
    let mut z = Complex::new(0.0, 0.0);
    for iteration in 0..iterations {
        z = (z * z) + c;
        if z.norm_sqr() > threshold_squared {
            return Some(iteration + 1)
        }
    }
    return None;
}
