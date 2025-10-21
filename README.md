# algo-vfi

Implementation of multiple optical flow estimation / video frame interpolation algorithms written in Rust

[Presentation](vfi/assets/VFI-Presentation.webm)

<p align="center">
  <img src="https://github.com/GregoryKogan/GregoryKogan/blob/main/readme_assets/flow.gif" width=49% />
  <img src="https://github.com/GregoryKogan/algo-vfi/blob/main/vfi/assets/vector-field.png" width=49% />
<p/>

*This is a research project code and is not intended for practical use.*

## Building and Running

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- FFmpeg (for extracting input frames from video)

### Building Executables

To build the optical flow estimation executables:

```bash
cd vfi
cargo build --release
```

The compiled executables will be created in `vfi/src/executables/`:

- `farneback` - Gunnar Farneback optical flow algorithm
- `lucas_kanade` - Lucas-Kanade optical flow algorithm

### Generating Input Frames

The optical flow algorithms require input frames extracted from the Bus.mp4 video:

```bash
# Extract 180 frames from the video
chmod +x scripts/extract_frames.sh
./scripts/extract_frames.sh
```

This will generate 180 PNG frames in the `vfi/input/` directory from the `vfi/assets/Bus.mp4` video.

### Generating Results

After building the executables and generating input frames, you can run the optical flow algorithms:

```bash
# Create results directory
mkdir -p vfi/Results

# Run the algorithms on your input images
./vfi/src/executables/farneback
./vfi/src/executables/lucas_kanade
```

The generated MP4 videos will be saved in the `vfi/Results/` directory.

**Note:** The `vfi/Results/` directory, `vfi/input/` directory, and compiled executables are excluded from version control to keep the repository size manageable. They can be regenerated locally after cloning.

MEPHI 2022
