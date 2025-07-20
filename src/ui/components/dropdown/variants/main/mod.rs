use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    pub options: Vec<(String, String)>, // (value, label)
    #[props(default = None)]
    pub selected: Option<String>,
    #[props(default = None)]
    pub placeholder: Option<String>,
    #[props(default = None)]
    pub onchange: Option<EventHandler<String>>,
    #[props(default = None)]
    pub class: Option<String>,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut is_open = use_signal(|| false);
    let class = props.class.unwrap_or_default();
    
    let selected_label = props.selected.as_ref()
        .and_then(|val| props.options.iter().find(|(v, _)| v == val))
        .map(|(_, label)| label.clone())
        .unwrap_or_else(|| props.placeholder.clone().unwrap_or_else(|| "Select...".to_string()));
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/ui/components/dropdown/variants/main/style.css"),
        }

        div {
            class: format!("dropdown {}", class),
            
            div {
                class: "dropdown-trigger",
                onclick: move |_| is_open.set(!is_open()),
                
                span { class: "dropdown-value", "{selected_label}" }
                svg {
                    class: format!("dropdown-icon {}", if is_open() { "open" } else { "" }),
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    path {
                        d: "m6 9 6 6 6-6",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                    }
                }
            }
            
            if is_open() {
                div {
                    class: "dropdown-menu",
                    for (value, label) in props.options.iter() {
                        div {
                            class: format!("dropdown-item {}", 
                                if props.selected.as_ref() == Some(value) { "selected" } else { "" }),
                            onclick: {
                                let value = value.clone();
                                let onchange = props.onchange.clone();
                                move |_| {
                                    if let Some(handler) = onchange.as_ref() {
                                        handler.call(value.clone());
                                    }
                                    is_open.set(false);
                                }
                            },
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}