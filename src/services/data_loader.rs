//! Data loading service for 6D microscopy arrays

use crate::io::{load_array, validate_file};
use crate::io::array_6d::{Array6D, Dimensions, FrameStats};
use anyhow::Result;
use std::path::Path;

/// Metadata structure for microscopy data
#[derive(Debug, Clone, PartialEq)]
pub struct MicroscopyMetadata {
    pub file_path: String,
    pub dimensions: Dimensions,
    pub pixel_size_um: f64,
    pub time_interval_s: f64,
    pub channel_names: Vec<String>,
    pub data_type: String,
    pub memory_usage_mb: usize,
}

impl From<&Array6D> for MicroscopyMetadata {
    fn from(array: &Array6D) -> Self {
        Self {
            file_path: "".to_string(), // Will be set by the loader
            dimensions: *array.dimensions(),
            pixel_size_um: array.pixel_size_um(),
            time_interval_s: array.time_interval_s(),
            channel_names: array.channel_names().to_vec(),
            data_type: array.data_type().to_string(),
            memory_usage_mb: array.memory_usage() / (1024 * 1024),
        }
    }
}

/// Load array file metadata only (lightweight operation)
pub async fn load_array_file<P: AsRef<Path>>(file_path: P) -> Result<MicroscopyMetadata> {
    let path = file_path.as_ref();
    let path_str = path.to_string_lossy().to_string();
    
    // First validate the file without loading all data
    let metadata = validate_file(path)?;
    
    Ok(MicroscopyMetadata {
        file_path: path_str,
        dimensions: metadata.dimensions,
        pixel_size_um: metadata.pixel_size_um,
        time_interval_s: metadata.time_interval_s,
        channel_names: metadata.channel_names,
        data_type: metadata.data_type,
        memory_usage_mb: (metadata.dimensions.total_elements() * 4) / (1024 * 1024),
    })
}

/// Load the full array data (heavy operation)
#[allow(dead_code)] // Future use for heavy data processing
pub async fn load_full_array<P: AsRef<Path>>(file_path: P) -> Result<Array6D> {
    let path = file_path.as_ref();
    
    // Load the complete array
    let array = load_array(path)?;
    
    Ok(array)
}

/// Get frame statistics for a specific frame without loading the full array
#[allow(dead_code)] // Future use for detailed frame analysis
pub async fn get_frame_statistics<P: AsRef<Path>>(
    file_path: P,
    t: usize,
    p: usize,
    z: usize,
    c: usize,
    saturation_threshold: f64,
) -> Result<FrameStats> {
    let path = file_path.as_ref();
    
    // For now, load the full array to get stats
    // In the future, this could be optimized to load only the specific frame
    let array = load_array(path)?;
    let stats = array.get_frame_stats(t, p, z, c, saturation_threshold)?;
    
    Ok(stats)
}

/// Check if a file exists and is a valid 6D file
#[allow(dead_code)] // Utility function for file validation
pub fn is_valid_6d_file<P: AsRef<Path>>(file_path: P) -> bool {
    let path = file_path.as_ref();
    
    // Check if .meta file exists
    if !path.exists() {
        return false;
    }
    
    // Check if corresponding .data file exists
    let data_path = path.with_extension("data");
    if !data_path.exists() {
        return false;
    }
    
    // Try to validate the file format
    validate_file(path).is_ok()
}

/// Get file size information
#[allow(dead_code)] // Utility function for file analysis
pub fn get_file_info<P: AsRef<Path>>(file_path: P) -> Result<(u64, u64)> {
    let path = file_path.as_ref();
    
    let meta_size = std::fs::metadata(path)?.len();
    let data_path = path.with_extension("data");
    let data_size = std::fs::metadata(data_path)?.len();
    
    Ok((meta_size, data_size))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::generate_small_test_file;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_load_array_file_metadata() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.meta");
        
        // Generate test file
        generate_small_test_file(&file_path).unwrap();
        
        // Load metadata
        let metadata = load_array_file(&file_path).await.unwrap();
        
        assert_eq!(metadata.dimensions.time, 3);
        assert_eq!(metadata.dimensions.channel, 2);
        assert_eq!(metadata.channel_names.len(), 2);
        assert!(metadata.memory_usage_mb > 0);
    }
    
    #[tokio::test]
    async fn test_load_full_array() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.meta");
        
        // Generate test file
        generate_small_test_file(&file_path).unwrap();
        
        // Load full array
        let array = load_full_array(&file_path).await.unwrap();
        
        assert_eq!(array.dimensions().time, 3);
        assert_eq!(array.dimensions().channel, 2);
        assert_eq!(array.channel_names().len(), 2);
    }
    
    #[test]
    fn test_is_valid_6d_file() {
        // Test with non-existent file
        assert!(!is_valid_6d_file("nonexistent.meta"));
        
        // Could add more tests with actual files
    }
}