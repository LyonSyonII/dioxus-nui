use dioxus::prelude::*;
use reusable::reuse;
use dioxus_nui_macros::render_component;
use crate::{init::CheckIfUninit, Align, UnwrapStr};


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
    prefix: Option<Element<'a>>,

    /// Suffix of the list item.
    ///
    /// Can be any element.
    ///
    /// It will be positioned at the left side of the item.
    suffix: Option<Element<'a>>,
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