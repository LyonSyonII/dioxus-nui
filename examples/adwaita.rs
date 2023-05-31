#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_nui::prelude::include_css;
use dioxus_nui::{
    class, Align, Button, ButtonStyle, InitNui, Input, InputType, List, ListItem, Theme, H1, H2,
    H3, H4,
};

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
                H1 { class: "{class::accent_hover}", "Title 1" }
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

            // List
            List { class: "nui-example-list",
                ListItem {
                    class: "{class::accent_hover} {class::h1}",
                    title: "Lists",
                    align: dioxus_nui::Align::Center
                }
                ListItem { title: "Title", subtitle: "Subtitle" }
                ListItem { title: "Only Title" }
                ListItem { subtitle: "Only Subtitle" }
                ListItem { title: "Left aligned (Default)", subtitle: "align: Align::Left" }
                ListItem {
                    title: "Center aligned",
                    subtitle: "align: Align::Center",
                    align: Align::Center
                }
                ListItem { title: "Right aligned", subtitle: "align: Align::Right", align: Align::Right }
                ListItem {
                    title: "Row with Prefix",
                    prefix: render! { Button { "Prefix" } }
                }
                ListItem {
                    title: "Row with Suffix",
                    suffix: render! { Button { "Suffix" } }
                }
                ListItem {
                    title: "Row with both Prefix and Suffix"
                    subtitle: "You can use any component you want!"
                    prefix: render! { Button { "Prefix" } }
                    suffix: render! { Button { "Suffix" } }
                    align: Align::Center
                }
            }
            
            // Inputs
            List {
                ListItem {
                    Input {
                        label: "Pick a color:"
                        input_type: InputType::Color
                        on_change: move |v| println!("{v}")
                    }
                }
                Input {
                    on_change: move |v| println!("{v}")
                    input_type: InputType::Button
                    value: "Button"
                }
                Input {
                    label: "Checkbox"
                    input_type: InputType::Checkbox
                    on_change: move |v| println!("{v}")
                }
                Input {
                    label: "Pick a color:"
                    input_type: InputType::Color
                    on_change: move |v| println!("{v}")
                }
                Input {
                    label: "Enter a date:"
                    input_type: InputType::Date
                    on_change: move |v| println!("{v}")
                }
                Input {
                    label: "Enter a date and time:"
                    on_change: move |v| println!("{v}")
                    input_type: InputType::DatetimeLocal
                }
                Input {
                    label: "Enter your email:"
                    on_change: move |v| println!("{v}")
                    input_type: InputType::Email
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::File,
                }
                Input {
                    label: "Should be hidden"
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Hidden,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Image,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Month,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Number,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Password,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Radio,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Range,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Reset,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Search,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Submit,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Tel,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Text,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Time,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Url,
                }
                Input {
                    on_change: move |v| println!("{v}"),
                    input_type: InputType::Week,
                }
            }
        }
    }
}
