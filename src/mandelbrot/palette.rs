use image::Rgb;

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum PresetPalette {
    #[default]
    Viridis,
    BlackWhite,
    Aurora,
}

pub fn sample_palette(palette: &PresetPalette, t: f64) -> Rgb<u8> {
    let palette_rgb = get_palette(palette);

    let scaled_t = t * (palette_rgb.len() as f64 - 1.0);
    let index = scaled_t.floor() as usize;
    let next_index = (index + 1).min(palette_rgb.len() - 1);
    let local_t = scaled_t - index as f64;

    interpolate_rgb(
        Rgb(palette_rgb[index]),
        Rgb(palette_rgb[next_index]),
        local_t
    )
}

fn get_palette(palette: &PresetPalette) -> Vec<[u8; 3]> {
    palette_to_rgb(match palette {
        PresetPalette::BlackWhite => vec![
            "#FFFFFF", "#000000"
        ],
        PresetPalette::Viridis => vec![
            "#000000", "#440c54", "#47337e", "#365c8d", "#277f8e", "#1ea187", "#49c26c", "#9eda3a", "#9eda3a"
        ],
        PresetPalette::Aurora => vec![
            "#001a33", "#003d66", "#007f7f", "#00b34d", "#b3ef00", "#ffd966", "#ff6600", "#99004d", "#330033"
        ],
    })
}

fn palette_to_rgb(palette: Vec<&str>) -> Vec<[u8; 3]> {
    palette.iter()
        .map(|color| hex_to_rgb(color))
        .collect::<Vec<[u8; 3]>>()
}

fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let trimmed = hex.trim_start_matches('#');
    [
        u8::from_str_radix(&trimmed[0..2], 16).unwrap(),
        u8::from_str_radix(&trimmed[2..4], 16).unwrap(),
        u8::from_str_radix(&trimmed[4..6], 16).unwrap()
    ]
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
