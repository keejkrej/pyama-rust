use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    #[props(default = None)]
    pub class: Option<String>,
    #[props(default = None)]
    pub style: Option<String>,
    #[props(default = true)]
    pub horizontal: bool,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let class = props.class.unwrap_or_default();
    let orientation = if props.horizontal { "horizontal" } else { "vertical" };
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/ui/components/separator/variants/main/style.css"),
        }
        
        div {
            class: format!("separator {}", class),
            "data-orientation": orientation,
            style: props.style.unwrap_or_default(),
            role: "separator",
            "aria-orientation": orientation,
        }
    }
}