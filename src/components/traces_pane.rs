use dioxus::prelude::*;
use super::ExportButton;

#[component]
pub fn TracesPane() -> Element {
    rsx! {
        div { class: "p-6 flex flex-col h-full",
            h1 { class: "text-3xl font-bold text-gray-900 mb-4", "Extracted Traces" }
            div { class: "flex-1 bg-white rounded-lg p-4 border border-gray-200 flex flex-col shadow-sm",
                div { class: "flex justify-between items-center mb-4",
                    h3 { class: "font-semibold text-gray-900", "Fluorescence Timelapses (45 cells)" }
                    ExportButton { 
                        onclick: move |_| {
                            // Export logic would go here
                        },
                        svg { 
                            class: "w-4 h-4", 
                            fill: "none", 
                            stroke: "currentColor", 
                            "viewBox": "0 0 24 24",
                            path { 
                                "stroke-linecap": "round", 
                                "stroke-linejoin": "round", 
                                "stroke-width": "2", 
                                d: "M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" 
                            }
                        }
                        "Export as CSV"
                    }
                }
                div { class: "flex-1 rounded-md bg-gray-50 p-4 flex items-center justify-center",
                    svg { 
                        width: "100%", 
                        height: "300", 
                        "viewBox": "0 0 700 300",
                        class: "text-gray-400",
                        // Chart axes
                        line { x1: "50", y1: "280", x2: "680", y2: "280", stroke: "currentColor", "stroke-width": "2" }
                        line { x1: "50", y1: "20", x2: "50", y2: "280", stroke: "currentColor", "stroke-width": "2" }
                        // Labels
                        text { x: "365", y: "298", "text-anchor": "middle", style: "font-size:12px; fill:currentColor;", "Time" }
                        text { x: "15", y: "150", "text-anchor": "middle", transform: "rotate(-90 15,150)", style: "font-size:12px; fill:currentColor;", "Fluorescence" }
                        // Sample traces
                        path { d: "M 50 250 C 150 100, 250 300, 350 150 S 550 50, 650 200", stroke: "#a78bfa", fill: "none", "stroke-width": "1.5", opacity: "0.7" }
                        path { d: "M 50 200 C 150 150, 250 250, 350 200 S 550 100, 650 150", stroke: "#7dd3fc", fill: "none", "stroke-width": "1.5", opacity: "0.7" }
                        path { d: "M 50 150 C 150 200, 250 100, 350 250 S 550 150, 650 100", stroke: "#fca5a5", fill: "none", "stroke-width": "1.5", opacity: "0.7" }
                        path { d: "M 50 180 C 150 280, 250 80, 350 180 S 550 280, 650 80", stroke: "#fdba74", fill: "none", "stroke-width": "1.5", opacity: "0.7" }
                    }
                }
            }
        }
    }
}
