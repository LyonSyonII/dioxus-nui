use crate::Theme;
use dioxus::prelude::*;

/// Initializes NUI styling.
///
/// Must be used before any element imported from this crate.
///
/// If not used, it'll be initialized with a default value (depending on the platform), see [`Theme`](theme) for more information.
#[inline_props]
pub fn InitNui(cx: Scope, theme: Option<Theme>) -> Element {
    INITIALIZED.store(true, std::sync::atomic::Ordering::Release);

    render! {
        style { display: "none", theme.unwrap_or_default().to_style() }
    }
}

/// If `true` NUI has been initialized with a theme.
///
/// Atomic because dioxus can start CheckIfUninit from different Tokio threads
static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Checks if NUI is initialized.
///
/// If not, it returns an [`InitNui`](InitNui) element.
pub(crate) fn CheckIfUninit(cx: Scope) -> Element {
    #[cfg(not(feature = "auto-init"))]
    return None;

    if INITIALIZED.load(std::sync::atomic::Ordering::Acquire) {
        return None;
    }

    #[cfg(debug_assertions)]
    println!("NUI: Initialized NUI");

    render! { InitNui {} }
}
