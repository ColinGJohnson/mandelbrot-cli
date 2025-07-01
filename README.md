
# mandelbrot-cli

![License](https://img.shields.io/github/license/ColinGJohnson/mandelbrot-cli)
![Build status](https://github.com/ColinGJohnson/mandelbrot-cli/actions/workflows/build.yml/badge.svg)

A command line utility to generate visualizations of the [Mandelbrot Set](https://en.wikipedia.org/wiki/Mandelbrot_set).

## Examples

```shell
mandelbrot --zoom 1000.0 --x-res 1920 --y-res 1080
```

![image](https://github.com/user-attachments/assets/494b255b-3996-4a71-ba74-948c26336c00)


```shell
mandelbrot --output double_spiral.png --real-offset=-0.745 --complex-offset=0.1 --zoom 200000.0 --x-res 1920 --y-res 1080 -m 1000
```
![image](https://github.com/user-attachments/assets/1ffea1a7-73a1-4b9e-85df-a8ba44157462)


## Usage

```
Usage: mandelbrot [OPTIONS]

Options:
  -o, --output <OUTPUT>
          Output file path [default: mandelbrot.png]
  -x, --x-res <X_RES>
          Width of the generated image [default: 1000]
  -y, --y-res <Y_RES>
          Height of the generated image [default: 1000]
  -r, --real-offset <REAL_OFFSET>
          Center location on the real (horizontal) axis [default: -0.5]
  -c, --complex-offset <COMPLEX_OFFSET>
          Center location on the imaginary (vertical) axis [default: 0]
  -z, --zoom <ZOOM>
          Zoom factor (pixels per unit distance on complex plane) [default: 300]
  -t, --threshold <THRESHOLD>
          Threshold past which the sequence is assumed to diverge [default: 2]
  -m, --max-iterations <MAX_ITERATIONS>
          Number of iterations before assuming sequence does not diverge [default: 100]
  -s, --samples <SAMPLES>
          Number of samples taken within each pixel, i.e. super-sampling [default: 4]
  -p, --palette <PALETTE>
          Color scheme for the resulting image [default: viridis] [possible values: viridis, black-white, aurora]
      --palette-clamp <PALETTE_CLAMP>
          Percentile after which to consider pixels as having reached the end of the color palette. Avoids a small number of extreme values throwing off the color scale [default: 0.99]
      --smooth
          
  -h, --help
          Print help
  -V, --version
          Print version
```
