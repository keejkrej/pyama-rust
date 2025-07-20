use dioxus::prelude::*;
use crate::ui::components::{Button, Input, Dropdown, Separator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    PhaseContrast,
    Fluorescence,
    Segmentation,
}

impl Channel {
    pub fn to_string(&self) -> String {
        match self {
            Channel::PhaseContrast => "Phase Contrast".to_string(),
            Channel::Fluorescence => "Fluorescence".to_string(),
            Channel::Segmentation => "Segmentation".to_string(),
        }
    }
    
    pub fn from_string(s: &str) -> Self {
        match s {
            "phase_contrast" => Channel::PhaseContrast,
            "fluorescence" => Channel::Fluorescence,
            "segmentation" => Channel::Segmentation,
            _ => Channel::PhaseContrast,
        }
    }
    
    pub fn to_channel_index(&self) -> usize {
        match self {
            Channel::PhaseContrast => 0,
            Channel::Fluorescence => 1,
            Channel::Segmentation => 2,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ViewerPanelProps {
    #[props(default = 1)]
    pub position: i32,
    #[props(default = 1)]
    pub frame: i32,
    #[props(default = Channel::PhaseContrast)]
    pub channel: Channel,
    #[props(default = None)]
    pub on_position_change: Option<EventHandler<i32>>,
    #[props(default = None)]
    pub on_frame_change: Option<EventHandler<i32>>,
    #[props(default = None)]
    pub on_channel_change: Option<EventHandler<Channel>>,
}

#[component]
pub fn ViewerPanel(props: ViewerPanelProps) -> Element {
    let mut position_input = use_signal(|| props.position.to_string());
    let mut frame_input = use_signal(|| props.frame.to_string());
    
    let channel_options = vec![
        ("phase_contrast".to_string(), "Phase Contrast".to_string()),
        ("fluorescence".to_string(), "Fluorescence".to_string()),
        ("segmentation".to_string(), "Segmentation".to_string()),
    ];
    
    let selected_channel = match props.channel {
        Channel::PhaseContrast => "phase_contrast",
        Channel::Fluorescence => "fluorescence", 
        Channel::Segmentation => "segmentation",
    }.to_string();
    
    rsx! {
        style {
            "
            .viewer-panel {{
                display: flex;
                flex-direction: column;
                height: 100%;
                background-color: var(--primary-color);
            }}
            .control-bar {{
                background-color: var(--primary-color-2);
                border-bottom: 1px solid var(--primary-color-6);
                padding: 1rem;
                display: flex;
                align-items: center;
                gap: 1.5rem;
                flex-wrap: wrap;
            }}
            .control-group {{
                display: flex;
                align-items: center;
                gap: 0.5rem;
            }}
            .control-label {{
                font-size: 0.875rem;
                font-weight: 500;
                color: var(--secondary-color-4);
                white-space: nowrap;
            }}
            .control-input {{
                width: 5rem;
            }}
            .control-dropdown {{
                width: 10rem;
            }}
            .view-section {{
                flex: 1;
                background-color: var(--primary-color-3);
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--secondary-color-5);
                font-style: italic;
            }}
            .view-placeholder {{
                text-align: center;
                padding: 3rem;
                border: 2px dashed var(--primary-color-6);
                border-radius: 0.75rem;
                margin: 2rem;
            }}
            .view-title {{
                font-size: 1.25rem;
                font-weight: 600;
                color: var(--secondary-color-4);
                margin-bottom: 0.5rem;
            }}
            .current-settings {{
                font-size: 0.875rem;
                color: var(--secondary-color-5);
                margin-top: 1rem;
            }}
            "
        }
        
        div { class: "viewer-panel",
            // Control Bar
            div { class: "control-bar",
                div { class: "control-group",
                    label { class: "control-label", "Position:" }
                    Input {
                        class: "control-input".to_string(),
                        input_type: "number".to_string(),
                        value: position_input().to_string(),
                        oninput: move |evt: FormEvent| position_input.set(evt.value()),
                        placeholder: "1".to_string(),
                    }
                }
                
                div { class: "control-group",
                    label { class: "control-label", "Frame:" }
                    Input {
                        class: "control-input".to_string(),
                        input_type: "number".to_string(),
                        value: frame_input().to_string(),
                        oninput: move |evt: FormEvent| frame_input.set(evt.value()),
                        placeholder: "1".to_string(),
                    }
                }
                
                div { class: "control-group",
                    label { class: "control-label", "Channel:" }
                    Dropdown {
                        class: "control-dropdown".to_string(),
                        options: channel_options,
                        selected: selected_channel,
                        placeholder: "Select channel...".to_string(),
                        onchange: move |value: String| {
                            if let Some(handler) = props.on_channel_change.as_ref() {
                                handler.call(Channel::from_string(&value));
                            }
                        },
                    }
                }
                
                Button {
                    variant: "primary".to_string(),
                    onclick: move |_| {
                        // Handle position update
                        if let Ok(pos) = position_input().parse::<i32>() {
                            if let Some(handler) = props.on_position_change.as_ref() {
                                handler.call(pos);
                            }
                        }
                        // Handle frame update
                        if let Ok(frame) = frame_input().parse::<i32>() {
                            if let Some(handler) = props.on_frame_change.as_ref() {
                                handler.call(frame);
                            }
                        }
                    },
                    "Apply"
                }
            }
            
            // View Section
            div { class: "view-section",
                div { class: "view-placeholder",
                    div { class: "view-title", "Image Viewer" }
                    p { "Main grid and detail panel will be displayed here" }
                    
                    Separator { 
                        style: "margin: 1rem 0; width: 100%;".to_string(),
                        horizontal: true 
                    }
                    
                    div { class: "current-settings",
                        p { "Current Settings:" }
                        p { "Position: {props.position} | Frame: {props.frame}" }
                        p { "Channel: {props.channel.to_string()}" }
                    }
                }
            }
        }
    }
}