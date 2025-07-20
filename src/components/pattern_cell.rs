use dioxus::prelude::*;
use super::{PatternState, PatternStatus};

#[component]
pub fn PatternCell(
    pattern_id: usize,
    pattern_state: PatternState,
    is_selected: bool,
    on_select: EventHandler<usize>,
) -> Element {
    let mut cell_class = "micropattern-cell cursor-pointer bg-gray-900 rounded-md flex items-center justify-center aspect-square transition-transform hover:scale-105".to_string();
    
    if is_selected {
        cell_class.push_str(" ring-4 ring-indigo-500");
    }
    
    match &pattern_state.status {
        Some(PatternStatus::Confirmed) => cell_class.push_str(" ring-2 ring-green-600"),
        Some(PatternStatus::Rejected) => cell_class.push_str(" ring-2 ring-red-600"),
        None => {}
    }

    let placeholder_url = if pattern_state.has_cell {
        "https://placehold.co/64x64/111827/a7f3d0?text=&font=inter"
    } else {
        "https://placehold.co/64x64/111827/4b5563?text=&font=inter"
    };

    rsx! {
        div { 
            class: "{cell_class}",
            onclick: move |_| on_select.call(pattern_id),
            img { 
                src: "{placeholder_url}",
                class: "w-full h-full object-cover rounded-md",
                alt: "Pattern {pattern_id}"
            }
        }
    }
}
