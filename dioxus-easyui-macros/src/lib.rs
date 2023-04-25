use proc_macro::TokenStream;

/// Renders component calling [`dioxus::prelude::render!`](dioxus::prelude::render) and adds [Global Attributes and Global Events](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes) to it.
///
/// # Example:
/// ```
/// render! {
///     button {
///         // Specific attributes
///         disabled: "{disabled}",
///         $GLOBALS,
///         "Button"
///     }
/// }
/// ```
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    // Add globals to Component
    let input = input.to_string().replace(
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

    // panic!("{input}");

    format!("dioxus::prelude::render! {{ {input} }}")
        .parse()
        .unwrap()
}
