#!/bin/bash

# Extract frames from Bus.mp4 video
# This script generates the 180 PNG frames needed for optical flow analysis

VIDEO_PATH="vfi/assets/Bus.mp4"
OUTPUT_DIR="vfi/input"

echo "Extracting frames from $VIDEO_PATH..."

# Check if video exists
if [ ! -f "$VIDEO_PATH" ]; then
    echo "Error: Video file $VIDEO_PATH not found!"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Extract frames using ffmpeg
# -r 30: extract at 30 fps (adjust based on video framerate)
# -vf fps=30: force 30 fps output
# -q:v 2: high quality (1-31, lower is better quality)
# -frames:v 180: extract exactly 180 frames
echo "Extracting 180 frames at 30fps..."
ffmpeg -i "$VIDEO_PATH" -vf fps=30 -frames:v 180 -q:v 2 "$OUTPUT_DIR/%d.png" -y

# Verify extraction
FRAME_COUNT=$(ls -1 "$OUTPUT_DIR"/*.png 2>/dev/null | wc -l)
echo "Extracted $FRAME_COUNT frames to $OUTPUT_DIR"

if [ "$FRAME_COUNT" -eq 180 ]; then
    echo "✅ Successfully extracted all 180 frames!"
else
    echo "⚠️  Warning: Expected 180 frames, but found $FRAME_COUNT frames"
fi

echo "Frame extraction complete!"
