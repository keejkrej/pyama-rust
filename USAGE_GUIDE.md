# 6D File Generation and Loading Guide

This guide shows how to generate mock 6D microscopy files and load them using the utilities in this project.

## 6D File Format

The 6D format uses two files:
- `.meta` file: JSON metadata with dimensions, channel names, pixel size, etc.
- `.data` file: Raw binary data (f32 values)

Dimensions follow TPZCYX convention:
- **T**: Time points
- **P**: Positions  
- **Z**: Z-stack depth
- **C**: Channels
- **Y**: Height (pixels)
- **X**: Width (pixels)

## Quick Start

### 1. Generate a Mock File

```bash
cargo run --example generate_6d
```

This creates `test_data.meta` and `test_data.data` (3×1×2×2×32×32 dimensions).

### 1b. Generate Noise Data (Alternative)

```bash
cargo run --example generate_noise
```

This creates noise datasets for testing noise reduction algorithms and image quality metrics.

### 2. Load the File

**Option A: Using the GUI**
1. Run the main app: `cargo run`
2. Navigate to the Data panel
3. Click "Load 6D Data" to open a file browser
4. Select your generated `test_data.meta` file

**Option B: Using the command line example**
```bash
cargo run --example load_6d
```

This validates and displays detailed information about the generated file.

### 2. Use Functions Directly in Code

```rust
use pyama_rust::utils::*;

// Generate a small test file
generate_small_test_file("my_test.meta")?;

// Generate a custom file
generate_mock_6d_file(
    "custom.meta",
    5,    // time points
    1,    // positions
    3,    // z slices
    2,    // channels
    128,  // height
    128   // width
)?;

// Generate 2D noise data for algorithm testing
generate_2d_noise_file(
    "noise.meta",
    256,   // width
    256,   // height
    100.0, // min intensity
    800.0  // max intensity
)?;

// Load and inspect
load_and_inspect_6d_file("my_test.meta")?;

// Just validate without loading all data
validate_6d_file("my_test.meta")?;
```

## Available Functions

### Generation Functions

- `generate_small_test_file(path)` - Creates 3×1×2×2×32×32 test file
- `generate_realistic_dataset(path)` - Creates 10×1×5×3×256×256 realistic file  
- `generate_mock_6d_file(path, t, p, z, c, h, w)` - Custom dimensions with mixed patterns
- `generate_custom_pattern_6d_file(path, t, p, z, h, w, patterns)` - Custom patterns per channel
- `generate_2d_noise_file(path, w, h, min, max)` - 2D noise data for algorithm testing

### Loading Functions

- `load_and_inspect_6d_file(path)` - Load and display detailed info
- `validate_6d_file(path)` - Quick validation without loading data

## Pattern Types

The generator creates different patterns for each channel:

1. **Uniform** - Flat intensity across all pixels
2. **Gradient** - Linear intensity gradient
3. **Gaussian Spots** - Random bright spots with Gaussian profile
4. **Circles** - Concentric circles pattern
5. **Moving Spots** - Spots that move over time
6. **Sine Wave** - Sinusoidal wave pattern
7. **Noise** - Random noise with configurable intensity range

### Using Custom Patterns

```bash
cargo run --example generate_custom_patterns
```

This example shows how to create 6D files with specific patterns for each channel, including noise patterns for algorithm testing.

## File Examples

### Small Test File (good for development)
```rust
generate_small_test_file("test.meta")?;
// Creates: test.meta + test.data (~8KB total)
// Dimensions: 3×1×2×2×32×32
```

### Realistic Dataset (good for testing performance)
```rust
generate_realistic_dataset("real.meta")?;
// Creates: real.meta + real.data (~15MB total)  
// Dimensions: 10×1×5×3×256×256
```

## Error Handling

All functions return `Result<()>` and will fail if:
- File I/O errors occur
- Dimensions would exceed memory limit (1GB)
- Invalid dimensions (zero values)
- Metadata/data file mismatch

## Memory Usage

The generator limits arrays to 1GB of memory. For f32 data:
- Memory = T × P × Z × C × Y × X × 4 bytes
- Example: 10×1×5×3×256×256 = ~15MB

Use `validate_6d_file()` to check file info without loading into memory.