//! Example showing how to generate 6D files with different pattern types

use clap::{Parser, ValueEnum};
use pyama_rust::io::{PatternType, ArrayGenerator, save_array};
use pyama_rust::io::generators::GeneratorConfig;
use pyama_rust::io::array_6d::Dimensions;

#[derive(Parser)]
#[command(name = "generate_6d")]
#[command(about = "Generate 6D files with different pattern types")]
struct Args {
    /// Type of pattern to generate
    #[arg(long)]
    r#type: PatternTypeArg,
    
    /// Output file path
    #[arg(short, long, default_value = "test.meta")]
    output: String,
    
    /// Time points
    #[arg(long, default_value = "3")]
    time: usize,
    
    /// Positions
    #[arg(long, default_value = "1")]
    positions: usize,
    
    /// Z slices
    #[arg(long, default_value = "2")]
    z_slices: usize,
    
    /// Number of channels
    #[arg(long, default_value = "2")]
    channels: usize,
    
    /// Height
    #[arg(long, default_value = "64")]
    height: usize,
    
    /// Width
    #[arg(long, default_value = "64")]
    width: usize,
}

#[derive(Debug, Clone, ValueEnum)]
enum PatternTypeArg {
    Noise,
    Gaussian,
    Gradient,
    Circles,
    Uniform,
    SineWave,
    MovingSpots,
}

impl From<PatternTypeArg> for PatternType {
    fn from(arg: PatternTypeArg) -> Self {
        match arg {
            PatternTypeArg::Noise => PatternType::Noise { min: 50.0, max: 200.0 },
            PatternTypeArg::Gaussian => PatternType::GaussianSpots { num_spots: 3, intensity: 800.0 },
            PatternTypeArg::Gradient => PatternType::Gradient,
            PatternTypeArg::Circles => PatternType::Circles,
            PatternTypeArg::Uniform => PatternType::Uniform(150.0),
            PatternTypeArg::SineWave => PatternType::SineWave { frequency: 0.1, amplitude: 200.0 },
            PatternTypeArg::MovingSpots => PatternType::MovingSpots { num_spots: 2, speed: 0.1 },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("=== Generating 6D File with {:?} Pattern ===\n", args.r#type);
    
    // Create dimensions
    let dims = Dimensions::new(args.time, args.positions, args.z_slices, args.channels, args.height, args.width);
    
    // Create channel patterns - all channels will use the same pattern type
    let pattern: PatternType = args.r#type.into();
    let mut channel_patterns = Vec::new();
    for i in 0..args.channels {
        channel_patterns.push((format!("Channel_{}", i + 1), pattern));
    }
    
    // Create generator config
    let config = GeneratorConfig::new(dims)
        .with_channels(channel_patterns)
        .pixel_size(0.65)
        .time_interval(2.0)
        .base_intensity(200.0)
        .noise_level(15.0);
    
    // Generate the array
    println!("Generating 6D array with dimensions: {}x{}x{}x{}x{}x{}", 
             args.time, args.positions, args.z_slices, args.channels, args.height, args.width);
    let array = ArrayGenerator::generate(config)?;
    
    // Save to file
    println!("Saving to: {}", args.output);
    save_array(&array, &args.output)?;
    
    println!("✓ Generated: {} and {}", args.output, args.output.replace(".meta", ".data"));
    println!("\nTo load this file, run: cargo run --example load_6d");
    
    Ok(())
}