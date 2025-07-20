//! UI module containing all user interface components and views

pub mod components;
pub mod views;

// Re-export commonly used UI items for external use
#[allow(unused_imports)]
pub use components::*;
#[allow(unused_imports)]
pub use views::*;