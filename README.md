
# rust-mandelbrot

A command line utility to generate visualizations of the [Mandelbrot Set](https://en.wikipedia.org/wiki/Mandelbrot_set).

## Example

```shell
./mandelbrot --output zoom_center.png --zoom 1000.0 --x-res 1920 --y-res 1080
```

![image](https://github.com/user-attachments/assets/343a88d4-9e91-4c59-aa62-c7cd7167e301)

## Usage

```
rust-mandelbrot.exe [OPTIONS]

  -z, --zoom <ZOOM>
          Zoom factor (pixels per unit distance on complex plane) [default: 250]
  -t, --threshold <THRESHOLD>
          Threshold past width the sequence is assumed to diverge [default: 2]
  -m, --max-iterations <MAX_ITERATIONS>
          Number of iterations before assuming sequence does not diverge [default: 100]
  -w, --workers <WORKERS>
          Number of worker threads to run the calculation on [default: 1]
  -h, --help
          Print help
  -V, --version
          Print version
```
