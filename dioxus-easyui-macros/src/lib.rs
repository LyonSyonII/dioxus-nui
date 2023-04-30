use css_minify::optimizations::Level;
use proc_macro::TokenStream;

/// Renders component calling [`dioxus::prelude::render!`](dioxus::prelude::render) and adds all [Global Attributes and Global Events](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes) to it.
/// 
/// If `$CLASS` is specified, the "class" attribute will be automatically appended.
/// 
/// If `$CHILDREN` is specified, `children` will be added to the end of the element.
/// # Example:
/// ```
/// render_component! {
///     button { 
///         // Same as 'class: "custom-class {class}"'
///         $CLASS: "custom-class",
///         // Specific attributes
///         disabled: "true",
///         // Globals Substitution
///         $GLOBALS,
///         // Children
///         "Button",
///         // Same as 'cx.props.children'
///         $CHILDREN
///     }
/// }
/// ```
/// 
/// # $CLASS
/// Will append automatically to the class of the element:
/// - User-specified `class` attribute
/// - `accent` attribute
/// 
/// `$CLASS` *must* be followed by a trailing comma.
#[proc_macro]
pub fn render_component(input: TokenStream) -> TokenStream {
    // Add globals to Component
    let mut input = input.to_string().replace(
        "$GLOBALS",
        r#"
        accesskey: cx.props.accesskey,
        autocapitalize: cx.props.autocapitalize,
        autofocus: cx.props.autofocus,
        contenteditable: cx.props.contenteditable,
        data: cx.props.data,
        dir: cx.props.dir,
        draggable: cx.props.draggable,
        enterkeyhint: cx.props.enterkeyhint,
        hidden: cx.props.hidden,
        id: cx.props.id,
        inputmode: cx.props.inputmode,
        is: cx.props.is,
        itemid: cx.props.itemid,
        itemprop: cx.props.itemprop,
        itemref: cx.props.itemref,
        itemtype: cx.props.itemtype,
        lang: cx.props.lang,
        nonce: cx.props.nonce,
        part: cx.props.part,
        role: cx.props.role,
        slot: cx.props.slot,
        spellcheck: cx.props.spellcheck,
        style: cx.props.style,
        tabindex: cx.props.tabindex,
        title: cx.props.title,
        translate: cx.props.translate,
        
        onabort: move |e| cx.props.onabort.call(e),
        onblur: move |e| cx.props.onblur.call(e),
        onchange: move |e| cx.props.onchange.call(e),
        onclick: move |e| cx.props.onclick.call(e),
        oncontextmenu: move |e| cx.props.oncontextmenu.call(e),
        ondblclick: move |e| cx.props.ondblclick.call(e),
        ondrag: move |e| cx.props.ondrag.call(e),
        ondragend: move |e| cx.props.ondragend.call(e),
        ondragenter: move |e| cx.props.ondragenter.call(e),
        ondragleave: move |e| cx.props.ondragleave.call(e),
        ondragover: move |e| cx.props.ondragover.call(e),
        ondragstart: move |e| cx.props.ondragstart.call(e),
        ondrop: move |e| cx.props.ondrop.call(e),
        ondurationchange: move |e| cx.props.ondurationchange.call(e),
        onemptied: move |e| cx.props.onemptied.call(e),
        onended: move |e| cx.props.onended.call(e),
        onerror: move |e| cx.props.onerror.call(e),
        onfocus: move |e| cx.props.onfocus.call(e),
        oninput: move |e| cx.props.oninput.call(e),
        oninvalid: move |e| cx.props.oninvalid.call(e),
        onkeydown: move |e| cx.props.onkeydown.call(e),
        onkeypress: move |e| cx.props.onkeypress.call(e),
        onkeyup: move |e| cx.props.onkeyup.call(e),
        onload: move |e| cx.props.onload.call(e),
        onloadeddata: move |e| cx.props.onloadeddata.call(e),
        onloadedmetadata: move |e| cx.props.onloadedmetadata.call(e),
        onloadstart: move |e| cx.props.onloadstart.call(e),
        onmousedown: move |e| cx.props.onmousedown.call(e),
        onmouseenter: move |e| cx.props.onmouseenter.call(e),
        onmouseleave: move |e| cx.props.onmouseleave.call(e),
        onmousemove: move |e| cx.props.onmousemove.call(e),
        onmouseout: move |e| cx.props.onmouseout.call(e),
        onmouseover: move |e| cx.props.onmouseover.call(e),
        onmouseup: move |e| cx.props.onmouseup.call(e),
        onpause: move |e| cx.props.onpause.call(e),
        onplay: move |e| cx.props.onplay.call(e),
        onplaying: move |e| cx.props.onplaying.call(e),
        onprogress: move |e| cx.props.onprogress.call(e),
        onratechange: move |e| cx.props.onratechange.call(e),
        onreset: move |e| cx.props.onreset.call(e),
        onscroll: move |e| cx.props.onscroll.call(e),
        onseeked: move |e| cx.props.onseeked.call(e),
        onseeking: move |e| cx.props.onseeking.call(e),
        onselect: move |e| cx.props.onselect.call(e),
        onstalled: move |e| cx.props.onstalled.call(e),
        onsubmit: move |e| cx.props.onsubmit.call(e),
        onsuspend: move |e| cx.props.onsuspend.call(e),
        ontimeupdate: move |e| cx.props.ontimeupdate.call(e),
        ontoggle: move |e| cx.props.ontoggle.call(e),
        onvolumechange: move |e| cx.props.onvolumechange.call(e),
        onwaiting: move |e| cx.props.onwaiting.call(e)"#
    );
    
    replace_single(&mut input, "$CHILDREN", "&cx.props.children");
    
    let class_token = "$CLASS";
    if let Some(i) = input.find(class_token) {
        let error: &str = "Expected $CLASS: \"...\",";

        input.replace_range(i..i+class_token.len(), "class");
        // Get start of "
        let start = input.get(i..).and_then(|input| input.find('"').map(|s| s + i + 1)).expect(error);
        // Get end of "
        let end = input.get(start..).and_then(|input| input.find(',').map(|e| e + start - 2)).expect(error);
        let class = input.get(start..=end).unwrap();
        
        let props_accent = "{cx.props.accent.then(|| \\\"accent\\\").unwrap_or_default()}";
        let props_class = "{cx.props.class.unwrap_or_default()}";

        input.replace_range(start..end+1, &format!("{class} {props_accent} {props_class}"));
    }

    // panic!("{input}");

    format!("dioxus::prelude::render! {{ {input} }}")
        .parse()
        .unwrap()
}

/// Same as [`include_str!`](https://doc.rust-lang.org/std/macro.include_str.html) but minifies the included CSS.
/// 
/// The path used is the root of the crate (not the file like `include_str!`).
/// 
/// Uses the [css-minify](https://crates.io/crates/css-minify) crate on Level 3.
/// 
/// If the CSS does not work properly, use the [`include_css_safe`](include_css_safe) macro.
/// 
/// # Example
/// ```
/// render! {
///     // File is located in $crate_root/styles/index.css
///     style { include_css!("styles/index.css") }
/// }
/// ```
#[proc_macro]
pub fn include_css(input: TokenStream) -> TokenStream {
    let path = {
        let input = input.to_string().replace('"', " ");
        std::path::PathBuf::from(input.trim())
    };
    let input = std::fs::read_to_string(path).unwrap();
    let out = css_minify::optimizations::Minifier::default().minify(&input, Level::Three).unwrap();
    
    format!("{out:?}").parse().unwrap()
}

/// Same as [`include_str!`](https://doc.rust-lang.org/std/macro.include_str.html) but minifies the included CSS.
/// 
/// The path used is the root of the crate (not the file like `include_str!`).
/// 
/// Uses the [css-minify](https://crates.io/crates/css-minify) crate on Level 1.
/// 
/// Use if the CSS did not work properly with the [`include_css`](include_css) macro.
/// 
/// # Example
/// ```
/// render! {
///     // File is located in $crate_root/styles/index.css
///     style { include_css!("styles/index.css") }
/// }
/// ```
#[proc_macro]
pub fn include_css_safe(input: TokenStream) -> TokenStream {
    let path = {
        let input = input.to_string().replace('"', " ");
        std::path::PathBuf::from(input.trim())
    };
    let input = std::fs::read_to_string(path).unwrap();
    let out = css_minify::optimizations::Minifier::default().minify(&input, Level::One).unwrap();
    
    format!("{out:?}").parse().unwrap()
}

/// Replaces the first match with another string.
/// 
/// Returns Some if it was replaced, and None if not.
fn replace_single(string: &mut String, from: &str, to: &str) -> Option<()> {
    let start = string.find(from)?;
    let end = start + from.len();
    string.replace_range(start..end, to);

    Some(())
}