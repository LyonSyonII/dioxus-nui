#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Theme {
    Adwaita,
    Qt,
    Windows10,
    Windows11,
    Macos,
}

impl Theme {
    /// Returns CSS style corresponding to the current theme
    fn to_style(self) -> &'static str {
        match self {
            Theme::Adwaita => include_str!("../styles/adwaita.css"),
            Theme::Qt => todo!(),
            Theme::Windows10 => todo!(),
            Theme::Windows11 => todo!(),
            Theme::Macos => todo!(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        {
            Theme::Windows11
        }
        #[cfg(target_os = "macos")]
        {
            Theme::Macos
        }
        #[cfg(target_os = "linux")]
        {
            match std::env::var("XDG_SESSION_DESKTOP").as_deref() {
                Ok("kde") => Theme::Qt,
                Ok("gnome") | Ok("ubuntu") | Ok("pop") => Theme::Adwaita,
                _ => Theme::Adwaita,
            }
        }
    }
}

#[inline_props]
pub fn InitEasyGui(cx: Scope, theme: Option<Theme>) -> Element {
    cx.render(rsx! {
        style {
            theme.unwrap_or_default().to_style()
        }
    })
}

type MouseEventHandler<'a> = EventHandler<'a, MouseEvent>;

pub enum ButtonStyle {
    Regular,
    Compact,
    Pill,
    Circular,
}

impl std::fmt::Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            ButtonStyle::Regular => "easygui-btn-regular",
            ButtonStyle::Compact => "easygui-btn-compact",
            ButtonStyle::Pill => "easygui-btn-pill",
            ButtonStyle::Circular => "easygui-btn-circular",
        };

        f.write_str(d)
    }
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(into, default = String::new())]
    class: String,
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
    #[props(default)]
    accent: bool,

    // Custom properties
    #[props(default = ButtonStyle::Regular)]
    button_style: ButtonStyle,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let ButtonProps {
        class,
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
        accent,
        button_style,
    } = cx.props;

    let accent = if *accent { "accent" } else { "" };

    let rsx = rsx!(button {
        class: "easygui-btn {button_style} {accent} {class}",
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
    });
    cx.render(rsx)
}

// Headers
#[derive(Props)]
pub struct HeaderProps<'a> {
    #[props(into, default = String::new())]
    class: String,
    children: Element<'a>,
}

pub fn H1<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { class, children } = cx.props;
    
    cx.render(rsx! {
        h1 { class: "easygui-h1 {class}",
            children
        }
    })
}

pub fn H2<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { class, children } = cx.props;
    
    cx.render(rsx! {
        h1 { class: "easygui-h2 {class}",
            children
        }
    })
}

pub fn H3<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { class, children } = cx.props;
    
    cx.render(rsx! {
        h1 { class: "easygui-h3 {class}",
            children
        }
    })
}

pub fn H4<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { class, children } = cx.props;
    
    cx.render(rsx! {
        h1 { class: "easygui-h4 {class}",
            children
        }
    })
}