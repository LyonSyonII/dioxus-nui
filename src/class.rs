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
/// Prefer using the [`Button`](crate::Button) component directly.
pub const btn: &str = "nui-btn";
/// Styles element as a NUI `Button` with `ButtonStyle::Regular`.
///
/// Prefer using the [`Button`](crate::Button) component directly.
pub const btn_regular: &str = "nui-btn--regular";
/// Styles element as a NUI `Button` with [`ButtonStyle::Compact`](crate::ButtonStyle).
///
/// Prefer using the [`Button`](crate::Button) component directly.
pub const btn_compact: &str = "nui-btn--compact";
/// Styles element as a NUI `Button` with [`ButtonStyle::Pill`](crate::ButtonStyle).
///
/// Prefer using the [`Button`](crate::Button) component directly.
pub const btn_pill: &str = "nui-btn--pill";
/// Styles element as a NUI `Button` with [`ButtonStyle::Circular`](crate::ButtonStyle).
///
/// Prefer using the [`Button`](crate::Button) component directly.
pub const btn_circular: &str = "nui-btn--circular";

/// Styles text as a NUI `Label`.
pub const label: &str = "nui-label";
/// Styles text as a NUI `H1`.
pub const h1: &str = "nui-h1";
/// Styles text as a NUI `H2`.
pub const h2: &str = "nui-h2";
/// Styles text as a NUI `H3`.
pub const h3: &str = "nui-h3";
/// Styles text as a NUI `H4`.
pub const h4: &str = "nui-h4";

/// Styles <input> element as a checkbox.
///
/// Prefer using the [`Checkbox`](crate::Checkbox) component directly or the [`Input`](crate::Input) component with [`InputType::Checkbox`](crate::InputType).
pub const input_checkbox: &str = "nui-input--checkbox";
/// Styles <input> element as a color picker.
pub const input_color: &str = "nui-input--color";
/// Styles <input> element as a date selector.
pub const input_date: &str = "nui-input--date";
/// Styles <input> element as a time selector.
pub const input_datetimelocal: &str = "nui-input--datetimelocal";
/// Styles <input> element as an email input.
pub const input_email: &str = "nui-input--email";
/// Styles <input> element as a file picker.
pub const input_file: &str = "nui-input--file";
pub const input_hidden: &str = "nui-input--hidden";
pub const input_image: &str = "nui-input--image";
pub const input_month: &str = "nui-input--month";
pub const input_number: &str = "nui-input--number";
pub const input_password: &str = "nui-input--password";
pub const input_radio: &str = "nui-input--radio";
pub const input_range: &str = "nui-input--range";
pub const input_reset: &str = "nui-input--reset";
pub const input_search: &str = "nui-input--search";
pub const input_submit: &str = "nui-input--submit";
pub const input_tel: &str = "nui-input--tel";
pub const input_text: &str = "nui-input--text";
pub const input_time: &str = "nui-input--time";
pub const input_url: &str = "nui-input--url";
pub const input_week: &str = "nui-input--week";

// Not public, as they are very specific.
pub(crate) const list: &str = "nui-list";
pub(crate) const list_item: &str = "nui-list__item";
pub(crate) const list_item_title: &str = "nui-list__item__title";
pub(crate) const list_item_subtitle: &str = "nui-list__item__subtitle";
pub(crate) const list_item_prefix: &str = "nui-list__item__prefix";
pub(crate) const list_item_suffix: &str = "nui-list__item__suffix";

/// Styles elements to the current's theme accent color, effect depends on the element.
///
/// Text (`h1`, `p`, `span`, etc) is styled by applying a `color` property.
///
/// All other elements are styled by changing the background color.
///
/// Elements that are affected by the `:enabled` selector (like `button` or `input`) will have `hover` and `active` animations.
pub const accent: &str = "nui-accent";
/// Same as [`class::accent`](accent) but only active when hovering the element.
pub const accent_hover: &str = "nui-accent-hover";

/// Sets the alignment of the element to the left.
///
/// Prefer using [`Align::Left`](crate::Align).
pub const align_left: &str = "nui-align-left";
/// Sets the alignment of the element to the left.
///
/// Prefer using [`Align::Left`](crate::Align).
pub const align_right: &str = "nui-align-right";
/// Sets the alignment of the element to the right.
///
/// Prefer using [`Align::Right`](crate::Align).
pub const align_center: &str = "nui-align-center";
