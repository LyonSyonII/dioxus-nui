#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_easyui::prelude::include_css;
use dioxus_easyui::{Button, ButtonStyle, InitEasyGui, List, ListItem, Theme, H1, H2, H3, H4};

fn main() {
    // hot_reload_init!(dioxus_hot_reload::Config::new().with_paths(&["src", "examples", "styles"]).with_rebuild_command("cargo run --example adwaita"));
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        // Include useful classes for the example
        style { include_str!("../examples/index.css") }

        // Example starts here
        InitEasyGui { theme: Theme::Adwaita } // Important: You MUST initialize EasyGui

        div { class: "easygui-example-root",
            // Headers
            div {
                H1 {
                    "Title 1",
                    accent: true
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
                    onclick: move |_| println!("It works!!!"),
                    "Regular"
                }
                Button {
                    "Accent",
                    accent: true,
                }
                Button {
                    "Disabled",
                    disabled: true,
                    onclick: move |_| println!("Should not print anything"),
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

            List { class: "easygui-example-list",
                ListItem {
                    title: "Item 1",
                    subtitle: "Subtitle 1"
                }
                ListItem {
                    title: "Item 2",
                    subtitle: "Subtitle 2"
                }
            }
        }
    }
}
