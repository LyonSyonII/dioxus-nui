#![allow(non_snake_case)]

use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, rsx, DragEvent, Element, EventHandler,
    FocusEvent, FormEvent, GlobalAttributes as GA, ImageEvent, KeyboardEvent, MediaEvent,
    MouseEvent, Props, Scope, ScrollEvent, SelectionEvent, ToggleEvent, VNode,
};
use dioxus_nui_macros::render_component;
use reusable::reuse;

mod global;

/// Theme that NUI will use.
///
/// The default value changes depending on the target platform.
///
/// # Values
/// - Windows 11:
///     - Windows 10, Windows 11
/// - MacOS
/// - Linux
///     - Qt: KDE
///     - Adwaita: Gnome, Ubuntu, PopOS!
///
/// # Notes
/// - The Windows 11 theme is used on Windows 10 to avoid including extra dependencies.
/// - The Linux default is chosen based on the XDG_SESSION_DESKTOP env variable.
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
            // TODO: Temporary measure to test on other platforms until other styles are made
            _ | Theme::Adwaita => dioxus_nui_macros::include_css_safe!("styles/adwaita.css"),
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

/// Initializes NUI styling.
///
/// Must be used before any element imported from this crate.
///
/// If not used, it'll be initialized with a default value (depending on the platform), see [`Theme`](theme) for more information.
#[inline_props]
pub fn InitNui(cx: Scope, theme: Option<Theme>) -> Element {
    render! {
        style { 
            // display: "none",
            theme.unwrap_or_default().to_style() 
        }
    }
}

/// Checks if NUI is initialized.
///
/// If not, it returns an [`InitNui`](InitNui) element.
#[cfg(feature = "auto-init")]
fn CheckIfUninit(cx: Scope) -> Element {
    /// If `true` NUI has been initialized with a theme.
    /// 
    /// Atomic because dioxus can start CheckIfUninit from different Tokio threads
    static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    
    if INITIALIZED.load(std::sync::atomic::Ordering::Acquire) {
        return None;
    }

    println!("Initialized NUI");

    INITIALIZED.store(true, std::sync::atomic::Ordering::Release);

    render! {
        InitNui {}
    }
}


#[derive(Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

impl ToStr<'static> for Align {
    fn to_str(&self) -> &'static str {
        match self {
            Align::Left => "nui-align-left",
            Align::Center => "nui-align-center",
            Align::Right => "nui-align-right",
        }
    }
}

impl std::fmt::Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Regular,
    Compact,
    Pill,
    Circular,
}

impl ToStr<'static> for ButtonStyle {
    fn to_str(&self) -> &'static str {
        match self {
            ButtonStyle::Regular => "nui-btn--regular",
            ButtonStyle::Compact => "nui-btn--compact",
            ButtonStyle::Pill => "nui-btn--pill",
            ButtonStyle::Circular => "nui-btn--circular",
        }
    }
}

impl std::fmt::Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

// `global_events` and `global_attributes` from crate::global
#[reuse(global_events, global_attributes)]
#[derive(Props)]
pub struct ButtonProps<'a> {
    children: Element<'a>,

    // Attributes
    disabled: Option<bool>,

    // Custom properties
    #[props(default)]
    button_style: ButtonStyle,
}

impl dioxus::prelude::GlobalAttributes for ButtonProps<'_> {}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let ButtonProps {
        button_style,
        disabled,
        ..
    } = cx.props;
    
    // TODO! Works when outside the render_component! macro but not inside
    let init = {
        #[cfg(feature = "auto-init")]
        rsx! { CheckIfUninit {} }
        
        #[cfg(not(feature = "auto-init"))]
        rsx! {}
    };

    render_component! {
        init
        button {
            $CLASS: "nui-btn {button_style}",
            
            disabled: disabled.map_str(),

            $GLOBALS,
            $CHILDREN
        }
    }
}


// Headers
#[reuse(global_attributes, global_events)]
#[derive(Props)]
pub struct HeaderProps<'a> {
    children: Element<'a>,
}

pub fn H1<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let HeaderProps { .. } = cx.props;

    render_component! {
        h1 {
            $CLASS: "nui-h1",
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

#[reuse(global_attributes, global_events)]
#[derive(Props)]
pub struct ListProps<'a> {
    children: Element<'a>,
}

/// Creates a List of items.
///
/// To work properly, use the [`ListItem`](crate::ListItem) component.
///
/// If another element is used the result is unspecified.
pub fn List<'a>(cx: Scope<'a, ListProps<'a>>) -> Element {
    let ListProps { .. } = cx.props;

    render_component! {
        div {
            $CLASS: "nui-list",
            $GLOBALS,
            $CHILDREN
        }
    }
}

#[reuse(global_attributes, global_events)]
#[derive(Props)]
pub struct ListItemProps<'a> {
    /// Title of the list item.
    title: Option<&'a str>,

    /// Class of the title, useful if you want to change its style.
    title_class: Option<&'a str>,

    /// Subtitle of the list item.
    subtitle: Option<&'a str>,

    /// Class of the subtitle, useful if you want to change its style.
    subtitle_class: Option<&'a str>,

    /// Alignment of the Title and Subtitle attributes.
    ///
    /// `Align::Left` by default.
    #[props(default)]
    align: Align,

    /// Prefix of the list item.
    ///
    /// Can be any element.
    ///
    /// It will be positioned at the right side of the item.
    prefix: Option<VNode<'a>>,

    /// Suffix of the list item.
    ///
    /// Can be any element.
    ///
    /// It will be positioned at the left side of the item.
    suffix: Option<VNode<'a>>,
}

/// Creates a new item for a [`List`](crate::List).
///
/// Use only nested in a `List` element.
pub fn ListItem<'a>(cx: Scope<'a, ListItemProps<'a>>) -> Element {
    let ListItemProps {
        title,
        title_class,
        subtitle,
        subtitle_class,
        align,
        prefix,
        suffix,
        ..
    } = cx.props;

    let title = title.map(|t| {
        rsx! {
            p { class: "nui-list__item__title {title_class.unwrap_as_str()}", t }
        }
    });

    let subtitle = subtitle.map(|t| {
        rsx! {
            p { class: "nui-list__item__subtitle {subtitle_class.unwrap_as_str()}", t }
        }
    });

    let prefix = prefix.as_ref().map(|p| {
        rsx! {
            div { class: "nui-list__item__prefix", p }
        }
    });

    let suffix = suffix.as_ref().map(|s| {
        rsx! {
            div { class: "nui-list__item__suffix", s }
        }
    });

    render_component! {
        div {
            $CLASS: "nui-list__item",
            $GLOBALS,
            prefix,
            div { class: "{align}",
                title,
                subtitle,
            },
            suffix,
        }
    }
}

/// Re-export of all the elements with the same name as the [`dioxus`](dioxus::prelude::dioxus_elements) ones.
///
/// Useful if you want to replace all of them without having to change the names.
///
/// All components unique to dioxus-nui (for example [`List`](crate::List)) will remain with the same name.
pub mod prelude {
    pub use crate::Button as button;
    pub use crate::List;
    pub use crate::ListItem;
    pub use crate::H1 as h1;
    pub use crate::H2 as h2;
    pub use crate::H3 as h3;
    pub use crate::H4 as h4;
    pub use dioxus_nui_macros::include_css;
    pub use dioxus_nui_macros::include_css_safe;
}

// UTILS
trait ToStr<'a> {
    /// Converts value to a `&str`.
    ///
    /// Same as `to_string` but with `&str` instead.
    fn to_str(&self) -> &'a str;
}

impl ToStr<'static> for bool {
    fn to_str(&self) -> &'static str {
        if *self {
            "true"
        } else {
            "false"
        }
    }
}

impl<'a> ToStr<'a> for &'a str {
    fn to_str(&self) -> &'a str {
        self
    }
}

trait MapStr<'a> {
    type StrOut;
    /// Converts contained value to `&str`.
    ///
    /// Shorthand for `self.map(|s| s.to_str())`.
    fn map_str(self) -> Self::StrOut;
}

impl<'a, T: ToStr<'a>> MapStr<'a> for Option<T> {
    type StrOut = Option<&'a str>;
    fn map_str(self) -> Self::StrOut {
        self.map(|b| b.to_str())
    }
}

impl<'a, T: ToStr<'a>, E> MapStr<'a> for Result<T, E> {
    type StrOut = Result<&'a str, E>;
    fn map_str(self) -> Self::StrOut {
        self.map(|r| r.to_str())
    }
}

trait UnwrapStr<'a> {
    /// Returns value contained as `&str` or `""` if it's `Result::Error` or `Option::None`.
    fn unwrap_as_str(&'a self) -> &'a str;
}

impl<'a, T: ToStr<'a>> UnwrapStr<'a> for Option<T> {
    fn unwrap_as_str(&'a self) -> &'a str {
        match &self {
            Some(s) => s.to_str(),
            None => "",
        }
    }
}

impl<'a, T: ToStr<'a>, E> UnwrapStr<'a> for Result<T, E> {
    fn unwrap_as_str(&'a self) -> &'a str {
        match &self {
            Ok(ok) => ok.to_str(),
            Err(_) => "",
        }
    }
}
