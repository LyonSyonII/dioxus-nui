use dioxus::prelude::*;

#[inline_props]
pub(crate) fn Square(cx: Scope) -> Element {
    render! {
        svg {
            width: 24,
            height: 24,
            view_box: "0 0 24 24",
            fill: "none",

            path {
                fill_rule: "evenodd", clip_rule: "evenodd", d: "M17 7H7V17H17V7ZM4 4V20H20V4H4Z", fill: "currentColor"
            }
        }
    }
}
