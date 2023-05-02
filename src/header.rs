use crate::{class, init::CheckIfUninit, Align};
use dioxus::prelude::*;
use dioxus_nui_macros::render_component;
use reusable::reuse;

#[reuse(global_attributes, global_events)]
#[derive(Props)]
pub struct HeaderProps<'a> {
    #[props(default)]
    align: Align,
    children: Element<'a>,
}

pub fn H1<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { align, .. } = cx.props;

    render_component! {
        h1 {
            $CLASS: "{class::h1} {align}",
            $GLOBALS,
            $CHILDREN
        }
    }
}

pub fn H2<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { align, .. } = cx.props;
    
    render_component! {
        h2 {
            $CLASS: "{class::h2} {align}",
            $GLOBALS,
            $CHILDREN
        }
    }
}

pub fn H3<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { align, .. } = cx.props;
    
    render_component! {
        h3 {
            $CLASS: "{class::h3} {align}",
            $GLOBALS,
            $CHILDREN
        }
    }
}

pub fn H4<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { align, .. } = cx.props;
    
    render_component! {
        h4 {
            $CLASS: "{class::h4} {align}",
            $GLOBALS,
            $CHILDREN
        }
    }
}
