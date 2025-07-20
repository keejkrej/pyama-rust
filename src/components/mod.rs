//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to define common UI elements like buttons, forms, and modals. 
//! 
//! This module contains all the reusable components for the PyAMA application including:
//! - Pattern cells for the micropattern grid
//! - Detail panels for showing pattern information
//! - Viewer and traces panes for different application views
//! - Navigation components like sidebar and top bar

// Export types first
pub mod types;
pub use types::*;

// UI Components
mod pattern_cell;
pub use pattern_cell::PatternCell;

mod detail_panel;
pub use detail_panel::DetailPanel;

mod viewer_pane;
pub use viewer_pane::ViewerPane;

mod traces_pane;
pub use traces_pane::TracesPane;

mod top_bar;
pub use top_bar::TopBar;

mod sidebar;
pub use sidebar::Sidebar;
