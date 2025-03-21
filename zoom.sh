#!/bin/bash

set -e
mkdir -p "frames"

ZOOM_STEPS=1000

for i in $(seq 0 $ZOOM_STEPS); do
  factor=$(echo "scale=10; l(1000000.0 / 250.0) * ($i / $ZOOM_STEPS)" | bc -l)
  current_zoom=$(echo "scale=10; 250.0 * e($factor)" | bc -l)
  frame_number=$(printf "%04d" "$i")

  ./mandelbrot \
    --output "frames/frame_$frame_number.png" \
    --x-res 1920 \
    --y-res 1080 \
    --real-offset="0.28" \
    --complex-offset "0.008" \
    --zoom "$current_zoom" \
    --workers "4" \

  echo "Generated frame $i/$ZOOM_STEPS: Zoom = $current_zoom"
done

ffmpeg -y -framerate 30 -i "frames/frame_%04d.png" -c:v libx264 -pix_fmt yuv420p "mandelbrot_zoom.mp4"
