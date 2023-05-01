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
///
/// Equivalent to the following CSS:
/// ```css
/// .nui-btn {
///    /* Reset default properties */
///    text-transform: none;
///    font-family: inherit;
///    font-size: 100%;
///    font-weight: inherit;
///    line-height: inherit;
///    color: inherit;
///    margin: 0;
///    padding: 0;
///    
///    /* Real properties */
///    background-color: var(--element-color);
///    font-weight: 700;
///    color: var(--text-color);
///    transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, -webkit-text-decoration-color;
///    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
///    transition-duration: 200ms;
///  }
/// ```
pub const btn: &str = "nui-btn";

/// Styles text as a NUI `H1`.
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-h1 {
///   margin: 0;
///   font-size: 1.875rem;
///   font-weight: 1000;
///   line-height: 2.5rem;
///   color: var(--text-color);
/// }
/// ```
pub const h1: &str = "nui-h1";
/// Styles text as a NUI `H2`.
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-h2 {
///     margin: 0;
///     font-size: 1.5rem;
///     font-weight: 800;
///     line-height: 2rem;
///     color: var(--text-color);
///   }
/// ```
pub const h2: &str = "nui-h2";
/// Styles text as a NUI `H3`.
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-h3 {
///     margin: 0;
///     font-size: 1.5rem;
///     font-weight: 500;
///     line-height: 2rem;
///     color: var(--text-color);
///   }
/// ```
pub const h3: &str = "nui-h3";
/// Styles text as a NUI `H4`.
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-h4 {
///     margin: 0;
///     font-size: 1.25rem;
///     font-weight: 500;
///     line-height: 2rem;
///     color: var(--text-color);
///   }
/// ```
pub const h4: &str = "nui-h4";

/// Styles elements to the accent color, effect depends on the element.
/// 
/// Text (h1, p, span, etc) is styled by applying a `color` property.
/// 
/// Other elements are styled by changing the background color.
/// 
/// Elements that are affected by the `:enabled` selector (like `button` or `input`) will have `hover` and `active` animations.
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-accent {
///     background-color: var(--accent-color);
///   }
///   
/// .nui-accent:hover:enabled {
///   background-color: var(--accent-hover-color);
/// }
/// 
/// .nui-accent:active:enabled {
///   background-color: var(--accent-active-color);
/// }
/// 
/// h1.nui-accent,
/// h2.nui-accent,
/// h3.nui-accent,
/// h4.nui-accent,
/// p.nui-accent,
/// span.nui-accent {
///   /* To avoid declaring all other elements */
///   background-color: unset;
///   color: var(--accent-color);
/// }
/// ```
/// 
pub const accent: &str = ".nui-accent";
/// Same as [`dioxus_nui::class::accent`](accent) but only active when hovering the element.
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-accent-hover {
///   transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, -webkit-text-decoration-color;
///   transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
///   transition-duration: 200ms;
/// }
/// 
/// .nui-accent-hover:hover {
///   background-color: var(--accent-color);
/// }
/// 
/// h1.nui-accent-hover:hover,
/// h2.nui-accent-hover:hover,
/// h3.nui-accent-hover:hover,
/// h4.nui-accent-hover:hover,
/// p.nui-accent-hover:hover,
/// span.nui-accent-hover:hover {
///   background-color: unset;
///   color: var(--accent-color);
/// }
/// ```
pub const accent_hover: &str = ".nui-accent-hover";

/// Sets the alignment of the element to the left.
/// 
/// Prefer using [`dioxus_nui::Align::Left`](crate::Align).
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-align-left {
///   margin-right: auto;
/// }
/// ```
pub const align_left: &str = ".nui-align-left";
/// Sets the alignment of the element to the left.
/// 
/// Prefer using [`dioxus_nui::Align::Left`](crate::Align).
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-align-left {
///   margin-right: auto;
/// }
/// ```
pub const align_right: &str = ".nui-align-right";
/// Sets the alignment of the element to the right.
/// 
/// Prefer using [`dioxus_nui::Align::Right`](crate::Align).
/// 
/// Equivalent to the following CSS:
/// ```css
/// .nui-align-right {
///   margin-left: auto;
/// }
/// ```
pub const align_center: &str = ".nui-align-center";