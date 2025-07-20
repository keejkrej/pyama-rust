use dioxus::prelude::*;
use crate::ui::components::{Button, Separator};
use crate::routes::Route;

#[component]
pub fn WelcomeScreen() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/theme.css"),
        }
        
        style {
            "
            .project-item {{
                cursor: pointer;
                padding: 0.5rem;
                border-radius: 0.375rem;
                transition: background-color 0.2s;
                color: var(--secondary-color-4);
            }}
            .project-item:hover {{
                background-color: var(--primary-color-5);
            }}
            "
        }
        
        div { 
            style: "display: flex; align-items: center; justify-content: center; min-height: 100vh; padding: 1rem; background-color: var(--primary-color);",
            div { 
                style: "text-align: center; padding: 2rem; background-color: var(--primary-color-2); border-radius: 0.75rem; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04); width: 100%; max-width: 28rem; border: 1px solid var(--primary-color-6);",
                
                div { 
                    style: "display: flex; justify-content: center; align-items: center; margin-bottom: 1rem;",
                    svg { 
                        style: "width: 3rem; height: 3rem; color: var(--focused-border-color);", 
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
                
                h1 { 
                    style: "font-size: 1.875rem; font-weight: bold; color: var(--secondary-color-1); margin-bottom: 0.5rem; margin-top: 0;", 
                    "PyAMA" 
                }
                
                p { 
                    style: "color: var(--secondary-color-5); margin-bottom: 2rem; margin-top: 0;", 
                    "Python-based Automated Microstructure Analysis" 
                }
                
                div { 
                    style: "display: flex; flex-direction: column; gap: 1rem;",
                    Link {
                        to: Route::MainApp {},
                        style: "text-decoration: none;",
                        Button {
                            variant: "primary".to_string(),
                            class: "w-full".to_string(),
                            "Create New Project"
                        }
                    }
                    Link {
                        to: Route::MainApp {},
                        style: "text-decoration: none;",
                        Button {
                            variant: "secondary".to_string(),
                            class: "w-full".to_string(),
                            "Open Existing Project"
                        }
                    }
                }
                
                Separator {
                    style: "margin: 2rem 0; width: 100%;".to_string(),
                    horizontal: true,
                }
                
                div { 
                    style: "text-align: left;",
                    h2 { 
                        style: "font-size: 1.125rem; font-weight: 600; color: var(--secondary-color-4); margin-bottom: 0.75rem; margin-top: 0;", 
                        "Recent Projects" 
                    }
                    ul { 
                        style: "list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.5rem;",
                        li { 
                            class: "project-item",
                            onclick: move |_| {
                                navigator().push(Route::MainApp {});
                            },
                            "siRNA_knockdown_exp1"
                        }
                        li { 
                            class: "project-item",
                            onclick: move |_| {
                                navigator().push(Route::MainApp {});
                            },
                            "Drug_response_trial_3"
                        }
                        li { 
                            class: "project-item",
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