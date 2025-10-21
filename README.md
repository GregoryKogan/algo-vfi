# algo-vfi

A comprehensive implementation of multiple optical flow estimation and video frame interpolation algorithms written in Rust. This research project demonstrates various approaches to motion estimation between consecutive video frames and generates interpolated frames for smooth video playback.

[Presentation](vfi/assets/VFI-Pres.mp4)

<p align="center">
  <img src="https://github.com/GregoryKogan/GregoryKogan/blob/main/readme_assets/flow.gif" width=49% />
  <img src="https://github.com/GregoryKogan/algo-vfi/blob/main/vfi/assets/vector-field.png" width=49% />
</p>

*This is a research project code and is not intended for practical use.*

## What is Optical Flow?

Optical flow is the pattern of apparent motion of objects, surfaces, and edges in a visual scene caused by the relative motion between an observer and the scene. In computer vision, optical flow algorithms estimate the motion of pixels between consecutive frames in a video sequence. This information is crucial for:

- **Video Frame Interpolation**: Creating intermediate frames to increase video frame rate
- **Motion Analysis**: Understanding object movement patterns
- **Video Stabilization**: Compensating for camera shake
- **Object Tracking**: Following moving objects across frames

## Implemented Algorithms

This project implements **20 different optical flow estimation algorithms**:

### Block Matching Algorithm (BMA) Variants

- **BMA(8-7)**: Basic block matching with 8×8 blocks and search radius 7
- **BMA(16-7)**: Block matching with 16×16 blocks and search radius 7
- **BDBMA(8-7)**: Bidirectional block matching for improved accuracy
- **BDBMA(16-7)**: Bidirectional block matching with larger blocks
- **SBMA(8-7-3)**: Smoothed block matching with 3×3 filter window
- **SBMA(8-7-5)**: Smoothed block matching with 5×5 filter window
- **SBMA(16-7-3)**: Smoothed block matching with larger blocks
- **SBDBMA(8-7-3)**: Smoothed bidirectional block matching
- **SBDBMA(8-7-5)**: Smoothed bidirectional block matching with larger filter
- **SBDBMA(16-7-3)**: Smoothed bidirectional block matching with larger blocks

### Grayscale BMA Variants

- **GBMA(8-7)**: Grayscale block matching
- **GBDBMA(8-7)**: Grayscale bidirectional block matching
- **GSBMA(8-7-3)**: Grayscale smoothed block matching
- **GSBDBMA(8-7-3)**: Grayscale smoothed bidirectional block matching

### Edge-Enhanced BMA Variants

- **EBMA(8-7)**: Edge-enhanced block matching
- **EBDBMA(8-7)**: Edge-enhanced bidirectional block matching
- **ESBMA(8-7-3)**: Edge-enhanced smoothed block matching
- **ESBDBMA(8-7-3)**: Edge-enhanced smoothed bidirectional block matching

### Classical Optical Flow Methods

- **Lucas-Kanade**: Sparse feature-based optical flow using OpenCV
- **Gunnar Farneback**: Dense optical flow using polynomial expansion

## Prerequisites

Before setting up the project, ensure you have the following installed:

### Required Software

- **Rust** (latest stable version) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo** (comes with Rust)
- **OpenCV 4.x** with C++ support
- **pkg-config** (for OpenCV detection)
- **FFmpeg** (for video processing)
- **Make** (for building external executables)

### Platform-Specific Installation

#### macOS (using Homebrew)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Install dependencies
brew install opencv pkg-config ffmpeg
```

#### Ubuntu/Debian

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Install dependencies
sudo apt update
sudo apt install build-essential pkg-config libopencv-dev ffmpeg
```

#### Windows (using vcpkg)

```bash
# Install Rust
# Download and run rustup-init.exe from https://rustup.rs/

# Install dependencies (requires Visual Studio Build Tools)
# Install vcpkg and OpenCV through vcpkg
# Install FFmpeg from https://ffmpeg.org/download.html
```

## Setup Instructions

### 1. Clone the Repository

```bash
git clone https://github.com/GregoryKogan/algo-vfi.git
cd algo-vfi
```

### 2. Build External Executables

The project uses external C++ executables for Lucas-Kanade and Farneback algorithms:

```bash
cd vfi/src/executables
make
```

This will compile:

- `lucas_kanade` - Lucas-Kanade optical flow implementation
- `farneback` - Gunnar Farneback optical flow implementation

**Note**: If you encounter OpenCV-related errors, ensure OpenCV is properly installed and `pkg-config` can find it.

### 3. Build the Rust Project

```bash
cd ../../  # Return to vfi directory
cargo build --release
```

### 4. Extract Input Frames

The algorithms require input frames extracted from the provided video:

```bash
# Make the script executable and run it
chmod +x scripts/extract_frames.sh
./scripts/extract_frames.sh
```

This extracts 180 PNG frames from `vfi/assets/Bus.mp4` and saves them to `vfi/input/`.

### 5. Run the Algorithms

Execute the main program to run all optical flow algorithms:

```bash
cargo run --release
```

**Note**: This will run all 20 algorithms, which may take several hours depending on your system. The program will:

1. Process 179 frame pairs (frames 1-2, 2-3, ..., 179-180)
2. Generate interpolated frames for each algorithm
3. Create optical flow visualizations
4. Output performance metrics
5. Generate MP4 videos of the results

### 6. View the Results

After completion, results will be available in `vfi/Results/`:

```plaintext
vfi/Results/
├── BMA(8-7)/
│   ├── frames/           # Interpolated frame sequences
│   ├── flow/             # Optical flow visualizations
│   ├── Performance.txt   # Execution time metrics
│   ├── BMA(8-7)-Interpolated30fps.mp4
│   └── BMA(8-7)-Flow30fps.mp4
├── BDBMA(8-7)/
│   └── ...
└── ... (one directory per algorithm)
```

## Understanding the Output

### Interpolated Frames

- **Location**: `Results/{Algorithm}/frames/`
- **Format**: PNG images numbered sequentially
- **Content**: Original frames + interpolated intermediate frames
- **Usage**: Can be combined into videos for smooth playback

### Optical Flow Visualizations

- **Location**: `Results/{Algorithm}/flow/`
- **Format**: PNG images showing motion vectors as colored arrows
- **Color Coding**:
  - Hue represents direction of motion
  - Saturation represents magnitude of motion
- **Usage**: Visual analysis of motion patterns

### Performance Metrics

- **Location**: `Results/{Algorithm}/Performance.txt`
- **Content**: Total execution time and average time per frame
- **Usage**: Algorithm comparison and optimization

### Generated Videos

- **Interpolated Videos**: `{Algorithm}-Interpolated30fps.mp4`
- **Flow Videos**: `{Algorithm}-Flow30fps.mp4`
- **Usage**: Direct playback of results

## Algorithm Selection

By default, the program runs all 20 algorithms. To run specific algorithms, you can modify the `get_every_estimator_setting()` function in `vfi/src/tester.rs`:

```rust
// Example: Run only BMA variants
let estimators = vec![
    estimator_1,  // BMA(8-7)
    estimator_2,  // BDBMA(8-7)
    // ... add other estimators as needed
];
```

## Project Structure

```plaintext
algo-vfi/
├── vfi/                          # Main Rust project
│   ├── src/
│   │   ├── executables/          # C++ optical flow executables
│   │   │   ├── lucas_kanade.cpp
│   │   │   ├── farneback.cpp
│   │   │   └── Makefile
│   │   ├── estimator/            # Optical flow algorithms
│   │   ├── main.rs              # Entry point
│   │   └── tester.rs            # Algorithm runner
│   ├── assets/
│   │   └── Bus.mp4              # Input video
│   ├── input/                   # Extracted frames (generated)
│   └── Results/                 # Algorithm outputs (generated)
├── scripts/
│   └── extract_frames.sh        # Frame extraction script
└── README.md
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
