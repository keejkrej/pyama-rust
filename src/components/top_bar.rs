use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn TopBar(project_name: String) -> Element {
    rsx! {
        header { class: "bg-white border-b border-gray-200 px-4 py-3",
            div { class: "flex items-center justify-between",
                div { class: "flex items-center gap-4",
                    div { class: "project-info",
                        span { class: "text-sm text-gray-600", "Project:" }
                        span { class: "text-sm font-semibold text-indigo-600 hover:underline cursor-pointer", "{project_name}" }
                    }
                }
                div { class: "flex items-center",
                    Link {
                        to: Route::WelcomeScreen {},
                        class: "flex items-center gap-2 py-2 px-3 rounded-md text-gray-500 hover:bg-gray-100 hover:text-gray-900 transition-colors text-sm",
                        svg { 
                            class: "w-4 h-4", 
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
            }
        }
    }
}
