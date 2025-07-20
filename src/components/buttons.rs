//! Button Components
//! 
//! This module provides reusable button components for the PyAMA application.
//! 
//! ## Available Button Components
//! 
//! - `PrimaryButton`: For main actions (blue/indigo styling)
//! - `ToggleButton`: For toggle states like channel selection (active/inactive states)
//! - `NavigationButton`: For navigation controls like time navigation
//! - `ExportButton`: For export actions (green styling with shadow)
//! - `ConfirmButton`: For positive confirmation actions (green styling)
//! - `RejectButton`: For negative/rejection actions (red styling)
//! - `Button`: Generic button with customizable variant and size
//! 
//! ## Usage Examples
//! 
//! ```rust
//! // Primary action button
//! PrimaryButton {
//!     onclick: move |_| { /* handle click */ },
//!     "Save Changes"
//! }
//! 
//! // Toggle button for channel selection
//! ToggleButton {
//!     active: selected_channel == Channel::PhaseContrast,
//!     onclick: move |_| on_channel_change.call(Channel::PhaseContrast),
//!     "Phase Contrast"
//! }
//! 
//! // Navigation button
//! NavigationButton {
//!     onclick: move |_| navigate_previous(),
//!     disabled: current_page == 1,
//!     "<"
//! }
//! ```

use dioxus::prelude::*;

/// Button variant enum to define different button styles
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Navigation,
}

/// Button size enum
#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// Primary button component for main actions
#[component]
pub fn PrimaryButton(
    onclick: EventHandler<MouseEvent>,
    disabled: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let disabled = disabled.unwrap_or(false);
    let additional_class = class.unwrap_or_default();
    
    rsx! {
        button {
            class: "px-4 py-2 text-sm bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white border border-indigo-600 rounded-lg transition-colors duration-200 {additional_class}",
            disabled,
            onclick: move |evt| {
                if !disabled {
                    onclick.call(evt);
                }
            },
            {children}
        }
    }
}

/// Toggle button component for channel selection and similar toggle actions
#[component]
pub fn ToggleButton(
    onclick: EventHandler<MouseEvent>,
    active: bool,
    disabled: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let disabled = disabled.unwrap_or(false);
    let additional_class = class.unwrap_or_default();
    
    let button_class = if active {
        "bg-indigo-600 text-white border-indigo-600"
    } else {
        "bg-white text-gray-700 border-gray-300 hover:bg-gray-50"
    };
    
    rsx! {
        button {
            class: "px-3 py-2 text-sm font-medium border rounded-lg transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed {button_class} {additional_class}",
            disabled,
            onclick: move |evt| {
                if !disabled {
                    onclick.call(evt);
                }
            },
            {children}
        }
    }
}

/// Navigation button component for time navigation and similar controls
#[component]
pub fn NavigationButton(
    onclick: EventHandler<MouseEvent>,
    disabled: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let disabled = disabled.unwrap_or(false);
    let additional_class = class.unwrap_or_default();
    
    rsx! {
        button {
            class: "px-3 py-1 text-base font-semibold border border-gray-300 bg-white rounded-lg transition-all duration-200 hover:bg-gray-50 active:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed {additional_class}",
            disabled,
            onclick: move |evt| {
                if !disabled {
                    onclick.call(evt);
                }
            },
            {children}
        }
    }
}

/// Generic button component with full customization
#[component]
pub fn Button(
    onclick: EventHandler<MouseEvent>,
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    disabled: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or(ButtonVariant::Secondary);
    let size = size.unwrap_or(ButtonSize::Medium);
    let disabled = disabled.unwrap_or(false);
    let additional_class = class.unwrap_or_default();
    
    let variant_class = match variant {
        ButtonVariant::Primary => "bg-indigo-600 hover:bg-indigo-700 text-white border-indigo-600",
        ButtonVariant::Secondary => "bg-white hover:bg-gray-50 text-gray-700 border-gray-300",
        ButtonVariant::Navigation => "bg-white hover:bg-gray-50 active:bg-gray-100 text-gray-900 border-gray-300",
    };
    
    let size_class = match size {
        ButtonSize::Small => "px-2 py-1 text-xs",
        ButtonSize::Medium => "px-3 py-2 text-sm",
        ButtonSize::Large => "px-4 py-2 text-base",
    };
    
    rsx! {
        button {
            class: "font-medium border rounded-lg transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed {variant_class} {size_class} {additional_class}",
            disabled,
            onclick: move |evt| {
                if !disabled {
                    onclick.call(evt);
                }
            },
            {children}
        }
    }
}

/// Export button component for export actions with specific styling
#[component]
pub fn ExportButton(
    onclick: EventHandler<MouseEvent>,
    disabled: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let disabled = disabled.unwrap_or(false);
    let additional_class = class.unwrap_or_default();
    
    rsx! {
        button {
            class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg text-sm transition-colors shadow-sm hover:shadow-md flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed {additional_class}",
            disabled,
            onclick: move |evt| {
                if !disabled {
                    onclick.call(evt);
                }
            },
            {children}
        }
    }
}

/// Confirm button component for positive actions
#[component]
pub fn ConfirmButton(
    onclick: EventHandler<MouseEvent>,
    disabled: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let disabled = disabled.unwrap_or(false);
    let additional_class = class.unwrap_or_default();
    
    rsx! {
        button {
            class: "flex-1 bg-green-600 hover:bg-green-700 text-white text-sm py-2 px-2 rounded-md shadow-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed {additional_class}",
            disabled,
            onclick: move |evt| {
                if !disabled {
                    onclick.call(evt);
                }
            },
            {children}
        }
    }
}

/// Reject button component for negative actions
#[component]
pub fn RejectButton(
    onclick: EventHandler<MouseEvent>,
    disabled: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let disabled = disabled.unwrap_or(false);
    let additional_class = class.unwrap_or_default();
    
    rsx! {
        button {
            class: "flex-1 bg-red-600 hover:bg-red-700 text-white text-sm py-2 px-2 rounded-md shadow-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed {additional_class}",
            disabled,
            onclick: move |evt| {
                if !disabled {
                    onclick.call(evt);
                }
            },
            {children}
        }
    }
}
