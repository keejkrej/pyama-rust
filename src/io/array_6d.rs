//! 6D Array structure for microscopy data
//! 
//! Implements a memory-efficient 6D array with dimensions TPZCYX:
//! - T: Time points
//! - P: Positions 
//! - Z: Z-stack depth
//! - C: Channels
//! - Y: Height
//! - X: Width

use ndarray::{Array6, ArrayView2};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

/// 6D array dimensions following TPZCYX convention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dimensions {
    pub time: usize,      // T
    pub position: usize,  // P 
    pub z: usize,         // Z
    pub channel: usize,   // C
    pub height: usize,    // Y
    pub width: usize,     // X
}

impl Dimensions {
    /// Create new dimensions
    pub fn new(time: usize, position: usize, z: usize, channel: usize, height: usize, width: usize) -> Self {
        Self { time, position, z, channel, height, width }
    }
    
    /// Create 2D dimensions (single z-plane)
    #[allow(dead_code)] // Used in tests
    pub fn new_2d(time: usize, position: usize, channel: usize, height: usize, width: usize) -> Self {
        Self::new(time, position, 1, channel, height, width)
    }
    
    /// Get total number of elements
    pub fn total_elements(&self) -> usize {
        self.time * self.position * self.z * self.channel * self.height * self.width
    }
    
    /// Get shape as array for ndarray
    pub fn shape(&self) -> [usize; 6] {
        [self.time, self.position, self.z, self.channel, self.height, self.width]
    }
    
    /// Validate dimensions are reasonable
    pub fn validate(&self) -> Result<()> {
        if self.time == 0 || self.position == 0 || self.z == 0 || 
           self.channel == 0 || self.height == 0 || self.width == 0 {
            return Err(anyhow!("All dimensions must be greater than 0"));
        }
        
        // Check for reasonable memory usage (< 1GB for f32)
        let memory_mb = (self.total_elements() * 4) / (1024 * 1024);
        if memory_mb > 1024 {
            return Err(anyhow!("Array would use {}MB memory, maximum is 1024MB", memory_mb));
        }
        
        Ok(())
    }
}

/// Statistics for a 2D frame
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameStats {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub total_pixels: usize,
    pub saturated_pixels: usize,
    pub saturation_threshold: f64,
}

impl FrameStats {
    /// Calculate statistics from a 2D array view
    pub fn from_frame(frame: ArrayView2<f32>, saturation_threshold: f64) -> Self {
        let values: Vec<f32> = frame.iter().cloned().collect();
        let total_pixels = values.len();
        
        if total_pixels == 0 {
            return Self {
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
                total_pixels: 0,
                saturated_pixels: 0,
                saturation_threshold,
            };
        }
        
        let mut sorted_values = values.clone();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let sum: f64 = values.iter().map(|&x| x as f64).sum();
        let mean = sum / total_pixels as f64;
        
        let median = if total_pixels % 2 == 0 {
            (sorted_values[total_pixels / 2 - 1] + sorted_values[total_pixels / 2]) as f64 / 2.0
        } else {
            sorted_values[total_pixels / 2] as f64
        };
        
        let variance = values.iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / total_pixels as f64;
        let std_dev = variance.sqrt();
        
        let min = sorted_values[0] as f64;
        let max = sorted_values[total_pixels - 1] as f64;
        
        let saturated_pixels = values.iter()
            .filter(|&&x| x as f64 >= saturation_threshold)
            .count();
        
        Self {
            mean,
            median,
            std_dev,
            min,
            max,
            total_pixels,
            saturated_pixels,
            saturation_threshold,
        }
    }
}

/// 6D array wrapper with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Array6D {
    /// The actual 6D array data
    data: Array6<f32>,
    /// Array dimensions
    dimensions: Dimensions,
    /// Pixel size in micrometers
    pixel_size_um: f64,
    /// Time interval between frames in seconds
    time_interval_s: f64,
    /// Channel names
    channel_names: Vec<String>,
    /// Data type description
    data_type: String,
}

impl Array6D {
    /// Create new 6D array
    pub fn new(
        data: Array6<f32>,
        dimensions: Dimensions,
        pixel_size_um: f64,
        time_interval_s: f64,
        channel_names: Vec<String>,
        data_type: String,
    ) -> Result<Self> {
        // Validate dimensions match data shape
        let expected_shape = dimensions.shape();
        let actual_shape: Vec<usize> = data.shape().to_vec();
        if actual_shape != expected_shape {
            return Err(anyhow!(
                "Data shape {:?} does not match dimensions {:?}",
                actual_shape, expected_shape
            ));
        }
        
        // Validate channel names
        if channel_names.len() != dimensions.channel {
            return Err(anyhow!(
                "Number of channel names ({}) does not match channel dimension ({})",
                channel_names.len(), dimensions.channel
            ));
        }
        
        Ok(Self {
            data,
            dimensions,
            pixel_size_um,
            time_interval_s,
            channel_names,
            data_type,
        })
    }
    
    /// Create empty array with given dimensions
    #[allow(dead_code)] // API function for creating empty arrays
    pub fn zeros(
        dimensions: Dimensions,
        pixel_size_um: f64,
        time_interval_s: f64,
        channel_names: Vec<String>,
        data_type: String,
    ) -> Result<Self> {
        dimensions.validate()?;
        
        let data = Array6::zeros(dimensions.shape());
        Self::new(data, dimensions, pixel_size_um, time_interval_s, channel_names, data_type)
    }
    
    /// Get dimensions
    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
    
    /// Get pixel size
    pub fn pixel_size_um(&self) -> f64 {
        self.pixel_size_um
    }
    
    /// Get time interval
    pub fn time_interval_s(&self) -> f64 {
        self.time_interval_s
    }
    
    /// Get channel names
    pub fn channel_names(&self) -> &[String] {
        &self.channel_names
    }
    
    /// Get data type
    pub fn data_type(&self) -> &str {
        &self.data_type
    }
    
    /// Get reference to underlying data
    pub fn data(&self) -> &Array6<f32> {
        &self.data
    }
    
    /// Get mutable reference to underlying data
    #[allow(dead_code)] // API function for data modification
    pub fn data_mut(&mut self) -> &mut Array6<f32> {
        &mut self.data
    }
    
    /// Get a 2D frame at specific coordinates
    pub fn get_frame(&self, t: usize, p: usize, z: usize, c: usize) -> Result<ArrayView2<f32>> {
        if t >= self.dimensions.time {
            return Err(anyhow!("Time index {} out of bounds (max: {})", t, self.dimensions.time - 1));
        }
        if p >= self.dimensions.position {
            return Err(anyhow!("Position index {} out of bounds (max: {})", p, self.dimensions.position - 1));
        }
        if z >= self.dimensions.z {
            return Err(anyhow!("Z index {} out of bounds (max: {})", z, self.dimensions.z - 1));
        }
        if c >= self.dimensions.channel {
            return Err(anyhow!("Channel index {} out of bounds (max: {})", c, self.dimensions.channel - 1));
        }
        
        Ok(self.data.slice(ndarray::s![t, p, z, c, .., ..]))
    }
    
    /// Set a 2D frame at specific coordinates
    #[allow(dead_code)] // Used in tests and API
    pub fn set_frame(&mut self, t: usize, p: usize, z: usize, c: usize, frame: &ArrayView2<f32>) -> Result<()> {
        if frame.shape() != [self.dimensions.height, self.dimensions.width] {
            return Err(anyhow!(
                "Frame shape {:?} does not match expected [{}x{}]",
                frame.shape(), self.dimensions.height, self.dimensions.width
            ));
        }
        
        let mut slice = self.data.slice_mut(ndarray::s![t, p, z, c, .., ..]);
        slice.assign(frame);
        Ok(())
    }
    
    /// Get statistics for a specific frame
    pub fn get_frame_stats(&self, t: usize, p: usize, z: usize, c: usize, saturation_threshold: f64) -> Result<FrameStats> {
        let frame = self.get_frame(t, p, z, c)?;
        Ok(FrameStats::from_frame(frame, saturation_threshold))
    }
    
    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        self.dimensions.total_elements() * std::mem::size_of::<f32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;
    
    #[test]
    fn test_dimensions_creation() {
        let dims = Dimensions::new(5, 1, 1, 3, 100, 100);
        assert_eq!(dims.time, 5);
        assert_eq!(dims.position, 1);
        assert_eq!(dims.z, 1);
        assert_eq!(dims.channel, 3);
        assert_eq!(dims.height, 100);
        assert_eq!(dims.width, 100);
        
        let dims_2d = Dimensions::new_2d(5, 1, 3, 100, 100);
        assert_eq!(dims_2d.z, 1);
    }
    
    #[test]
    fn test_dimensions_validation() {
        let valid_dims = Dimensions::new(2, 1, 1, 2, 10, 10);
        assert!(valid_dims.validate().is_ok());
        
        let zero_dims = Dimensions::new(0, 1, 1, 2, 10, 10);
        assert!(zero_dims.validate().is_err());
        
        // Test memory limit
        let huge_dims = Dimensions::new(1000, 1, 1, 1, 1000, 1000);
        assert!(huge_dims.validate().is_err());
    }
    
    #[test]
    fn test_array6d_creation() {
        let dims = Dimensions::new_2d(2, 1, 2, 4, 4);
        let data = Array6::zeros(dims.shape());
        let channel_names = vec!["Channel1".to_string(), "Channel2".to_string()];
        
        let array = Array6D::new(
            data,
            dims,
            0.65,
            1.0,
            channel_names,
            "uint16".to_string(),
        );
        
        assert!(array.is_ok());
        let array = array.unwrap();
        assert_eq!(array.dimensions().total_elements(), 2 * 1 * 1 * 2 * 4 * 4);
    }
    
    #[test]
    fn test_frame_access() {
        let dims = Dimensions::new_2d(2, 1, 2, 4, 4);
        let mut data = Array6::zeros(dims.shape());
        
        // Set some test data
        data[[0, 0, 0, 0, 1, 1]] = 42.0;
        
        let channel_names = vec!["Channel1".to_string(), "Channel2".to_string()];
        let array = Array6D::new(
            data,
            dims,
            0.65,
            1.0,
            channel_names,
            "uint16".to_string(),
        ).unwrap();
        
        let frame = array.get_frame(0, 0, 0, 0).unwrap();
        assert_eq!(frame[[1, 1]], 42.0);
        
        // Test bounds checking
        assert!(array.get_frame(2, 0, 0, 0).is_err());
        assert!(array.get_frame(0, 1, 0, 0).is_err());
        assert!(array.get_frame(0, 0, 1, 0).is_err());
        assert!(array.get_frame(0, 0, 0, 2).is_err());
    }
    
    #[test]
    fn test_frame_stats() {
        let dims = Dimensions::new_2d(1, 1, 1, 3, 3);
        let mut data = Array6::zeros(dims.shape());
        
        // Create a test pattern: [1, 2, 3; 4, 5, 6; 7, 8, 9]
        for i in 0..3 {
            for j in 0..3 {
                data[[0, 0, 0, 0, i, j]] = (i * 3 + j + 1) as f32;
            }
        }
        
        let channel_names = vec!["Test".to_string()];
        let array = Array6D::new(
            data,
            dims,
            0.65,
            1.0,
            channel_names,
            "uint16".to_string(),
        ).unwrap();
        
        let stats = array.get_frame_stats(0, 0, 0, 0, 8.0).unwrap();
        assert_eq!(stats.total_pixels, 9);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 9.0);
        assert_eq!(stats.mean, 5.0);
        assert_eq!(stats.median, 5.0);
        assert_eq!(stats.saturated_pixels, 2); // values 8 and 9
    }
    
    #[test]
    fn test_frame_setting() {
        let dims = Dimensions::new_2d(1, 1, 1, 3, 3);
        let data = Array6::zeros(dims.shape());
        let channel_names = vec!["Test".to_string()];
        
        let mut array = Array6D::new(
            data,
            dims,
            0.65,
            1.0,
            channel_names,
            "uint16".to_string(),
        ).unwrap();
        
        // Create test frame
        let test_frame = Array2::from_shape_vec((3, 3), vec![1.0; 9]).unwrap();
        let frame_view = test_frame.view();
        
        assert!(array.set_frame(0, 0, 0, 0, &frame_view).is_ok());
        
        let retrieved_frame = array.get_frame(0, 0, 0, 0).unwrap();
        assert_eq!(retrieved_frame[[0, 0]], 1.0);
        assert_eq!(retrieved_frame[[2, 2]], 1.0);
    }
}