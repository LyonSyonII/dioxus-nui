#![allow(non_snake_case)]

mod global;
mod init;
mod button;
mod header;
mod list;
pub mod class;


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

/// Defines the horizontal alignment for text and elements on certain components.
/// 
/// # Example
/// ```
/// use dioxus_nui::{List, ListItem, Align};
/// List {
///     ListItem {
///         title: "Interesting title",
///         align: Align::Center
///     }
/// }
/// ```
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

// Re-export all elements to avoid having to import all modules.
pub use crate::init::*;
pub use crate::button::*;
pub use crate::list::*;
pub use crate::header::*;
pub use dioxus_nui_macros::{include_css, include_css_safe};

/// Re-export of all the elements with the same name as the [`dioxus`](dioxus::prelude::dioxus_elements) ones.
///
/// Useful if you want to replace all of them without having to change the names.
///
/// All components unique to dioxus-nui (for example [`List`](crate::List)) will remain with the same name.
pub mod prelude {
    pub use crate::init::*;
    pub use crate::button::Button as button;
    pub use crate::list::*;
    pub use crate::header::H1 as h1;
    pub use crate::header::H2 as h2;
    pub use crate::header::H3 as h3;
    pub use crate::header::H4 as h4;
    pub use dioxus_nui_macros::{include_css, include_css_safe};
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
