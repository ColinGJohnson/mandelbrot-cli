
use clap::Parser;
use num::Complex;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file path to use instead of the Image preview window.
    path: Option<std::path::PathBuf>,

    /// Width of the generated image.
    #[arg(short, long, default_value_t = 800)]
    x_res: u32,

    /// Height of the generated image.
    #[arg(short, long, default_value_t = 800)]
    y_res: u32,

    /// Center location on the real axis.
    #[arg(short, long, default_value_t = 0f64)]
    real_center: f64,

    /// Center location on the imaginary axis.
    #[arg(short, long, default_value_t = 0f64)]
    complex_center: f64,

    /// Zoom factor (pixels per unit distance on complex plane).
    #[arg(short, long, default_value_t = 800f64)]
    zoom: f64,

    /// Threshold past width the sequence is assumed to diverge.
    #[arg(short, long, default_value_t = 2f64)]
    threshold: f64,

    /// Number of iterations before assuming sequence does not diverge.
    #[arg(short, long, default_value_t = 100u32)]
    iterations: u32,

    /// Number of worker threads to run the calculation on.
    #[arg(short, long, default_value_t = 1)]
    workers: u16,
}

fn main() {
    let args = Args::parse();
    println!("{zoom}", zoom = args.zoom);
    println!("{width}", width = args.x_res);

    let location = Complex::new(args.real_center, args.complex_center);
    match sample_mandelbrot(location, args.threshold, args.iterations) {
        Some(iterations) => println!("{iterations}"),
        None => println!("Did not diverge.")
    }


}

fn sample_mandelbrot(c: Complex<f64>, threshold: f64, iterations: u32) -> Option<u32> {
    let threshold_squared = threshold * threshold;
    let mut z = Complex::new(0f64, 0f64);
    for iteration in 0..iterations {
        z = (z * z) + c;
        if z.norm_sqr() > threshold_squared {
            return Some(iteration + 1)
        }
    }
    return None;
}
