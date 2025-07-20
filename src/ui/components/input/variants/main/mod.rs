use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    #[props(default = None)]
    pub placeholder: Option<String>,
    #[props(default = None)]
    pub value: Option<String>,
    #[props(default = None)]
    pub oninput: Option<EventHandler<FormEvent>>,
    #[props(default = None)]
    pub class: Option<String>,
    #[props(default = "text".to_string())]
    pub input_type: String,
    #[props(default = false)]
    pub disabled: bool,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let class = props.class.unwrap_or_default();
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/ui/components/input/variants/main/style.css"),
        }

        input {
            class: format!("input {}", class),
            r#type: props.input_type,
            placeholder: props.placeholder.unwrap_or_default(),
            value: props.value.unwrap_or_default(),
            disabled: props.disabled,
            oninput: move |evt| {
                if let Some(handler) = props.oninput.as_ref() {
                    handler.call(evt);
                }
            },
        }
    }
}