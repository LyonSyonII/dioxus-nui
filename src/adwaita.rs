#![allow(non_snake_case)]

use dioxus::prelude::*;

const TEXT_COLOR: &str = "#FFFFFF";
const ACCENT_COLOR: &str = "#3584E4";
const BACKGROUND_COLOR: &str = "#242424";
const ELEMENT_COLOR: &str = "#3A3A3A";
const ELEMENT_HOVER_COLOR: &str = "#404040";
const ELEMENT_CLICKED_COLOR: &str = "#666666";

type MouseEventHandler<'a> = EventHandler<'a, MouseEvent>;

pub enum ButtonStyle {
    Regular,
    Compact,
    Pill,
    Circular,
}

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
    #[props(default = false)]
    disabled: bool,

    // Custom properties
    #[props(default = ButtonStyle::Regular)]
    button_style: ButtonStyle,
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
        button_style,
    } = cx.props;
    
    let style = concat!(include_str!("../styles/Button/index.css"), include_str!("../styles/Button/regular.css"));
    
    // TODO! En comptes de fer un document per cada tema, fer una linia que s'hagi d'incloure a `style` al principi del programa, creant les classes corresponents.
    let rsx = rsx!(
        style {
            style
        }
        button { class: "btn btn-regular",
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
