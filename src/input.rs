use crate::{ToStr, class};

use std::fmt::Display;

use dioxus::prelude::*;
use dioxus_nui_macros::render_component;
use reusable::reuse;

#[reuse(global_events, global_attributes)]
#[derive(Props)]
pub struct InputProps<'a> {
    #[props(default)]
    input_type: InputType,
    on_change: EventHandler<'a, String>,
    value: Option<&'a str>,
    name: Option<&'a str>,
    label: Option<&'a str>,
}

pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element<'a> {
    let InputProps {
        name,
        input_type,
        on_change,
        value,
        label,
        ..
    } = cx.props;

    render_component! {
        label {
            class: "{class::label}",
            *label,
            input {
                $CLASS: "{input_type.to_style()}",
                oninput: move |v: Event<FormData>| on_change.call(v.value.clone()),
                name: *name,
                r#type: "{input_type}",
                value: *value,
                $GLOBALS,
            }
        }
    }
}

/// `type` attribute of the [`Input`](crate::Input) component.
///
/// For more information see https://www.w3schools.com/tags/tag_input.asp.
#[derive(Default)]
pub enum InputType {
    /// Sets the [`Input`] component type as a button.
    ///
    /// Prefer using the [`Button`](crate::Button) component.
    Button,
    /// Sets the [`Input`] component type as a checkbox.
    ///
    /// Prefer using the [`Checkbox`](crate::Checkbox) component.
    Checkbox,
    /// Sets the [`Input`] component type as a color picker.
    ///
    /// Prefer using the [`Checkbox`](crate::Checkbox) component.
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    #[default]
    Text,
    Time,
    Url,
    Week,
}

impl InputType {
    pub fn to_style(&self) -> &'static str {
        use crate::class;
        match self {
            InputType::Button => constcat::concat!(class::btn, " ", class::btn_compact),
            InputType::Checkbox => class::input_checkbox,
            InputType::Color => class::input_color,
            InputType::Date => class::input_date,
            InputType::DatetimeLocal => class::input_datetimelocal,
            InputType::Email => class::input_email,
            InputType::File => class::input_file,
            InputType::Hidden => class::input_hidden,
            InputType::Image => class::input_image,
            InputType::Month => class::input_month,
            InputType::Number => class::input_number,
            InputType::Password => class::input_password,
            InputType::Radio => class::input_radio,
            InputType::Range => class::input_range,
            InputType::Reset => class::input_reset,
            InputType::Search => class::input_search,
            InputType::Submit => class::input_submit,
            InputType::Tel => class::input_tel,
            InputType::Text => class::input_text,
            InputType::Time => class::input_time,
            InputType::Url => class::input_url,
            InputType::Week => class::input_week,
        }
    }

    #[inline]
    pub const fn to_const_str(&self) -> &'static str {
        match self {
            InputType::Button => "button",
            InputType::Checkbox => "checkbox",
            InputType::Color => "color",
            InputType::Date => "date",
            InputType::DatetimeLocal => "datetime-local",
            InputType::Email => "email",
            InputType::File => "file",
            InputType::Hidden => "hidden",
            InputType::Image => "image",
            InputType::Month => "month",
            InputType::Number => "number",
            InputType::Password => "password",
            InputType::Radio => "radio",
            InputType::Range => "range",
            InputType::Reset => "reset",
            InputType::Search => "search",
            InputType::Submit => "submit",
            InputType::Tel => "tel",
            InputType::Text => "text",
            InputType::Time => "time",
            InputType::Url => "url",
            InputType::Week => "week",
        }
    }
}

impl ToStr<'static> for InputType {
    fn to_str(&self) -> &'static str {
        self.to_const_str()
    }
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
