use dioxus::prelude::*;
use crate::ui::components::{Separator, Button};
use crate::io::array_6d::FrameStats;

#[derive(Debug, Clone, PartialEq)]
pub enum DataLoadingState {
    NotLoaded,
    Loading,
    Loaded,
    #[allow(dead_code)]
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageData {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_kb: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FluorescenceData {
    pub image_data: ImageData,
    pub intensity_stats: IntensityStats,
    pub channel_info: ChannelInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntensityStats {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub total_pixels: u32,
    pub saturated_pixels: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelInfo {
    pub wavelength: String,
    pub exposure_time: f64,
    pub gain: f64,
    pub filter: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SegmentationData {
    pub image_data: ImageData,
    pub cell_count: u32,
    pub analysis_params: AnalysisParams,
    pub region_stats: Vec<RegionStats>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisParams {
    pub algorithm: String,
    pub threshold: f64,
    pub min_area: u32,
    pub max_area: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegionStats {
    pub id: u32,
    pub area: f64,
    pub perimeter: f64,
    pub circularity: f64,
    pub centroid_x: f64,
    pub centroid_y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelData {
    PhaseContrast(ImageData),
    Fluorescence(FluorescenceData),
    Segmentation(SegmentationData),
    MicroscopyArray(MicroscopyArrayData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MicroscopyArrayData {
    pub file_path: String,
    pub dimensions: ArrayDimensions,
    pub metadata: ArrayMetadata,
    pub current_frame_stats: Option<FrameStats>,
    pub current_frame_image: Option<String>, // Base64 encoded image
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayDimensions {
    pub time: usize,
    pub position: usize,
    pub z: usize,
    pub channel: usize,
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayMetadata {
    pub pixel_size_um: f64,
    pub time_interval_s: f64,
    pub channel_names: Vec<String>,
    pub data_type: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct DataPanelProps {
    pub channel: crate::ui::components::Channel,
    pub position: i32,
    pub frame: i32,
    #[props(default = DataLoadingState::NotLoaded)]
    pub loading_state: DataLoadingState,
    #[props(default = None)]
    pub data: Option<ChannelData>,
    #[props(default = None)]
    pub on_load_data: Option<EventHandler<(crate::ui::components::Channel, i32, i32)>>,
    #[props(default = None)]
    pub on_load_file: Option<EventHandler<String>>,
}

#[component]
pub fn DataPanel(props: DataPanelProps) -> Element {
    let channel = props.channel.clone();
    
    // Mock data generation for demonstration
    let _generate_mock_data = move |channel: &crate::ui::components::Channel, position: i32, frame: i32| -> ChannelData {
        match channel {
            crate::ui::components::Channel::PhaseContrast => {
                ChannelData::PhaseContrast(ImageData {
                    path: format!("images/phase_contrast_p{}_f{}.tiff", position, frame),
                    width: 2048,
                    height: 2048,
                    format: "TIFF".to_string(),
                    size_kb: 8192,
                })
            },
            crate::ui::components::Channel::Fluorescence => {
                ChannelData::Fluorescence(FluorescenceData {
                    image_data: ImageData {
                        path: format!("images/fluorescence_p{}_f{}.tiff", position, frame),
                        width: 2048,
                        height: 2048,
                        format: "TIFF 16-bit".to_string(),
                        size_kb: 16384,
                    },
                    intensity_stats: IntensityStats {
                        mean: 1024.5,
                        median: 987.2,
                        std_dev: 456.7,
                        min: 12.0,
                        max: 4095.0,
                        total_pixels: 4194304,
                        saturated_pixels: 1247,
                    },
                    channel_info: ChannelInfo {
                        wavelength: "488nm".to_string(),
                        exposure_time: 100.0,
                        gain: 2.5,
                        filter: "FITC".to_string(),
                    },
                })
            },
            crate::ui::components::Channel::Segmentation => {
                ChannelData::Segmentation(SegmentationData {
                    image_data: ImageData {
                        path: format!("analysis/segmentation_p{}_f{}.png", position, frame),
                        width: 2048,
                        height: 2048,
                        format: "PNG".to_string(),
                        size_kb: 2048,
                    },
                    cell_count: 127,
                    analysis_params: AnalysisParams {
                        algorithm: "Watershed".to_string(),
                        threshold: 0.75,
                        min_area: 50,
                        max_area: 2000,
                    },
                    region_stats: vec![
                        RegionStats {
                            id: 1,
                            area: 245.6,
                            perimeter: 58.3,
                            circularity: 0.91,
                            centroid_x: 156.7,
                            centroid_y: 289.1,
                        },
                        RegionStats {
                            id: 2,
                            area: 189.2,
                            perimeter: 52.1,
                            circularity: 0.87,
                            centroid_x: 345.2,
                            centroid_y: 178.9,
                        },
                        // ... more regions would be here
                    ],
                })
            },
        }
    };

    rsx! {
        style {
            "
            .data-panel {{
                background-color: var(--primary-color-2);
                border: 1px solid var(--primary-color-6);
                border-radius: 0.5rem;
                padding: 1rem;
                height: 100%;
                overflow-y: auto;
            }}
            .data-header {{
                display: flex;
                align-items: center;
                justify-content: space-between;
                margin-bottom: 1rem;
            }}
            .data-title {{
                font-size: 1rem;
                font-weight: 600;
                color: var(--secondary-color-1);
            }}
            .load-button {{
                font-size: 0.75rem;
                padding: 0.25rem 0.5rem;
            }}
            .data-content {{
                color: var(--secondary-color-4);
                font-size: 0.875rem;
            }}
            .data-section {{
                margin-bottom: 1rem;
            }}
            .section-title {{
                font-weight: 600;
                color: var(--secondary-color-1);
                margin-bottom: 0.5rem;
            }}
            .data-grid {{
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 0.5rem;
                margin-bottom: 0.5rem;
            }}
            .data-item {{
                display: flex;
                justify-content: space-between;
                padding: 0.25rem 0;
            }}
            .data-label {{
                color: var(--secondary-color-5);
            }}
            .data-value {{
                color: var(--secondary-color-1);
                font-weight: 500;
            }}
            .loading-state {{
                text-align: center;
                padding: 2rem;
                color: var(--secondary-color-5);
                font-style: italic;
            }}
            .error-state {{
                text-align: center;
                padding: 2rem;
                color: var(--primary-error-color);
            }}
            .region-list {{
                max-height: 200px;
                overflow-y: auto;
                border: 1px solid var(--primary-color-6);
                border-radius: 0.25rem;
                padding: 0.5rem;
            }}
            .region-item {{
                padding: 0.25rem 0;
                border-bottom: 1px solid var(--primary-color-6);
                font-size: 0.75rem;
            }}
            .region-item:last-child {{
                border-bottom: none;
            }}
            "
        }
        
        div { class: "data-panel",
            div { class: "data-header",
                h3 { class: "data-title", 
                    "{channel.to_string()} Data"
                }
                if matches!(props.loading_state, DataLoadingState::NotLoaded) {
                    Button {
                        variant: "default".to_string(),
                        class: "load-button".to_string(),
                        onclick: move |_| {
                            let handler = props.on_load_file.clone();
                            spawn(async move {
                                if let Some(handler) = handler {
                                    match crate::services::select_6d_file().await {
                                        Ok(Some(path)) => {
                                            handler.call(path.to_string_lossy().to_string());
                                        }
                                        Ok(None) => {
                                            // User cancelled - do nothing
                                        }
                                        Err(e) => {
                                            println!("Error opening file dialog: {}", e);
                                        }
                                    }
                                }
                            });
                        },
                        "Load 6D Data"
                    }
                }
            }
            
            match props.loading_state {
                DataLoadingState::NotLoaded => rsx! {
                    div { class: "loading-state",
                        "Click 'Load 6D Data' to browse and select a 6D microscopy file"
                        br {}
                        "Supported formats: .meta files with corresponding .data files"
                    }
                },
                DataLoadingState::Loading => rsx! {
                    div { class: "loading-state",
                        "Loading 6D array data..."
                    }
                },
                DataLoadingState::Error(ref error) => rsx! {
                    div { class: "error-state",
                        "Error loading data: {error}"
                    }
                },
                DataLoadingState::Loaded => {
                    if let Some(ref data) = props.data {
                        match data {
                            ChannelData::PhaseContrast(ref img_data) => rsx! {
                                div { class: "data-content",
                                    div { class: "data-section",
                                        div { class: "section-title", "Image Information" }
                                        div { class: "data-item",
                                            span { class: "data-label", "File:" }
                                            span { class: "data-value", "{img_data.path}" }
                                        }
                                        div { class: "data-grid",
                                            div { class: "data-item",
                                                span { class: "data-label", "Dimensions:" }
                                                span { class: "data-value", "{img_data.width} × {img_data.height}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Format:" }
                                                span { class: "data-value", "{img_data.format}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Size:" }
                                                span { class: "data-value", "{img_data.size_kb} KB" }
                                            }
                                        }
                                    }
                                }
                            },
                            ChannelData::Fluorescence(ref fluor_data) => rsx! {
                                div { class: "data-content",
                                    div { class: "data-section",
                                        div { class: "section-title", "Image Information" }
                                        div { class: "data-item",
                                            span { class: "data-label", "File:" }
                                            span { class: "data-value", "{fluor_data.image_data.path}" }
                                        }
                                        div { class: "data-grid",
                                            div { class: "data-item",
                                                span { class: "data-label", "Dimensions:" }
                                                span { class: "data-value", "{fluor_data.image_data.width} × {fluor_data.image_data.height}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Format:" }
                                                span { class: "data-value", "{fluor_data.image_data.format}" }
                                            }
                                        }
                                    }
                                    
                                    Separator { 
                                        style: "margin: 1rem 0; width: 100%;".to_string(),
                                        horizontal: true 
                                    }
                                    
                                    div { class: "data-section",
                                        div { class: "section-title", "Channel Settings" }
                                        div { class: "data-grid",
                                            div { class: "data-item",
                                                span { class: "data-label", "Wavelength:" }
                                                span { class: "data-value", "{fluor_data.channel_info.wavelength}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Filter:" }
                                                span { class: "data-value", "{fluor_data.channel_info.filter}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Exposure:" }
                                                span { class: "data-value", "{fluor_data.channel_info.exposure_time:.1} ms" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Gain:" }
                                                span { class: "data-value", "{fluor_data.channel_info.gain:.1}x" }
                                            }
                                        }
                                    }
                                    
                                    Separator { 
                                        style: "margin: 1rem 0; width: 100%;".to_string(),
                                        horizontal: true 
                                    }
                                    
                                    div { class: "data-section",
                                        div { class: "section-title", "Intensity Statistics" }
                                        div { class: "data-grid",
                                            div { class: "data-item",
                                                span { class: "data-label", "Mean:" }
                                                span { class: "data-value", "{fluor_data.intensity_stats.mean:.1}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Median:" }
                                                span { class: "data-value", "{fluor_data.intensity_stats.median:.1}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Std Dev:" }
                                                span { class: "data-value", "{fluor_data.intensity_stats.std_dev:.1}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Range:" }
                                                span { class: "data-value", "{fluor_data.intensity_stats.min:.0} - {fluor_data.intensity_stats.max:.0}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Saturated:" }
                                                span { class: "data-value", "{fluor_data.intensity_stats.saturated_pixels}" }
                                            }
                                        }
                                    }
                                }
                            },
                            ChannelData::Segmentation(ref seg_data) => rsx! {
                                div { class: "data-content",
                                    div { class: "data-section",
                                        div { class: "section-title", "Analysis Results" }
                                        div { class: "data-grid",
                                            div { class: "data-item",
                                                span { class: "data-label", "Cell Count:" }
                                                span { class: "data-value", "{seg_data.cell_count}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Algorithm:" }
                                                span { class: "data-value", "{seg_data.analysis_params.algorithm}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Threshold:" }
                                                span { class: "data-value", "{seg_data.analysis_params.threshold:.2}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Size Range:" }
                                                span { class: "data-value", "{seg_data.analysis_params.min_area} - {seg_data.analysis_params.max_area} px" }
                                            }
                                        }
                                    }
                                    
                                    Separator { 
                                        style: "margin: 1rem 0; width: 100%;".to_string(),
                                        horizontal: true 
                                    }
                                    
                                    div { class: "data-section",
                                        div { class: "section-title", "Detected Regions (First 10)" }
                                        div { class: "region-list",
                                            for (_i, region) in seg_data.region_stats.iter().enumerate().take(10) {
                                                div { class: "region-item",
                                                    "#{region.id}: Area {region.area:.1} px², Circularity {region.circularity:.2}"
                                                }
                                            }
                                            if seg_data.region_stats.len() > 10 {
                                                div { class: "region-item",
                                                    "... and {seg_data.region_stats.len() - 10} more regions"
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            ChannelData::MicroscopyArray(ref array_data) => rsx! {
                                div { class: "data-content",
                                    div { class: "data-section",
                                        div { class: "section-title", "6D Array Information" }
                                        div { class: "data-item",
                                            span { class: "data-label", "File:" }
                                            span { class: "data-value", "{array_data.file_path}" }
                                        }
                                        div { class: "data-grid",
                                            div { class: "data-item",
                                                span { class: "data-label", "Dimensions (TPZCYX):" }
                                                span { class: "data-value", "{array_data.dimensions.time}×{array_data.dimensions.position}×{array_data.dimensions.z}×{array_data.dimensions.channel}×{array_data.dimensions.height}×{array_data.dimensions.width}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Data Type:" }
                                                span { class: "data-value", "{array_data.metadata.data_type}" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Pixel Size:" }
                                                span { class: "data-value", "{array_data.metadata.pixel_size_um:.3} μm" }
                                            }
                                            div { class: "data-item",
                                                span { class: "data-label", "Time Interval:" }
                                                span { class: "data-value", "{array_data.metadata.time_interval_s:.1} s" }
                                            }
                                        }
                                    }
                                    
                                    Separator { 
                                        style: "margin: 1rem 0; width: 100%;".to_string(),
                                        horizontal: true 
                                    }
                                    
                                    div { class: "data-section",
                                        div { class: "section-title", "Available Channels" }
                                        div { class: "data-grid",
                                            for (i, channel_name) in array_data.metadata.channel_names.iter().enumerate() {
                                                div { class: "data-item",
                                                    span { class: "data-label", "Channel {i}:" }
                                                    span { class: "data-value", "{channel_name}" }
                                                }
                                            }
                                        }
                                    }
                                    
                                    if let Some(ref stats) = array_data.current_frame_stats {
                                        Separator { 
                                            style: "margin: 1rem 0; width: 100%;".to_string(),
                                            horizontal: true 
                                        }
                                        
                                        div { class: "data-section",
                                            div { class: "section-title", "Current Frame Statistics (T:{props.frame}, C:{props.channel.to_channel_index()})" }
                                            div { class: "data-grid",
                                                div { class: "data-item",
                                                    span { class: "data-label", "Mean:" }
                                                    span { class: "data-value", "{stats.mean:.1}" }
                                                }
                                                div { class: "data-item",
                                                    span { class: "data-label", "Median:" }
                                                    span { class: "data-value", "{stats.median:.1}" }
                                                }
                                                div { class: "data-item",
                                                    span { class: "data-label", "Std Dev:" }
                                                    span { class: "data-value", "{stats.std_dev:.1}" }
                                                }
                                                div { class: "data-item",
                                                    span { class: "data-label", "Range:" }
                                                    span { class: "data-value", "{stats.min:.0} - {stats.max:.0}" }
                                                }
                                                div { class: "data-item",
                                                    span { class: "data-label", "Total Pixels:" }
                                                    span { class: "data-value", "{stats.total_pixels}" }
                                                }
                                                div { class: "data-item",
                                                    span { class: "data-label", "Saturated:" }
                                                    span { class: "data-value", "{stats.saturated_pixels}" }
                                                }
                                            }
                                        }
                                    }
                                    
                                    if let Some(ref _image_data) = array_data.current_frame_image {
                                        Separator { 
                                            style: "margin: 1rem 0; width: 100%;".to_string(),
                                            horizontal: true 
                                        }
                                        
                                        div { class: "data-section",
                                            div { class: "section-title", "Frame Preview" }
                                            div { style: "text-align: center; padding: 1rem; border: 1px solid var(--primary-color-6); border-radius: 0.25rem;",
                                                "Frame visualization will be implemented in the viewer panel"
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    } else {
                        rsx! {
                            div { class: "loading-state",
                                "No data available"
                            }
                        }
                    }
                }
            }
        }
    }
}