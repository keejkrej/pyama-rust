//! File format support for 6D arrays
//! 
//! Supports loading and saving 6D arrays in Split format (.meta + .data files).

use super::array_6d::Array6D;
use anyhow::{Result, anyhow};
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

/// Metadata that gets saved separately from array data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayMetadata {
    pub dimensions: super::array_6d::Dimensions,
    pub pixel_size_um: f64,
    pub time_interval_s: f64,
    pub channel_names: Vec<String>,
    pub data_type: String,
    pub format_version: String,
    pub created_at: String,
}

impl From<&Array6D> for ArrayMetadata {
    fn from(array: &Array6D) -> Self {
        Self {
            dimensions: *array.dimensions(),
            pixel_size_um: array.pixel_size_um(),
            time_interval_s: array.time_interval_s(),
            channel_names: array.channel_names().to_vec(),
            data_type: array.data_type().to_string(),
            format_version: "1.0".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(), // Simplified timestamp
        }
    }
}

/// Save a 6D array to split format (.meta + .data files)
pub fn save_array<P: AsRef<Path>>(array: &Array6D, path: P) -> Result<()> {
    save_split(array, path.as_ref())
}

/// Load a 6D array from split format (.meta + .data files)
pub fn load_array<P: AsRef<Path>>(path: P) -> Result<Array6D> {
    load_split(path.as_ref())
}


/// Save in split format (metadata + data files)
fn save_split(array: &Array6D, path: &Path) -> Result<()> {
    // Save metadata
    let metadata = ArrayMetadata::from(array);
    let meta_path = path;
    let mut meta_file = File::create(meta_path)?;
    let meta_json = serde_json::to_string_pretty(&metadata)?;
    meta_file.write_all(meta_json.as_bytes())?;
    
    // Save data
    let data_path = path.with_extension("data");
    let mut data_file = File::create(data_path)?;
    
    // Save raw array data as f32 binary
    let data_slice = array.data().as_slice().ok_or_else(|| {
        anyhow!("Array data is not contiguous in memory")
    })?;
    
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            data_slice.as_ptr() as *const u8,
            data_slice.len() * std::mem::size_of::<f32>(),
        )
    };
    
    data_file.write_all(bytes)?;
    Ok(())
}

/// Load from split format (metadata + data files)
fn load_split(path: &Path) -> Result<Array6D> {
    // Load metadata
    let mut meta_file = File::open(path)?;
    let mut meta_content = String::new();
    meta_file.read_to_string(&mut meta_content)?;
    let metadata: ArrayMetadata = serde_json::from_str(&meta_content)?;
    
    // Load data
    let data_path = path.with_extension("data");
    let mut data_file = File::open(data_path)?;
    let mut buffer = Vec::new();
    data_file.read_to_end(&mut buffer)?;
    
    // Convert bytes back to f32 array
    let expected_elements = metadata.dimensions.total_elements();
    let expected_bytes = expected_elements * std::mem::size_of::<f32>();
    
    if buffer.len() != expected_bytes {
        return Err(anyhow!(
            "Data file size mismatch: expected {} bytes, got {}",
            expected_bytes, buffer.len()
        ));
    }
    
    let data_slice: &[f32] = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr() as *const f32,
            expected_elements,
        )
    };
    
    // Reconstruct ndarray
    let data = ndarray::Array6::from_shape_vec(metadata.dimensions.shape(), data_slice.to_vec())?;
    
    Array6D::new(
        data,
        metadata.dimensions,
        metadata.pixel_size_um,
        metadata.time_interval_s,
        metadata.channel_names,
        metadata.data_type,
    )
}

/// Get file size estimate for split format
#[allow(dead_code)] // Utility function for size estimation
pub fn estimate_file_size(array: &Array6D) -> usize {
    let data_size = array.memory_usage();
    let metadata_size = 1024; // Rough estimate for JSON metadata
    
    // Raw data + JSON metadata
    data_size + metadata_size * 2
}

/// Validate that a split format file can be loaded
pub fn validate_file<P: AsRef<Path>>(path: P) -> Result<ArrayMetadata> {
    let path = path.as_ref();
    
    // Load and validate metadata
    let mut meta_file = File::open(path)?;
    let mut meta_content = String::new();
    meta_file.read_to_string(&mut meta_content)?;
    let metadata: ArrayMetadata = serde_json::from_str(&meta_content)?;
    
    // Check if data file exists
    let data_path = path.with_extension("data");
    if !data_path.exists() {
        return Err(anyhow!("Data file not found: {:?}", data_path));
    }
    
    // Validate data file size
    let data_file_size = std::fs::metadata(data_path)?.len() as usize;
    let expected_size = metadata.dimensions.total_elements() * std::mem::size_of::<f32>();
    if data_file_size != expected_size {
        return Err(anyhow!(
            "Data file size mismatch: expected {} bytes, got {}",
            expected_size, data_file_size
        ));
    }
    
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::generators::ArrayGenerator;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_split_save_load() {
        let array = ArrayGenerator::generate_minimal().unwrap();
        let temp_file = NamedTempFile::new().unwrap();
        let meta_path = temp_file.path().with_extension("meta");
        let data_path = temp_file.path().with_extension("data");
        
        // Save
        save_array(&array, &meta_path).unwrap();
        
        // Verify both files exist
        assert!(meta_path.exists());
        assert!(data_path.exists());
        
        // Load
        let loaded_array = load_array(&meta_path).unwrap();
        
        // Compare metadata
        assert_eq!(array.dimensions(), loaded_array.dimensions());
        assert_eq!(array.channel_names(), loaded_array.channel_names());
        assert_eq!(array.pixel_size_um(), loaded_array.pixel_size_um());
        
        // Compare data
        let orig_frame = array.get_frame(0, 0, 0, 0).unwrap();
        let loaded_frame = loaded_array.get_frame(0, 0, 0, 0).unwrap();
        assert_eq!(orig_frame[[0, 0]], loaded_frame[[0, 0]]);
        assert_eq!(orig_frame[[7, 7]], loaded_frame[[7, 7]]);
        
        // Clean up
        std::fs::remove_file(&meta_path).ok();
        std::fs::remove_file(&data_path).ok();
    }
    
    #[test]
    fn test_file_validation() {
        let array = ArrayGenerator::generate_minimal().unwrap();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().with_extension("meta");
        
        // Save array
        save_array(&array, &path).unwrap();
        
        // Validate
        let metadata = validate_file(&path).unwrap();
        assert_eq!(metadata.dimensions, *array.dimensions());
        assert_eq!(metadata.channel_names, array.channel_names());
        
        // Clean up
        std::fs::remove_file(&path.with_extension("meta")).ok();
        std::fs::remove_file(&path.with_extension("data")).ok();
    }
    
    #[test]
    fn test_file_size_estimation() {
        let array = ArrayGenerator::generate_minimal().unwrap();
        let file_size = estimate_file_size(&array);
        
        // Should be reasonable size (data + metadata overhead)
        let expected_data_size = array.memory_usage();
        assert!(file_size > expected_data_size); // Should include metadata overhead
        assert!(file_size > 1024); // Should be at least 1KB due to metadata
    }
    
    #[test]
    fn test_metadata_content() {
        let array = ArrayGenerator::generate_minimal().unwrap();
        let metadata = ArrayMetadata::from(&array);
        
        // Check metadata fields
        assert_eq!(metadata.dimensions.time, 2);
        assert_eq!(metadata.dimensions.channel, 2);
        assert_eq!(metadata.format_version, "1.0");
        assert!(!metadata.created_at.is_empty());
        assert_eq!(metadata.channel_names.len(), 2);
    }
}

// Removed chrono dependency - using simplified timestamps