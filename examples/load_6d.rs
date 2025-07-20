//! Example showing how to load and inspect 6D files

use clap::Parser;
use pyama_rust::utils::{load_and_inspect_6d_file, validate_6d_file};

#[derive(Parser)]
#[command(name = "load_6d")]
#[command(about = "Load and inspect 6D files")]
struct Args {
    /// Input file path
    #[arg(short, long, default_value = "test.meta")]
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("=== Loading 6D File: {} ===\n", args.input);
    
    // Check if file exists
    if !std::path::Path::new(&args.input).exists() {
        println!("File {} not found.", args.input);
        println!("Generate it first by running: cargo run --example generate_6d --type <TYPE>");
        return Ok(());
    }
    
    // Quick validation
    println!("1. Validating file...");
    validate_6d_file(&args.input)?;
    println!();
    
    // Full load and inspection
    println!("2. Loading and inspecting file...");
    load_and_inspect_6d_file(&args.input)?;
    
    Ok(())
}