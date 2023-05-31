use crate::{class, MapStr, ToStr};
use dioxus::prelude::*;
use dioxus_nui_macros::render_component;
use reusable::reuse;

// `global_events` and `global_attributes` from crate::global
#[reuse(global_events, global_attributes)]
#[derive(Props)]
pub struct ButtonProps<'a> {
    children: Element<'a>,

    // Attributes
    disabled: Option<bool>,

    // Custom properties
    #[props(default)]
    button_style: ButtonStyle,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let ButtonProps {
        button_style,
        disabled,
        ..
    } = cx.props;

    render_component! {
        button {
            $CLASS: "{class::btn} {button_style}",

            disabled: disabled.map_str(),

            $GLOBALS,
            $CHILDREN
        }
    }
}

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Regular,
    Compact,
    Pill,
    Circular,
}

impl ToStr<'static> for ButtonStyle {
    fn to_str(&self) -> &'static str {
        match self {
            ButtonStyle::Regular => class::btn_regular,
            ButtonStyle::Compact => class::btn_compact,
            ButtonStyle::Pill => class::btn_pill,
            ButtonStyle::Circular => class::btn_circular,
        }
    }
}

impl std::fmt::Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
