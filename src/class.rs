#![allow(non_upper_case_globals)]

//! Classes used to style the NUI components.
//! 
//! This module exposes the ones that can be used as a standalone class,
//! which can be useful if you want to achieve a particular look that's not available through properties.
//! 
//! # Example
//! ```
//! use dioxus_nui::{List, ListItem, Align};
//! use dioxus_nui::class;
//! List {
//!     // Gives `title` the same styling as a `H1`
//!     ListItem {
//!         class: "{class::h1}",
//!         title: "Interesting title",
//!         align: Align::Center
//!     }
//! }
//! ```

/// Styles element as a NUI `Button`.
/// 
/// Prefer using the [`dioxus_nui::Button`](crate::Button) component directly.
pub const btn: &str = "nui-btn";
/// Styles element as a NUI
pub const btn_regular: &str = "nui-btn--regular";

/// Styles text as a NUI `H1`.
pub const h1: &str = "nui-h1";
/// Styles text as a NUI `H2`.
pub const h2: &str = "nui-h2";
/// Styles text as a NUI `H3`.
pub const h3: &str = "nui-h3";
/// Styles text as a NUI `H4`.
pub const h4: &str = "nui-h4";

/// Styles elements to the current's theme accent color, effect depends on the element.
/// 
/// Text (h1, p, span, etc) is styled by applying a `color` property.
/// 
/// Other elements are styled by changing the background color.
/// 
/// Elements that are affected by the `:enabled` selector (like `button` or `input`) will have `hover` and `active` animations.
pub const accent: &str = ".nui-accent";
/// Same as [`dioxus_nui::class::accent`](accent) but only active when hovering the element.
pub const accent_hover: &str = ".nui-accent-hover";

/// Sets the alignment of the element to the left.
/// 
/// Prefer using [`dioxus_nui::Align::Left`](crate::Align).
pub const align_left: &str = ".nui-align-left";
/// Sets the alignment of the element to the left.
/// 
/// Prefer using [`dioxus_nui::Align::Left`](crate::Align).
pub const align_right: &str = ".nui-align-right";
/// Sets the alignment of the element to the right.
/// 
/// Prefer using [`dioxus_nui::Align::Right`](crate::Align).
pub const align_center: &str = ".nui-align-center";