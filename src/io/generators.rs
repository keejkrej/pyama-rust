//! Array generators for creating synthetic 6D microscopy data
//! 
//! Provides various pattern generators for testing and demonstration purposes.

use super::array_6d::{Array6D, Dimensions};
use ndarray::Array6;
use anyhow::{Result, anyhow};

/// Types of patterns that can be generated
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    /// Uniform value across all pixels
    Uniform(f32),
    /// Linear gradient from top-left to bottom-right
    Gradient,
    /// Concentric circles pattern
    Circles,
    /// Random noise
    Noise { min: f32, max: f32 },
    /// Gaussian spots at random locations
    GaussianSpots { num_spots: usize, intensity: f32 },
    /// Sine wave pattern
    SineWave { frequency: f32, amplitude: f32 },
    /// Moving spots that change position over time
    MovingSpots { num_spots: usize, speed: f32 },
}

/// Configuration for array generation
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub dimensions: Dimensions,
    pub pixel_size_um: f64,
    pub time_interval_s: f64,
    pub channel_patterns: Vec<(String, PatternType)>,
    pub data_type: String,
    pub base_intensity: f32,
    pub noise_level: f32,
}

impl GeneratorConfig {
    /// Create new generator config
    pub fn new(dimensions: Dimensions) -> Self {
        // Create default patterns to match the number of channels
        let mut channel_patterns = Vec::new();
        for i in 0..dimensions.channel {
            let name = format!("Channel{}", i + 1);
            let pattern = if i == 0 {
                PatternType::Gradient
            } else {
                PatternType::GaussianSpots { num_spots: 3, intensity: 500.0 }
            };
            channel_patterns.push((name, pattern));
        }
        
        Self {
            dimensions,
            pixel_size_um: 0.65,
            time_interval_s: 1.0,
            channel_patterns,
            data_type: "uint16".to_string(),
            base_intensity: 100.0,
            noise_level: 10.0,
        }
    }
    
    /// Set a channel with specific pattern (replaces existing channels)
    pub fn with_channels(mut self, channels: Vec<(String, PatternType)>) -> Self {
        if channels.len() != self.dimensions.channel {
            panic!("Number of channels ({}) must match dimension ({})", channels.len(), self.dimensions.channel);
        }
        self.channel_patterns = channels;
        self
    }
    
    /// Set pixel size
    pub fn pixel_size(mut self, size_um: f64) -> Self {
        self.pixel_size_um = size_um;
        self
    }
    
    /// Set time interval
    pub fn time_interval(mut self, interval_s: f64) -> Self {
        self.time_interval_s = interval_s;
        self
    }
    
    /// Set base intensity level
    pub fn base_intensity(mut self, intensity: f32) -> Self {
        self.base_intensity = intensity;
        self
    }
    
    /// Set noise level
    pub fn noise_level(mut self, noise: f32) -> Self {
        self.noise_level = noise;
        self
    }
}

/// Main array generator
pub struct ArrayGenerator;

impl ArrayGenerator {
    /// Generate a 6D array based on the provided configuration
    pub fn generate(config: GeneratorConfig) -> Result<Array6D> {
        let dims = config.dimensions;
        dims.validate()?;
        
        // Validate channel count matches patterns
        if config.channel_patterns.len() != dims.channel {
            return Err(anyhow!(
                "Number of channel patterns ({}) must match channel dimension ({})",
                config.channel_patterns.len(), dims.channel
            ));
        }
        
        // Create data array
        let mut data = Array6::zeros(dims.shape());
        
        // Generate data for each channel
        for (c, (_, pattern)) in config.channel_patterns.iter().enumerate() {
            Self::generate_channel(
                &mut data, 
                &dims, 
                c, 
                *pattern, 
                config.base_intensity, 
                config.noise_level
            )?;
        }
        
        // Extract channel names
        let channel_names: Vec<String> = config.channel_patterns
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        
        Array6D::new(
            data,
            dims,
            config.pixel_size_um,
            config.time_interval_s,
            channel_names,
            config.data_type,
        )
    }
    
    /// Generate data for a specific channel
    fn generate_channel(
        data: &mut Array6<f32>,
        dims: &Dimensions,
        channel: usize,
        pattern: PatternType,
        base_intensity: f32,
        noise_level: f32,
    ) -> Result<()> {
        for t in 0..dims.time {
            for p in 0..dims.position {
                for z in 0..dims.z {
                    for y in 0..dims.height {
                        for x in 0..dims.width {
                            let value = Self::generate_pixel_value(
                                pattern,
                                t, p, z, x, y,
                                dims,
                                base_intensity,
                            );
                            
                            // Add noise
                            let noise = (fastrand::f32() - 0.5) * 2.0 * noise_level;
                            let final_value = (value + noise).max(0.0);
                            
                            data[[t, p, z, channel, y, x]] = final_value;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Generate pixel value based on pattern type
    fn generate_pixel_value(
        pattern: PatternType,
        t: usize, _p: usize, _z: usize, x: usize, y: usize,
        dims: &Dimensions,
        base_intensity: f32,
    ) -> f32 {
        let center_x = dims.width as f32 / 2.0;
        let center_y = dims.height as f32 / 2.0;
        let max_distance = (center_x.powi(2) + center_y.powi(2)).sqrt();
        
        match pattern {
            PatternType::Uniform(value) => value,
            
            PatternType::Gradient => {
                let dx = x as f32 / dims.width as f32;
                let dy = y as f32 / dims.height as f32;
                base_intensity * (dx + dy) / 2.0
            },
            
            PatternType::Circles => {
                let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                let normalized_distance = distance / max_distance;
                base_intensity * (1.0 - normalized_distance).max(0.0)
            },
            
            PatternType::Noise { min, max } => {
                min + fastrand::f32() * (max - min)
            },
            
            PatternType::GaussianSpots { num_spots, intensity } => {
                let mut value = base_intensity * 0.1; // Low background
                
                // Generate deterministic spots based on position
                for spot_id in 0..num_spots {
                    let seed = (spot_id * 12345 + t * 67890) as u64;
                    fastrand::seed(seed);
                    
                    let spot_x = fastrand::f32() * dims.width as f32;
                    let spot_y = fastrand::f32() * dims.height as f32;
                    let sigma = 10.0 + fastrand::f32() * 20.0; // Spot size
                    
                    let dx = x as f32 - spot_x;
                    let dy = y as f32 - spot_y;
                    let distance_sq = dx * dx + dy * dy;
                    
                    let gaussian = (-distance_sq / (2.0 * sigma * sigma)).exp();
                    value += intensity * gaussian;
                }
                
                value
            },
            
            PatternType::SineWave { frequency, amplitude } => {
                let phase = 2.0 * std::f32::consts::PI * frequency;
                let wave_x = (x as f32 * phase / dims.width as f32).sin();
                let wave_y = (y as f32 * phase / dims.height as f32).sin();
                base_intensity + amplitude * wave_x * wave_y
            },
            
            PatternType::MovingSpots { num_spots, speed } => {
                let mut value = base_intensity * 0.2;
                
                for spot_id in 0..num_spots {
                    // Circular motion
                    let angle = t as f32 * speed + spot_id as f32 * 2.0 * std::f32::consts::PI / num_spots as f32;
                    let radius = 50.0;
                    
                    let spot_x = center_x + radius * angle.cos();
                    let spot_y = center_y + radius * angle.sin();
                    
                    let dx = x as f32 - spot_x;
                    let dy = y as f32 - spot_y;
                    let distance_sq = dx * dx + dy * dy;
                    
                    let sigma = 15.0;
                    let gaussian = (-distance_sq / (2.0 * sigma * sigma)).exp();
                    value += 500.0 * gaussian;
                }
                
                value
            },
        }
    }
    
    /// Generate a simple test array for unit testing
    #[allow(dead_code)] // Used in tests
    pub fn generate_test_array(t: usize, p: usize, z: usize, c: usize, h: usize, w: usize) -> Result<Array6D> {
        let dims = Dimensions::new(t, p, z, c, h, w);
        let config = GeneratorConfig::new(dims)
            .base_intensity(100.0)
            .noise_level(0.0); // No noise for predictable tests
        
        Self::generate(config)
    }
    
    /// Generate a minimal array for performance testing
    #[allow(dead_code)] // Used in tests
    pub fn generate_minimal() -> Result<Array6D> {
        let dims = Dimensions::new_2d(2, 1, 2, 8, 8);
        let config = GeneratorConfig::new(dims)
            .with_channels(vec![
                ("Test1".to_string(), PatternType::Uniform(50.0)),
                ("Test2".to_string(), PatternType::Uniform(150.0)),
            ])
            .base_intensity(100.0)
            .noise_level(5.0);
        
        Self::generate(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generator_config_creation() {
        let dims = Dimensions::new_2d(5, 1, 3, 64, 64);
        let config = GeneratorConfig::new(dims)
            .pixel_size(0.5)
            .time_interval(2.0)
            .base_intensity(200.0)
            .noise_level(15.0);
        
        assert_eq!(config.pixel_size_um, 0.5);
        assert_eq!(config.time_interval_s, 2.0);
        assert_eq!(config.base_intensity, 200.0);
        assert_eq!(config.noise_level, 15.0);
    }
    
    #[test]
    fn test_generate_minimal_array() {
        let array = ArrayGenerator::generate_minimal().unwrap();
        
        assert_eq!(array.dimensions().time, 2);
        assert_eq!(array.dimensions().channel, 2);
        assert_eq!(array.dimensions().height, 8);
        assert_eq!(array.dimensions().width, 8);
        
        // Check that different channels have different values
        let frame1 = array.get_frame(0, 0, 0, 0).unwrap();
        let frame2 = array.get_frame(0, 0, 0, 1).unwrap();
        assert_ne!(frame1[[0, 0]], frame2[[0, 0]]);
    }
    
    #[test]
    fn test_generate_test_array() {
        let array = ArrayGenerator::generate_test_array(3, 1, 1, 2, 10, 10).unwrap();
        
        assert_eq!(array.dimensions().time, 3);
        assert_eq!(array.dimensions().position, 1);
        assert_eq!(array.dimensions().z, 1);
        assert_eq!(array.dimensions().channel, 2);
        assert_eq!(array.dimensions().height, 10);
        assert_eq!(array.dimensions().width, 10);
    }
    
    #[test]
    fn test_uniform_pattern() {
        let dims = Dimensions::new_2d(1, 1, 1, 4, 4);
        let config = GeneratorConfig::new(dims)
            .with_channels(vec![("Uniform".to_string(), PatternType::Uniform(42.0))])
            .noise_level(0.0);
        
        let array = ArrayGenerator::generate(config).unwrap();
        let frame = array.get_frame(0, 0, 0, 0).unwrap();
        
        // All pixels should be the same value (42.0)
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(frame[[i, j]], 42.0);
            }
        }
    }
    
    #[test]
    fn test_gradient_pattern() {
        let dims = Dimensions::new_2d(1, 1, 1, 4, 4);
        let config = GeneratorConfig::new(dims)
            .with_channels(vec![("Gradient".to_string(), PatternType::Gradient)])
            .base_intensity(100.0)
            .noise_level(0.0);
        
        let array = ArrayGenerator::generate(config).unwrap();
        let frame = array.get_frame(0, 0, 0, 0).unwrap();
        
        // Top-left should be darker than bottom-right
        assert!(frame[[0, 0]] < frame[[3, 3]]);
        // Values should be monotonically increasing
        assert!(frame[[0, 0]] <= frame[[0, 3]]);
        assert!(frame[[0, 0]] <= frame[[3, 0]]);
    }
    
    #[test]
    fn test_noise_pattern() {
        let dims = Dimensions::new_2d(1, 1, 1, 10, 10);
        let config = GeneratorConfig::new(dims)
            .with_channels(vec![("Noise".to_string(), PatternType::Noise { min: 50.0, max: 150.0 })])
            .noise_level(0.0);
        
        let array = ArrayGenerator::generate(config).unwrap();
        let frame = array.get_frame(0, 0, 0, 0).unwrap();
        
        // All values should be in range [50, 150]
        for i in 0..10 {
            for j in 0..10 {
                let value = frame[[i, j]];
                assert!(value >= 50.0 && value <= 150.0);
            }
        }
        
        // Should have some variation (not all the same)
        let first_value = frame[[0, 0]];
        let has_variation = (0..10).any(|i| {
            (0..10).any(|j| (frame[[i, j]] - first_value).abs() > 1.0)
        });
        assert!(has_variation);
    }
    
    #[test]
    #[should_panic]
    fn test_channel_mismatch_error() {
        let dims = Dimensions::new_2d(1, 1, 3, 4, 4); // 3 channels
        let _config = GeneratorConfig::new(dims)
            .with_channels(vec![
                ("Only1".to_string(), PatternType::Uniform(42.0)),
                ("Only2".to_string(), PatternType::Uniform(84.0)),
                // Missing third channel - should panic
            ]);
    }
}