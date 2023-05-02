use dioxus::prelude::*;
use dioxus_nui_macros::render_component;
use reusable::reuse;
use crate::{class, init::CheckIfUninit};

#[reuse(global_attributes, global_events)]
#[derive(Props)]
pub struct HeaderProps<'a> {
    children: Element<'a>,
}

pub fn H1<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { .. } = cx.props;

    render_component! {
        h1 {
            $CLASS: "{class::h1}",
            $GLOBALS,
            $CHILDREN
        }
    }
}

pub fn H2<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { .. } = cx.props;

    render_component! {
        h1 {
            $CLASS: "nui-h2",
            $GLOBALS,
            $CHILDREN
        }
    }
}

pub fn H3<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { .. } = cx.props;

    render_component! {
        h1 {
            $CLASS: "nui-h3",
            $GLOBALS,
            $CHILDREN
        }
    }
}

pub fn H4<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { .. } = cx.props;

    render_component! {
        h1 {
            $CLASS: "nui-h4",
            $GLOBALS,
            $CHILDREN
        }
    }
}