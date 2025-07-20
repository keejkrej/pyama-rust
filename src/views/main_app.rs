use dioxus::prelude::*;
use crate::components::{
    ActiveView, Channel, PatternState, PatternStatus,
    ViewerPane, TracesPane, TopBar, Sidebar
};

#[component]
pub fn MainApp() -> Element {
    let mut active_view = use_signal(|| ActiveView::Viewer);
    let mut selected_channel = use_signal(|| Channel::PhaseContrast);
    let mut current_timepoint = use_signal(|| 1);
    let mut current_position = use_signal(|| 1);
    let mut current_frame = use_signal(|| 1);
    let mut selected_pattern = use_signal(|| None::<usize>);
    let mut pattern_states = use_signal(|| {
        // Initialize 64 patterns (8x8 grid)
        (0..64).map(|_| {
            PatternState {
                has_cell: fastrand::f32() > 0.4, // Random cell presence
                status: None,
            }
        }).collect::<Vec<_>>()
    });

    let max_timepoint = 100;
    let max_position = 10;
    let max_frame = 500;

    rsx! {
        div { class: "h-screen flex flex-col",
            TopBar { project_name: "siRNA_knockdown_exp1".to_string() }
            
            div { class: "flex flex-1 overflow-hidden",
                Sidebar {
                    active_view: active_view(),
                    on_view_change: move |view| active_view.set(view),
                }

                // Center Panel: Main Content
                main { class: "flex-1 bg-gray-100",
                    match active_view() {
                        ActiveView::Viewer => rsx! {
                            ViewerPane {
                                selected_channel: selected_channel(),
                                current_timepoint: current_timepoint(),
                                max_timepoint,
                                current_position: current_position(),
                                max_position,
                                current_frame: current_frame(),
                                max_frame,
                                selected_pattern: selected_pattern(),
                                pattern_states: pattern_states.read().clone(),
                                on_channel_change: move |channel| selected_channel.set(channel),
                                on_timepoint_change: move |timepoint| current_timepoint.set(timepoint),
                                on_position_change: move |position| current_position.set(position),
                                on_frame_change: move |frame| current_frame.set(frame),
                                on_pattern_select: move |pattern_id| selected_pattern.set(Some(pattern_id)),
                                on_pattern_state_change: move |(pattern_id, status): (usize, PatternStatus)| {
                                    pattern_states.with_mut(|states| {
                                        if let Some(state) = states.get_mut(pattern_id) {
                                            state.status = Some(status);
                                        }
                                    });
                                },
                            }
                        },
                        ActiveView::Traces => rsx! {
                            TracesPane {}
                        },
                    }
                }
            }
            footer { class: "bg-gray-200 border-t border-gray-300 p-2 text-xs text-gray-600 text-center",
                "Status: Idle"
            }
        }
    }
}
