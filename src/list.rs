use crate::{class, init::CheckIfUninit, Align, UnwrapStr};
use dioxus::{prelude::*, core::DynamicNode};
use dioxus_nui_macros::render_component;
use reusable::reuse;

#[reuse(global_attributes, global_events)]
#[derive(Props)]
pub struct ListProps<'a> {
    children: Element<'a>,
}

/// Creates a List of items.
///
/// To work properly, use the [`ListItem`](crate::ListItem) component.
///
/// If another element is used, it will be wrapped in a `ListItem`.
pub fn List<'a>(cx: Scope<'a, ListProps<'a>>) -> Element {
    let children = cx.props.children.clone();
    println!("{children:#?}");


    render_component! {
        div {
            $CLASS: "{class::list}",
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
    children: Element<'a>,
}

/// Creates a new item for a [`List`](crate::List).
///
/// Use only nested in a `List` element.
/// 
/// Any `children` of the component will be positioned as a `suffix` element.
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
            p { class: "{class::list_item_title} {title_class.unwrap_as_str()}", t }
        }
    });

    let subtitle = subtitle.map(|t| {
        rsx! {
            p { class: "{class::list_item_subtitle} {subtitle_class.unwrap_as_str()}", t }
        }
    });

    let prefix = prefix.as_ref().map(|p| {
        rsx! {
            div { class: "{class::list_item_prefix}", p }
        }
    });

    let suffix = suffix.as_ref().map(|s| {
        rsx! {
            div { class: "{class::list_item_suffix}", s }
        }
    });

    render_component! {
        div {
            $CLASS: "{class::list_item}",
            $GLOBALS,
            prefix,
            div { class: "{align}",
                title,
                subtitle,
            },
            suffix,
            $CHILDREN
        }
    }
}
