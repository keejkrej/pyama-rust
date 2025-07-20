use dioxus::prelude::*;
use crate::ui::components::{Separator, ViewerPanel, Channel, DataPanel, ChannelData, DataLoadingState};
use crate::services::{MicroscopyMetadata, load_array_file};
use crate::routes::Route;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveView {
    Viewer,
    Traces,
    Data,
}

#[component]
pub fn MainApp() -> Element {
    let mut active_view = use_signal(|| ActiveView::Data);
    let mut position = use_signal(|| 1);
    let mut frame = use_signal(|| 1);
    let mut channel = use_signal(|| Channel::PhaseContrast);
    let mut data_loading_state = use_signal(|| DataLoadingState::NotLoaded);
    let mut current_data = use_signal(|| None::<ChannelData>);
    let microscopy_metadata = use_signal(|| None::<MicroscopyMetadata>);

    // Mock data generation function
    let generate_mock_data = move |channel: Channel, position: i32, frame: i32| -> ChannelData {
        use crate::ui::components::data_panel::*;
        match channel {
            Channel::PhaseContrast => {
                ChannelData::PhaseContrast(ImageData {
                    path: format!("images/phase_contrast_p{}_f{}.tiff", position, frame),
                    width: 2048,
                    height: 2048,
                    format: "TIFF".to_string(),
                    size_kb: 8192,
                })
            },
            Channel::Fluorescence => {
                ChannelData::Fluorescence(FluorescenceData {
                    image_data: ImageData {
                        path: format!("images/fluorescence_p{}_f{}.tiff", position, frame),
                        width: 2048,
                        height: 2048,
                        format: "TIFF 16-bit".to_string(),
                        size_kb: 16384,
                    },
                    intensity_stats: IntensityStats {
                        mean: 1024.5 + (position as f64 * 10.0) + (frame as f64 * 2.0),
                        median: 987.2 + (position as f64 * 8.0),
                        std_dev: 456.7,
                        min: 12.0,
                        max: 4095.0,
                        total_pixels: 4194304,
                        saturated_pixels: 1247 + (frame as u32 * 3),
                    },
                    channel_info: ChannelInfo {
                        wavelength: "488nm".to_string(),
                        exposure_time: 100.0,
                        gain: 2.5,
                        filter: "FITC".to_string(),
                    },
                })
            },
            Channel::Segmentation => {
                ChannelData::Segmentation(SegmentationData {
                    image_data: ImageData {
                        path: format!("analysis/segmentation_p{}_f{}.png", position, frame),
                        width: 2048,
                        height: 2048,
                        format: "PNG".to_string(),
                        size_kb: 2048,
                    },
                    cell_count: 127 + (position as u32 * 5) - (frame as u32 * 2),
                    analysis_params: AnalysisParams {
                        algorithm: "Watershed".to_string(),
                        threshold: 0.75,
                        min_area: 50,
                        max_area: 2000,
                    },
                    region_stats: vec![
                        RegionStats {
                            id: 1,
                            area: 245.6 + (position as f64 * 2.0),
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
                    ],
                })
            },
        }
    };

    // No automatic resets needed - let user control the state manually

    // Use use_callback to avoid closure capture issues
    let load_data_callback = use_callback(
        move |params: (Channel, i32, i32)| {
            println!("Starting mock data load...");
            data_loading_state.set(DataLoadingState::Loading);
            
            let mock_data = generate_mock_data(params.0, params.1, params.2);
            current_data.set(Some(mock_data));
            data_loading_state.set(DataLoadingState::Loaded);
            println!("Mock data loaded successfully");
        }
    );
    
    // Callback for loading array files (lightweight metadata only)
    let load_file_callback = use_callback(
        move |file_path: String| {
            println!("Starting array file metadata load: {}", file_path);
            data_loading_state.set(DataLoadingState::Loading);
            
            let mut microscopy_metadata_signal = microscopy_metadata.clone();
            let mut current_data_signal = current_data.clone();
            let mut data_loading_state_signal = data_loading_state.clone();
            
            spawn(async move {
                match load_array_file(file_path.clone()).await {
                    Ok(metadata) => {
                        let array_data = crate::ui::components::data_panel::MicroscopyArrayData {
                            file_path: metadata.file_path.clone(),
                            dimensions: crate::ui::components::data_panel::ArrayDimensions {
                                time: metadata.dimensions.time,
                                position: metadata.dimensions.position,
                                z: metadata.dimensions.z,
                                channel: metadata.dimensions.channel,
                                height: metadata.dimensions.height,
                                width: metadata.dimensions.width,
                            },
                            metadata: crate::ui::components::data_panel::ArrayMetadata {
                                pixel_size_um: metadata.pixel_size_um,
                                time_interval_s: metadata.time_interval_s,
                                channel_names: metadata.channel_names.clone(),
                                data_type: metadata.data_type.clone(),
                            },
                            current_frame_stats: None, // No heavy data processing
                            current_frame_image: None, // No image generation
                        };
                        
                        microscopy_metadata_signal.set(Some(metadata));
                        current_data_signal.set(Some(ChannelData::MicroscopyArray(array_data)));
                        data_loading_state_signal.set(DataLoadingState::Loaded);
                        println!("Array file metadata loaded successfully");
                    }
                    Err(e) => {
                        println!("Error loading array file: {}", e);
                        data_loading_state_signal.set(DataLoadingState::Error(format!("Failed to load file: {}", e)));
                    }
                }
            });
        }
    );

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/theme.css"),
        }
        
        style {
            "
            .main-layout {{
                display: flex;
                flex-direction: column;
                height: 100vh;
                background-color: var(--primary-color);
            }}
            .top-bar {{
                background-color: var(--primary-color-2);
                border-bottom: 1px solid var(--primary-color-6);
                padding: 0.75rem 1rem;
                display: flex;
                align-items: center;
                justify-content: space-between;
            }}
            .project-info {{
                display: flex;
                align-items: center;
                gap: 1rem;
            }}
            .project-label {{
                font-size: 0.875rem;
                color: var(--secondary-color-5);
            }}
            .project-name {{
                font-size: 0.875rem;
                font-weight: 600;
                color: var(--focused-border-color);
                cursor: pointer;
                text-decoration: none;
            }}
            .project-name:hover {{
                text-decoration: underline;
            }}
            .close-button {{
                display: flex;
                align-items: center;
                gap: 0.5rem;
                padding: 0.5rem 0.75rem;
                border-radius: 0.375rem;
                color: var(--secondary-color-5);
                background-color: transparent;
                font-size: 0.875rem;
                text-decoration: none;
                transition: all 0.2s;
            }}
            .close-button:hover {{
                background-color: var(--primary-color-5);
                color: var(--secondary-color-1);
            }}
            .main-content {{
                display: flex;
                flex: 1;
                overflow: hidden;
            }}
            .sidebar {{
                width: 12rem;
                background-color: var(--primary-color-2);
                padding: 1rem;
                display: flex;
                flex-direction: column;
                border-right: 1px solid var(--primary-color-6);
            }}
            .nav-item {{
                display: flex;
                align-items: center;
                gap: 0.75rem;
                padding: 0.75rem;
                border-radius: 0.375rem;
                color: var(--secondary-color-4);
                cursor: pointer;
                transition: all 0.2s;
                text-decoration: none;
                margin-bottom: 0.5rem;
            }}
            .nav-item:hover {{
                background-color: var(--primary-color-5);
                color: var(--secondary-color-1);
            }}
            .nav-item.active {{
                background-color: var(--focused-border-color);
                color: var(--primary-color);
            }}
            .nav-item.active:hover {{
                background-color: var(--focused-border-color);
                color: var(--primary-color);
            }}
            .nav-icon {{
                width: 1rem;
                height: 1rem;
            }}
            .content-area {{
                flex: 1;
                background-color: var(--primary-color-3);
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--secondary-color-4);
            }}
            .placeholder-content {{
                text-align: center;
                padding: 2rem;
            }}
            .placeholder-title {{
                font-size: 1.5rem;
                font-weight: 600;
                margin-bottom: 0.5rem;
                color: var(--secondary-color-1);
            }}
            .footer {{
                background-color: var(--primary-color-5);
                border-top: 1px solid var(--primary-color-6);
                padding: 0.5rem 1rem;
                font-size: 0.75rem;
                color: var(--secondary-color-5);
                text-align: center;
            }}
            "
        }
        
        div { class: "main-layout",
            // Top Bar
            header { class: "top-bar",
                div { class: "project-info",
                    span { class: "project-label", "Project:" }
                    span { class: "project-name", "siRNA_knockdown_exp1" }
                }
                Link {
                    to: Route::WelcomeScreen {},
                    class: "close-button",
                    svg { 
                        class: "nav-icon", 
                        fill: "none", 
                        stroke: "currentColor", 
                        "viewBox": "0 0 24 24",
                        path { 
                            "stroke-linecap": "round", 
                            "stroke-linejoin": "round", 
                            "stroke-width": "2", 
                            d: "M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" 
                        }
                    }
                    "Close Project"
                }
            }
            
            // Main Content Area
            div { class: "main-content",
                // Sidebar
                aside { class: "sidebar",
                    nav {
                        div { 
                            class: format!("nav-item {}", if active_view() == ActiveView::Data { "active" } else { "" }),
                            onclick: move |_| active_view.set(ActiveView::Data),
                            svg { 
                                class: "nav-icon", 
                                fill: "none", 
                                stroke: "currentColor", 
                                "viewBox": "0 0 24 24",
                                path { 
                                    "stroke-linecap": "round", 
                                    "stroke-linejoin": "round", 
                                    "stroke-width": "2", 
                                    d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" 
                                }
                            }
                            span { "Data" }
                        }
                        div { 
                            class: format!("nav-item {}", if active_view() == ActiveView::Viewer { "active" } else { "" }),
                            onclick: move |_| active_view.set(ActiveView::Viewer),
                            svg { 
                                class: "nav-icon", 
                                fill: "none", 
                                stroke: "currentColor", 
                                "viewBox": "0 0 24 24",
                                path { 
                                    "stroke-linecap": "round", 
                                    "stroke-linejoin": "round", 
                                    "stroke-width": "2", 
                                    d: "M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.536L16.732 3.732z" 
                                }
                            }
                            span { "Viewer" }
                        }
                        div { 
                            class: format!("nav-item {}", if active_view() == ActiveView::Traces { "active" } else { "" }),
                            onclick: move |_| active_view.set(ActiveView::Traces),
                            svg { 
                                class: "nav-icon", 
                                fill: "none", 
                                stroke: "currentColor", 
                                "viewBox": "0 0 24 24",
                                path { 
                                    "stroke-linecap": "round", 
                                    "stroke-linejoin": "round", 
                                    "stroke-width": "2", 
                                    d: "M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" 
                                }
                            }
                            span { "Traces" }
                        }
                    }
                }

                // Main Content Area
                main { class: "content-area",
                    match active_view() {
                        ActiveView::Data => rsx! {
                            DataPanel {
                                channel: channel(),
                                position: position(),
                                frame: frame(),
                                loading_state: data_loading_state(),
                                data: current_data(),
                                on_load_data: move |params: (Channel, i32, i32)| {
                                    load_data_callback.call(params);
                                },
                                on_load_file: move |file_path: String| {
                                    load_file_callback.call(file_path);
                                },
                            }
                        },
                        ActiveView::Viewer => rsx! {
                            ViewerPanel {
                                position: position(),
                                frame: frame(),
                                channel: channel(),
                                on_position_change: move |new_pos| position.set(new_pos),
                                on_frame_change: move |new_frame| frame.set(new_frame),
                                on_channel_change: move |new_channel| channel.set(new_channel),
                            }
                        },
                        ActiveView::Traces => rsx! {
                            div { class: "placeholder-content",
                                h2 { class: "placeholder-title", "Traces Panel" }
                                p { "Trace analysis and visualization will go here" }
                                Separator { 
                                    style: "margin: 1rem 0; width: 100%;".to_string(),
                                    horizontal: true 
                                }
                                p { "This is where temporal data and cell tracking will be displayed" }
                            }
                        },
                    }
                }
            }
            
            // Footer
            footer { class: "footer",
                "Status: Ready • PyAMA v1.0 • dioxus primitives"
            }
        }
    }
}