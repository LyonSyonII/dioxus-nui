#![allow(non_snake_case)]

use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, rsx, DragEvent, Element, EventHandler,
    FocusEvent, FormEvent, GlobalAttributes as GA, ImageEvent, KeyboardEvent, MediaEvent,
    MouseEvent, Props, Scope, ScrollEvent, SelectionEvent, ToggleEvent, VNode,
};
use dioxus_nui_macros::render_component;
use reusable::{reusable, reuse};

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
            Theme::Adwaita => dioxus_nui_macros::include_css!("styles/adwaita.css"),
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
        style { theme.unwrap_or_default().to_style() }
    }
}

/// Checks if NUI is initialized.
///
/// If not, it returns an [`InitNui`](InitNui) element.
fn CheckIfUninit(cx: Scope) -> Element {
    /// If `true` NUI has been initialized with a theme.
    /// 
    /// Atomic because dioxus can start CheckIfUninit from different Tokio threads
    static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    
    if INITIALIZED.load(std::sync::atomic::Ordering::Acquire) {
        return None;
    }

    INITIALIZED.store(true, std::sync::atomic::Ordering::Release);

    render! {
        InitNui {}
    }
}

// Global Attributes & Events
// Uses the [reusable](https://crates.io/crates/reusable) crate to add the global attributes to the built elements.
#[allow(dead_code)]
#[reusable(global_attributes)]
#[derive(Props, PartialEq)]
/// Struct representing global HTML attributes that can be applied to various HTML elements.
struct GlobalAttributes<'a> {
    /// Specifies a shortcut key to activate or focus an element.
    #[props(into)]
    accesskey: Option<&'a str>,
    /// Controls whether and how text input is automatically capitalized as it is entered/edited by the user.
    #[props(into)]
    autocapitalize: Option<&'a str>,
    /// Indicates whether an element should have input focus when the page loads.
    #[props(into)]
    autofocus: Option<&'a str>,
    /// Specifies one or more class names for an element.
    #[props(into)]
    class: Option<&'a str>,
    /// Indicates whether the content of an element can be edited by the user.
    #[props(into)]
    contenteditable: Option<&'a str>,
    /// Used to store custom data private to the page or application.
    #[props(into)]
    data: Option<&'a str>,
    /// Specifies the text direction for the content in an element.
    #[props(into)]
    dir: Option<&'a str>,
    /// Specifies whether an element is draggable or not.
    #[props(into)]
    draggable: Option<&'a str>,
    /// Provides a hint for the type of user action that is expected when an element receives focus.
    #[props(into)]
    enterkeyhint: Option<&'a str>,
    /// Indicates whether the element is relevant.
    #[props(into)]
    hidden: Option<&'a str>,
    /// Defines a unique identifier for an element.
    #[props(into)]
    id: Option<&'a str>,
    /// Specifies a hint to the browser for which virtual keyboard to display.
    #[props(into)]
    inputmode: Option<&'a str>,
    /// Defines the type of the element.
    #[props(into)]
    is: Option<&'a str>,
    /// Specifies a URL which designates a page which describes the offering.
    #[props(into)]
    itemid: Option<&'a str>,
    /// Associates an element with one or more items within an HTML document.
    #[props(into)]
    itemprop: Option<&'a str>,
    /// Specifies additional items to include in the accessibility tree.
    #[props(into)]
    itemref: Option<&'a str>,
    /// Specifies the URL(s) of the vocabulary that defines the item(s) for an element.
    #[props(into)]
    itemtype: Option<&'a str>,
    /// Specifies the primary language for the element's contents.
    #[props(into)]
    lang: Option<&'a str>,
    /// Provides a mechanism to enable the server to declare a policy for the page to report back to it when a nonce is used.
    #[props(into)]
    nonce: Option<&'a str>,
    /// Assigns a part name to an element, so that it can be used from the element's CSS, JavaScript or other parts of the page.
    #[props(into)]
    part: Option<&'a str>,
    /// Defines a role name for an element.
    #[props(into)]
    role: Option<&'a str>,
    /// Assigns a slot name to an element.
    #[props(into)]
    slot: Option<&'a str>,
    /// Specifies whether a user can edit the content of an element.
    #[props(into)]
    spellcheck: Option<&'a str>,
    /// Specifies an inline CSS style for an element.
    #[props(into)]
    style: Option<&'a str>,
    /// Specifies the tabbing order for an element.
    #[props(into)]
    tabindex: Option<&'a str>,
    /// Defines a title for the element.
    #[props(into)]
    title: Option<&'a str>,
    /// Specifies whether the element's attribute values and the values of its Text node children are to be translated when the page is localized.
    #[props(into)]
    translate: Option<&'a str>,

    // SPECIFIC TO NUI
    /// Sets accent, will work with most elements.
    #[props(default)]
    accent: bool,
}

#[allow(dead_code)]
#[reusable(global_events)]
#[derive(Props)]
/// A struct representing global event handlers that can be used with various HTML elements.
struct GlobalEvents<'a> {
    /// Triggers when a media playback is aborted.
    #[props(default)]
    onabort: EventHandler<'a, MediaEvent>,
    /// Triggers when an element loses focus.
    #[props(default)]
    onblur: EventHandler<'a, FocusEvent>,
    /// Triggers when the value of a form element is changed.
    #[props(default)]
    onchange: EventHandler<'a, FormEvent>,
    /// Triggers when an element is clicked.
    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,
    /// Triggers when the context menu is triggered by right-clicking on an element.
    #[props(default)]
    oncontextmenu: EventHandler<'a, MouseEvent>,
    /// Triggers when an element is double-clicked.
    #[props(default)]
    ondblclick: EventHandler<'a, MouseEvent>,
    /// Triggers when an element is being dragged.
    #[props(default)]
    ondrag: EventHandler<'a, DragEvent>,
    /// Triggers when the dragging of an element is finished.
    #[props(default)]
    ondragend: EventHandler<'a, DragEvent>,
    /// Triggers when a dragged element enters a drop target.
    #[props(default)]
    ondragenter: EventHandler<'a, DragEvent>,
    /// Triggers when a dragged element leaves a drop target.
    #[props(default)]
    ondragleave: EventHandler<'a, DragEvent>,
    /// Triggers when an element is being dragged over a drop target.
    #[props(default)]
    ondragover: EventHandler<'a, DragEvent>,
    /// Triggers when the dragging of an element starts.
    #[props(default)]
    ondragstart: EventHandler<'a, DragEvent>,
    /// Triggers when a dragged element is dropped on a drop target.
    #[props(default)]
    ondrop: EventHandler<'a, DragEvent>,
    /// Triggers when the duration of a media element is changed.
    #[props(default)]
    ondurationchange: EventHandler<'a, MediaEvent>,
    /// Triggers when a media element is emptied (e.g. via the "pause" button).
    #[props(default)]
    onemptied: EventHandler<'a, MediaEvent>,
    /// Triggers when a media element has ended playback.
    #[props(default)]
    onended: EventHandler<'a, MediaEvent>,
    /// Triggers when an error occurs while loading an image.
    #[props(default)]
    onerror: EventHandler<'a, ImageEvent>,
    /// Triggers when an element receives focus.
    #[props(default)]
    onfocus: EventHandler<'a, FocusEvent>,
    /// Triggers when the value of a form element is being changed.
    #[props(default)]
    oninput: EventHandler<'a, FormEvent>,
    /// Triggers when a form element's value is invalid.
    #[props(default)]
    oninvalid: EventHandler<'a, FormEvent>,
    /// Triggers when a key is pressed down while the element is in focus.
    #[props(default)]
    onkeydown: EventHandler<'a, KeyboardEvent>,
    /// Triggers when a key is pressed while the element is in focus.
    #[props(default)]
    onkeypress: EventHandler<'a, KeyboardEvent>,
    /// Triggers when a key is released while the element is in focus.
    #[props(default)]
    onkeyup: EventHandler<'a, KeyboardEvent>,
    /// Triggers when an image is finished loading.
    #[props(default)]
    onload: EventHandler<'a, ImageEvent>,
    /// Triggers when the media data is loaded and ready to be played.
    #[props(default)]
    onloadeddata: EventHandler<'a, MediaEvent>,
    /// Triggers when the metadata (e.g. length) of a media element is loaded.
    #[props(default)]
    onloadedmetadata: EventHandler<'a, MediaEvent>,
    /// Triggers when the loading of a media element starts.
    #[props(default)]
    onloadstart: EventHandler<'a, MediaEvent>,
    /// Triggers when a mouse button is pressed down on an element.
    #[props(default)]
    onmousedown: EventHandler<'a, MouseEvent>,
    /// Triggers when the mouse pointer enters an element.
    #[props(default)]
    onmouseenter: EventHandler<'a, MouseEvent>,
    /// Triggers when the mouse pointer leaves an element.
    #[props(default)]
    onmouseleave: EventHandler<'a, MouseEvent>,
    /// Triggers when the mouse pointer is moving over an element.
    #[props(default)]
    onmousemove: EventHandler<'a, MouseEvent>,
    /// Triggers when the mouse pointer leaves an element, or one of its child elements, and enters another element.
    #[props(default)]
    onmouseout: EventHandler<'a, MouseEvent>,
    /// Triggers when the mouse pointer enters an element, or one of its child elements, from another element.
    #[props(default)]
    onmouseover: EventHandler<'a, MouseEvent>,
    /// Triggers when a mouse button is released on an element.
    #[props(default)]
    onmouseup: EventHandler<'a, MouseEvent>,
    /// Triggers when a media element is paused.
    #[props(default)]
    onpause: EventHandler<'a, MediaEvent>,
    /// Triggers when a media element starts playing.
    #[props(default)]
    onplay: EventHandler<'a, MediaEvent>,
    /// Triggers when a media element has started playing.
    #[props(default)]
    onplaying: EventHandler<'a, MediaEvent>,
    /// Triggers when a media element is progressing through its playback.
    #[props(default)]
    onprogress: EventHandler<'a, MediaEvent>,
    /// Triggers when the playback rate of a media element is changed.
    #[props(default)]
    onratechange: EventHandler<'a, MediaEvent>,
    /// Triggers when a form element is reset.
    #[props(default)]
    onreset: EventHandler<'a, FormEvent>,
    /// Triggers when an element is being scrolled.
    #[props(default)]
    onscroll: EventHandler<'a, ScrollEvent>,
    /// Triggers when a media element has finished seeking to a new position.
    #[props(default)]
    onseeked: EventHandler<'a, MediaEvent>,
    /// Triggers when a media element is seeking a new position.
    #[props(default)]
    onseeking: EventHandler<'a, MediaEvent>,
    /// Triggers when the text within a textarea or text input is selected by the user.
    #[props(default)]
    onselect: EventHandler<'a, SelectionEvent>,
    /// Triggers when a media element is trying to load but eventually stalling.
    #[props(default)]
    onstalled: EventHandler<'a, MediaEvent>,
    /// Triggers when a form is submitted.
    #[props(default)]
    onsubmit: EventHandler<'a, FormEvent>,
    /// Triggers when the loading of a media element is suspended.
    #[props(default)]
    onsuspend: EventHandler<'a, MediaEvent>,
    /// Triggers when the current playback position of a media element is updated.
    #[props(default)]
    ontimeupdate: EventHandler<'a, MediaEvent>,
    /// Triggers when a checkbox element is toggled between on and off states.
    #[props(default)]
    ontoggle: EventHandler<'a, ToggleEvent>,
    /// Triggers when the volume of a media element is changed.
    #[props(default)]
    onvolumechange: EventHandler<'a, MediaEvent>,
    /// Triggers when a media element is waiting for data to load.
    #[props(default)]
    onwaiting: EventHandler<'a, MediaEvent>,
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

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let ButtonProps {
        button_style,
        disabled,
        ..
    } = cx.props;
    

    render_component! {
        CheckIfUninit {},
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
