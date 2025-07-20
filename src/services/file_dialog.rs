//! File dialog service for selecting 6D data files

use anyhow::Result;
use std::path::PathBuf;

/// Open a file dialog to select a 6D data file (.meta)
pub async fn select_6d_file() -> Result<Option<PathBuf>> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("6D Data Files", &["meta"])
        .add_filter("All Files", &["*"])
        .set_title("Select 6D Data File")
        .set_directory(".")
        .pick_file()
        .await;
    
    match file {
        Some(handle) => Ok(Some(handle.path().to_path_buf())),
        None => Ok(None), // User cancelled
    }
}

/// Open a file dialog to select any file
#[allow(dead_code)] // Alternative file selection method
pub async fn select_any_file() -> Result<Option<PathBuf>> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("All Files", &["*"])
        .set_title("Select File")
        .set_directory(".")
        .pick_file()
        .await;
    
    match file {
        Some(handle) => Ok(Some(handle.path().to_path_buf())),
        None => Ok(None), // User cancelled
    }
}

/// Open a file dialog to save a 6D data file
#[allow(dead_code)] // Future use for saving functionality
pub async fn save_6d_file() -> Result<Option<PathBuf>> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("6D Data Files", &["meta"])
        .set_title("Save 6D Data File")
        .set_directory(".")
        .set_file_name("data.meta")
        .save_file()
        .await;
    
    match file {
        Some(handle) => Ok(Some(handle.path().to_path_buf())),
        None => Ok(None), // User cancelled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Note: These tests can't run in CI since they require GUI interaction
    #[tokio::test]
    #[ignore]
    async fn test_select_6d_file() {
        let result = select_6d_file().await;
        assert!(result.is_ok());
        // Result depends on user interaction
    }
}