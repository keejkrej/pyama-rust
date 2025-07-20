use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = "primary".to_string())]
    pub variant: String,
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
    #[props(default = None)]
    pub class: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = props.class.unwrap_or_default();
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/ui/components/button/variants/main/style.css"),
        }

        button {
            class: format!("button {}", class),
            "data-style": props.variant,
            onclick: move |evt| {
                if let Some(handler) = props.onclick.as_ref() {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}