#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_easyui::adwaita::Button;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Button { onclick: move |_| println!("a"), "Hello!" }
    })
}
