use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn WelcomeScreen() -> Element {
    rsx! {
        div { class: "flex items-center justify-center h-screen bg-gray-100",
            div { class: "text-center p-8 bg-white rounded-lg shadow-2xl w-full max-w-md border border-gray-200",
                div { class: "flex justify-center items-center mb-4",
                    svg { 
                        class: "w-12 h-12 text-indigo-600", 
                        fill: "none", 
                        stroke: "currentColor", 
                        "viewBox": "0 0 24 24",
                        path { 
                            "stroke-linecap": "round", 
                            "stroke-linejoin": "round", 
                            "stroke-width": "2", 
                            d: "M9.75 3.104l7.5 7.5-7.5 7.5" 
                        }
                        path { 
                            "stroke-linecap": "round", 
                            "stroke-linejoin": "round", 
                            "stroke-width": "2", 
                            d: "M6 3v18" 
                        }
                    }
                }
                h1 { class: "text-3xl font-bold text-gray-900 mb-2", "PyAMA" }
                p { class: "text-gray-500 mb-8", "Python-based Automated Microstructure Analysis" }
                div { class: "space-y-4",
                    Link {
                        to: Route::MainApp {},
                        class: "w-full bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-3 px-4 rounded-lg transition-colors shadow-sm hover:shadow-md block text-center",
                        "Create New Project"
                    }
                    Link {
                        to: Route::MainApp {},
                        class: "w-full bg-gray-200 hover:bg-gray-300 text-gray-800 font-bold py-3 px-4 rounded-lg transition-colors block text-center",
                        "Open Existing Project"
                    }
                }
                div { class: "mt-8 text-left",
                    h2 { class: "text-lg font-semibold text-gray-600 mb-3", "Recent Projects" }
                    ul { class: "space-y-2",
                        li { 
                            class: "cursor-pointer hover:bg-gray-100 p-2 rounded-md transition-colors text-gray-700",
                            onclick: move |_| {
                                navigator().push(Route::MainApp {});
                            },
                            "siRNA_knockdown_exp1"
                        }
                        li { 
                            class: "cursor-pointer hover:bg-gray-100 p-2 rounded-md transition-colors text-gray-700",
                            onclick: move |_| {
                                navigator().push(Route::MainApp {});
                            },
                            "Drug_response_trial_3"
                        }
                        li { 
                            class: "cursor-pointer hover:bg-gray-100 p-2 rounded-md transition-colors text-gray-700",
                            onclick: move |_| {
                                navigator().push(Route::MainApp {});
                            },
                            "Control_group_2024-07-15"
                        }
                    }
                }
            }
        }
    }
}
