#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_easyui::{Button, ButtonStyle, InitEasyGui, Theme, H1, H2, H3, H4};

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        // Include useful classes from Tailwind, not needed
        style { include_str!("tailwind.css") }

        // Example starts here
        InitEasyGui { theme: Theme::Adwaita } // Important: You MUST initialize EasyGui
        div { class: "easygui-example-root",
            // Headers
            div {
                H1 {
                    "Title 1"
                }
                H2 {
                    "Title 2"
                }
                H3 {
                    "Title 3"
                }
                H4 {
                    "Title 4"
                }
            }

            // Buttons
            div { class: "easygui-example-buttongrid",
                Button {
                    button_style: ButtonStyle::Regular,
                    "Regular"
                }
                Button {
                    button_style: ButtonStyle::Compact,
                    "Compact"
                }
                Button {
                    button_style: ButtonStyle::Pill,
                    "Pill"
                }
                Button {
                    button_style: ButtonStyle::Circular,
                    "C"
                }
            }
        }
    })
}
