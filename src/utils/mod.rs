//! Utility functions for working with 6D arrays

use crate::io::{load_array};
use anyhow::Result;
use std::path::Path;


/// Load and display basic information about a 6D file
#[allow(dead_code)] // Used in examples
pub fn load_and_inspect_6d_file<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let path = file_path.as_ref();
    println!("Loading 6D file: {:?}", path);
    
    // Load the array
    let array = load_array(path)?;
    
    // Display basic information
    let dims = array.dimensions();
    println!("Dimensions (T×P×Z×C×Y×X): {}×{}×{}×{}×{}×{}", 
             dims.time, dims.position, dims.z, dims.channel, dims.height, dims.width);
    println!("Total elements: {}", dims.total_elements());
    println!("Memory usage: {} MB", array.memory_usage() / (1024 * 1024));
    println!("Pixel size: {:.3} μm", array.pixel_size_um());
    println!("Time interval: {:.1} s", array.time_interval_s());
    println!("Data type: {}", array.data_type());
    
    // Display channel information
    println!("\nChannels:");
    for (i, name) in array.channel_names().iter().enumerate() {
        println!("  {}: {}", i, name);
    }
    
    // Display some frame statistics
    println!("\nFrame statistics (T=0, P=0, Z=0):");
    for c in 0..dims.channel {
        let stats = array.get_frame_stats(0, 0, 0, c, 1000.0)?;
        println!("  Channel {}: min={:.1}, max={:.1}, mean={:.1}, std={:.1}", 
                 c, stats.min, stats.max, stats.mean, stats.std_dev);
    }
    
    Ok(())
}

/// Quick validation of a 6D file without loading all data  
#[allow(dead_code)] // Used in examples
pub fn validate_6d_file<P: AsRef<Path>>(file_path: P) -> Result<()> {
    use crate::io::validate_file;
    
    let path = file_path.as_ref();
    println!("Validating 6D file: {:?}", path);
    
    let metadata = validate_file(path)?;
    
    println!("✓ File validation successful");
    println!("Dimensions: {}×{}×{}×{}×{}×{}", 
             metadata.dimensions.time, metadata.dimensions.position, 
             metadata.dimensions.z, metadata.dimensions.channel,
             metadata.dimensions.height, metadata.dimensions.width);
    println!("Format version: {}", metadata.format_version);
    println!("Data type: {}", metadata.data_type);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
}