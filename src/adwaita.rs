#![allow(non_snake_case)]

use dioxus::{prelude::*, html::AttributeDiscription};

// fn onclick<'a, E, T>(_cx: &'a ::dioxus_core::ScopeState, _f: impl FnMut(::dioxus_core::Event<MouseData>) -> E + 'a) -> ::dioxus_core::Attribute<'a>
// where
//     E: crate::EventReturn<T> {
//
//     }

type MouseEventHandler<'a> = EventHandler<'a, MouseEvent>;

#[derive(Props)]
pub struct ButtonProps<'a> {
    children: Element<'a>,
    // Mouse Events
    #[props(default = MouseEventHandler::default())]
    onclick: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    oncontextmenu: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    ondoubleclick: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    ondblclick: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    onmousedown: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    onmouseenter: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    onmouseleave: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    onmousemove: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    onmouseout: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    onmouseover: MouseEventHandler<'a>,
    #[props(default = MouseEventHandler::default())]
    onmouseup: MouseEventHandler<'a>,

    // Attributes
    #[props(default)]
    autofocus: bool,
    #[props(default)]
    disabled: bool,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let ButtonProps {
        children,
        onclick,
        oncontextmenu,
        ondoubleclick,
        ondblclick,
        onmousedown,
        onmouseenter,
        onmouseleave,
        onmousemove,
        onmouseout,
        onmouseover,
        onmouseup,
        autofocus,
        disabled,
    } = cx.props;

    let rsx = rsx!(
        button {
            onclick: move |e| onclick.call(e),
            oncontextmenu: move |e| oncontextmenu.call(e),
            ondoubleclick: move |e| ondoubleclick.call(e),
            ondblclick: move |e| ondblclick.call(e),
            onmousedown: move |e| onmousedown.call(e),
            onmouseenter: move |e| onmouseenter.call(e),
            onmouseleave: move |e| onmouseleave.call(e),
            onmousemove: move |e| onmousemove.call(e),
            onmouseout: move |e| onmouseout.call(e),
            onmouseover: move |e| onmouseover.call(e),
            onmouseup: move |e| onmouseup.call(e),

            autofocus: *autofocus,
            disabled: *disabled,

            children
        }
    );
    cx.render(rsx)
}
