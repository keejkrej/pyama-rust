use dioxus::prelude::*;
use super::{PatternState, PatternStatus};

#[component]
pub fn DetailPanel(
    selected_pattern: Option<usize>,
    pattern_states: Vec<PatternState>,
    on_pattern_state_change: EventHandler<(usize, PatternStatus)>,
) -> Element {
    rsx! {
        div { class: "col-span-1 bg-white rounded-lg p-4 flex flex-col border border-gray-200 shadow-sm",
            h3 { class: "font-semibold text-gray-900 mb-3", "Micropattern Details" }
            
            if let Some(pattern_id) = selected_pattern {
                if let Some(pattern_state) = pattern_states.get(pattern_id) {
                    div { class: "flex-1 flex items-center justify-center bg-gray-900 rounded-md mb-3",
                        img { 
                            src: if pattern_state.has_cell {
                                "https://placehold.co/256x256/111827/a7f3d0?text=Cell&font=inter"
                            } else {
                                "https://placehold.co/256x256/111827/4b5563?text=Empty&font=inter"
                            },
                            class: "max-w-full max-h-full object-contain",
                            alt: "Selected micropattern"
                        }
                    }
                    div { class: "text-center",
                        h4 { class: "font-bold text-lg", "P1-R{pattern_id / 8 + 1}-C{pattern_id % 8 + 1}" }
                        p { 
                            class: if pattern_state.has_cell { "text-sm text-green-600" } else { "text-sm text-gray-500" },
                            if pattern_state.has_cell { "Cell Detected" } else { "Empty" }
                        }
                        
                        match &pattern_state.status {
                            Some(PatternStatus::Confirmed) => rsx! {
                                div { class: "mt-4 p-2 text-sm font-semibold text-green-800 bg-green-100 rounded-md",
                                    "Status: Confirmed ROI"
                                }
                            },
                            Some(PatternStatus::Rejected) => rsx! {
                                div { class: "mt-4 p-2 text-sm font-semibold text-red-800 bg-red-100 rounded-md",
                                    "Status: Rejected"
                                }
                            },
                            None if pattern_state.has_cell => rsx! {
                                div { class: "mt-4 flex gap-2",
                                    button { 
                                        class: "flex-1 bg-green-600 hover:bg-green-700 text-white text-sm py-2 px-2 rounded-md shadow-sm transition-colors",
                                        onclick: move |_| on_pattern_state_change.call((pattern_id, PatternStatus::Confirmed)),
                                        "Confirm as ROI"
                                    }
                                    button { 
                                        class: "flex-1 bg-red-600 hover:bg-red-700 text-white text-sm py-2 px-2 rounded-md shadow-sm transition-colors",
                                        onclick: move |_| on_pattern_state_change.call((pattern_id, PatternStatus::Rejected)),
                                        "Reject"
                                    }
                                }
                            },
                            None => rsx! { div {} }
                        }
                    }
                } else {
                    p { class: "text-sm text-gray-500", "Pattern not found." }
                }
            } else {
                div { class: "flex-1 flex items-center justify-center bg-gray-900 rounded-md mb-3",
                    img { 
                        src: "https://placehold.co/256x256/111827/4b5563?text=Select+a+pattern&font=inter",
                        class: "max-w-full max-h-full object-contain",
                        alt: "No pattern selected"
                    }
                }
                div { class: "text-center",
                    p { class: "text-sm text-gray-500", "No pattern selected." }
                }
            }
        }
    }
}
