#![allow(non_snake_case)]

use dioxus::html::button;
use dioxus::prelude::*;
use dioxus_nui::prelude::include_css;
use dioxus_nui::{Button, ButtonStyle, InitNui, List, ListItem, Theme, H1, H2, H3, H4};

fn main() {
    // hot_reload_init!(dioxus_hot_reload::Config::new().with_paths(&["src", "examples", "styles"]).with_rebuild_command("cargo run --example adwaita"));
    dioxus_desktop::launch(App);
}
fn App(cx: Scope) -> Element {
    render! {
        // Include useful classes for the example
        style { include_css!("examples/index.css") }

        // Example starts here
        InitNui { theme: Theme::Adwaita }
        
        div { class: "nui-example-root",
            // Headers
            div {
                H1 { class: "nui-accent-hover", "Title 1" }
                H2 { "Title 2" }
                H3 { "Title 3" }
                H4 { "Title 4" }
            }

            // Buttons
            div { class: "nui-example-buttongrid",
                Button { button_style: ButtonStyle::Regular, onclick: move |_| println!("It works!!!"), "Regular" }
                Button { accent: true, "Accent" }
                Button { disabled: true, onclick: move |_| println!("Should not print anything"), "Disabled" }
                Button { button_style: ButtonStyle::Compact, "Compact" }
                Button { button_style: ButtonStyle::Pill, "Pill" }
                Button { button_style: ButtonStyle::Circular, "C" }
            }

            List { class: "nui-example-list",
                ListItem {
                    class: "nui-accent-hover nui-h1",
                    title: "Lists",
                    align: dioxus_nui::Align::Center
                }
                ListItem { title: "Title", subtitle: "Subtitle" }
                ListItem { title: "Only Title" }
                ListItem { subtitle: "Only Subtitle" }
                ListItem { title: "Left aligned (Default)" }
                ListItem { title: "Center aligned", align: dioxus_nui::Align::Center }
                ListItem { title: "Right aligned", align: dioxus_nui::Align::Right }
                ListItem {
                    title: "Row with Prefix",
                    prefix: render! { Button { "Prefix" } }
                }
                ListItem {
                    title: "Row with Suffix",
                    suffix: render! { Button { "Suffix" } }
                }
                ListItem {
                    title: "Row with both Prefix and Suffix",
                    subtitle: "You can use any component you want!",
                    prefix: render! { Button { "Prefix" } },
                    suffix: render! { Button { "Suffix" } }
                }
            }
        }
    }
}
