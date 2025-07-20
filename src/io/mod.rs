//! IO module for handling 6D microscopy arrays
//! 
//! This module provides functionality for:
//! - Generating 6D arrays with specific patterns
//! - Loading and saving 6D arrays to/from disk
//! - Converting between different formats
//! - Validating array structures

pub mod array_6d;
pub mod generators;
pub mod formats;

#[allow(unused_imports)] // Re-exported for external API
pub use array_6d::Array6D;
pub use generators::{ArrayGenerator, PatternType};
pub use formats::{save_array, load_array, validate_file};