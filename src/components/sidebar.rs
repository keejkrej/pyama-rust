use dioxus::prelude::*;
use super::ActiveView;

#[component]
pub fn Sidebar(
    active_view: ActiveView,
    on_view_change: EventHandler<ActiveView>,
) -> Element {
    rsx! {
        aside { 
            class: "w-20 bg-white/80 backdrop-blur-sm p-4 flex flex-col border-r border-gray-200 transition-all duration-300",
            div { class: "sidebar-content",
                nav { id: "main-nav",
                    ul { class: "space-y-2 items-center",
                        li {
                            a { 
                                class: format!("nav-link {}", if active_view == ActiveView::Viewer { "active" } else { "" }),
                                onclick: move |_| on_view_change.call(ActiveView::Viewer),
                                title: "Viewer",
                                svg { 
                                    class: "sidebar-icon", 
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
                                span { class: "nav-text hidden", "Viewer" }
                            }
                        }
                        li {
                            a { 
                                class: format!("nav-link {}", if active_view == ActiveView::Traces { "active" } else { "" }),
                                onclick: move |_| on_view_change.call(ActiveView::Traces),
                                title: "Traces",
                                svg { 
                                    class: "sidebar-icon", 
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
                                span { class: "nav-text hidden", "Traces" }
                            }
                        }
                    }
                }
            }
        }
    }
}
