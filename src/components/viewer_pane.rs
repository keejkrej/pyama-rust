use dioxus::prelude::*;
use super::{Channel, PatternState, PatternStatus, PatternCell, DetailPanel};

#[component]
pub fn ViewerPane(
    selected_channel: Channel,
    current_timepoint: i32,
    max_timepoint: i32,
    selected_pattern: Option<usize>,
    pattern_states: Vec<PatternState>,
    on_channel_change: EventHandler<Channel>,
    on_timepoint_change: EventHandler<i32>,
    on_pattern_select: EventHandler<usize>,
    on_pattern_state_change: EventHandler<(usize, PatternStatus)>,
    on_switch_to_traces: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "p-6 flex flex-col h-full",
            // Header
            div { class: "flex-none",
                div { class: "flex justify-between items-center mb-4",
                    h1 { class: "text-3xl font-bold text-gray-900", "Viewer" }
                    button { 
                        class: "bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded-lg transition-colors shadow-sm hover:shadow-md flex items-center gap-2",
                        onclick: move |_| on_switch_to_traces.call(()),
                        svg { 
                            class: "w-5 h-5", 
                            fill: "none", 
                            stroke: "currentColor", 
                            "viewBox": "0 0 24 24",
                            path { 
                                "stroke-linecap": "round", 
                                "stroke-linejoin": "round", 
                                "stroke-width": "2", 
                                d: "M13 10V3L4 14h7v7l9-11h-7z" 
                            }
                        }
                        "Run Extraction & View Traces"
                    }
                }
                // Controls
                div { class: "grid grid-cols-2 gap-4 mb-4 bg-white p-3 rounded-lg border border-gray-200 shadow-sm",
                    div {
                        label { class: "text-sm font-medium text-gray-600 block mb-1", "Measurement" }
                        select { class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-indigo-500 focus:border-indigo-500 block w-full p-2",
                            option { selected: true, "Position 1" }
                            option { "Position 2" }
                        }
                    }
                    div {
                        label { class: "text-sm font-medium text-gray-600 block mb-1", "Channel" }
                        div { class: "flex gap-2",
                            button { 
                                class: format!("channel-btn flex-1 {}", if selected_channel == Channel::PhaseContrast { "active" } else { "" }),
                                onclick: move |_| on_channel_change.call(Channel::PhaseContrast),
                                "Phase Contrast"
                            }
                            button { 
                                class: format!("channel-btn flex-1 {}", if selected_channel == Channel::Fluorescence { "active" } else { "" }),
                                onclick: move |_| on_channel_change.call(Channel::Fluorescence),
                                "Fluorescence"
                            }
                            button { 
                                class: format!("channel-btn flex-1 {}", if selected_channel == Channel::Segmentation { "active" } else { "" }),
                                onclick: move |_| on_channel_change.call(Channel::Segmentation),
                                "Segmentation"
                            }
                        }
                    }
                }
            }
            
            // Main grid and detail panel
            div { class: "flex-1 grid grid-cols-4 gap-4 overflow-hidden",
                // Micropattern grid
                div { class: "col-span-3 bg-white rounded-lg p-4 border border-gray-200 flex flex-col shadow-sm",
                    div { class: "micropattern-grid flex-1 overflow-y-auto pr-2 grid grid-cols-8 gap-2",
                        for (i, pattern_state) in pattern_states.iter().enumerate() {
                            PatternCell {
                                key: "{i}",
                                pattern_id: i,
                                pattern_state: pattern_state.clone(),
                                is_selected: selected_pattern == Some(i),
                                on_select: move |pattern_id| on_pattern_select.call(pattern_id),
                            }
                        }
                    }
                    // Time navigation
                    div { class: "flex-none pt-4 flex items-center justify-center gap-4",
                        button { 
                            class: "time-nav-btn",
                            onclick: move |_| on_timepoint_change.call(1),
                            "<<"
                        }
                        button { 
                            class: "time-nav-btn",
                            onclick: move |_| on_timepoint_change.call((current_timepoint - 1).max(1)),
                            "<"
                        }
                        div { class: "text-sm font-medium text-gray-600 px-4",
                            "Timepoint: {current_timepoint} / {max_timepoint}"
                        }
                        button { 
                            class: "time-nav-btn",
                            onclick: move |_| on_timepoint_change.call((current_timepoint + 1).min(max_timepoint)),
                            ">"
                        }
                        button { 
                            class: "time-nav-btn",
                            onclick: move |_| on_timepoint_change.call(max_timepoint),
                            ">>"
                        }
                    }
                }
                
                // Detail panel
                DetailPanel {
                    selected_pattern,
                    pattern_states: pattern_states.clone(),
                    on_pattern_state_change,
                }
            }
        }
    }
}
