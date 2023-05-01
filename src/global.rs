use dioxus::prelude::*;
use reusable::reusable;

// Global Attributes & Events
// Uses the [reusable](https://crates.io/crates/reusable) crate to add the global attributes to the built elements.
#[allow(dead_code)]
#[reusable(global_attributes)]
#[derive(Props, PartialEq)]
/// Struct representing global HTML attributes that can be applied to various HTML elements.
pub(crate) struct GlobalAttributes<'a> {
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
pub(crate) struct GlobalEvents<'a> {
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
