use dioxus::prelude::*;
use super::{Channel, PatternState, PatternStatus, PatternCell, DetailPanel, PrimaryButton, ToggleButton, NavigationButton};

#[component]
pub fn ViewerPane(
    selected_channel: Channel,
    current_timepoint: i32,
    max_timepoint: i32,
    current_position: i32,
    max_position: i32,
    current_frame: i32,
    max_frame: i32,
    selected_pattern: Option<usize>,
    pattern_states: Vec<PatternState>,
    on_channel_change: EventHandler<Channel>,
    on_timepoint_change: EventHandler<i32>,
    on_position_change: EventHandler<i32>,
    on_frame_change: EventHandler<i32>,
    on_pattern_select: EventHandler<usize>,
    on_pattern_state_change: EventHandler<(usize, PatternStatus)>,
) -> Element {
    let mut temp_position = use_signal(|| current_position.to_string());
    let mut temp_frame = use_signal(|| current_frame.to_string());
    
    // Update temp values when props change
    use_effect(move || {
        temp_position.set(current_position.to_string());
    });
    
    use_effect(move || {
        temp_frame.set(current_frame.to_string());
    });

    rsx! {
        div { class: "p-4 flex flex-col h-full",
            // Controls
            div { class: "flex-none mb-3",
                div { class: "flex gap-4 bg-white p-2 rounded-lg border border-gray-200 shadow-sm",
                    div {
                        label { class: "text-sm font-medium text-gray-600 block mb-1", "Position & Frame" }
                        div { class: "flex gap-2 items-end",
                            div { class: "flex gap-2",
                                input { 
                                    class: "position-input bg-white border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 p-2 transition-all duration-200 hover:border-gray-400 shadow-sm max-w-24",
                                    r#type: "number",
                                    min: "1",
                                    max: "{max_position}",
                                    value: "{temp_position()}",
                                    placeholder: "Position",
                                    oninput: move |evt| temp_position.set(evt.value()),
                                    onkeydown: move |evt| {
                                        if evt.key() == Key::Enter {
                                            if let Ok(pos) = temp_position().parse::<i32>() {
                                                on_position_change.call(pos.max(1).min(max_position));
                                            }
                                            if let Ok(frame) = temp_frame().parse::<i32>() {
                                                on_frame_change.call(frame.max(1).min(max_frame));
                                            }
                                        }
                                    }
                                }
                                input { 
                                    class: "frame-input bg-white border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 p-2 transition-all duration-200 hover:border-gray-400 shadow-sm max-w-24",
                                    r#type: "number",
                                    min: "1",
                                    max: "{max_frame}",
                                    value: "{temp_frame()}",
                                    placeholder: "Frame",
                                    oninput: move |evt| temp_frame.set(evt.value()),
                                    onkeydown: move |evt| {
                                        if evt.key() == Key::Enter {
                                            if let Ok(pos) = temp_position().parse::<i32>() {
                                                on_position_change.call(pos.max(1).min(max_position));
                                            }
                                            if let Ok(frame) = temp_frame().parse::<i32>() {
                                                on_frame_change.call(frame.max(1).min(max_frame));
                                            }
                                        }
                                    }
                                }
                            }
                            PrimaryButton { 
                                onclick: move |_| {
                                    if let Ok(pos) = temp_position().parse::<i32>() {
                                        on_position_change.call(pos.max(1).min(max_position));
                                    }
                                    if let Ok(frame) = temp_frame().parse::<i32>() {
                                        on_frame_change.call(frame.max(1).min(max_frame));
                                    }
                                },
                                "Apply"
                            }
                        }
                    }
                    div { class: "flex-1",
                        label { class: "text-sm font-medium text-gray-600 block mb-1", "Channel" }
                        div { class: "flex gap-2",
                            ToggleButton { 
                                active: selected_channel == Channel::PhaseContrast,
                                onclick: move |_| on_channel_change.call(Channel::PhaseContrast),
                                "Phase Contrast"
                            }
                            ToggleButton { 
                                active: selected_channel == Channel::Fluorescence,
                                onclick: move |_| on_channel_change.call(Channel::Fluorescence),
                                "Fluorescence"
                            }
                            ToggleButton { 
                                active: selected_channel == Channel::Segmentation,
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
                        NavigationButton { 
                            onclick: move |_| on_timepoint_change.call(1),
                            "<<"
                        }
                        NavigationButton { 
                            onclick: move |_| on_timepoint_change.call((current_timepoint - 1).max(1)),
                            "<"
                        }
                        div { class: "text-sm font-medium text-gray-600 px-4",
                            "Timepoint: {current_timepoint} / {max_timepoint}"
                        }
                        NavigationButton { 
                            onclick: move |_| on_timepoint_change.call((current_timepoint + 1).min(max_timepoint)),
                            ">"
                        }
                        NavigationButton { 
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
