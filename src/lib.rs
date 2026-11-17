//! Minimal repro: on macOS, a single-line input's text moves sideways when
//! the input takes the focus, by an amount that depends on which input was
//! focused before.
//!
//! The window has a long input (text wider than the field) twice, and a
//! short one. Compare the focused long input's text with the unfocused one
//! below it: both should start at the same x.
//!
//! - `cargo run`: the first long input is focused at launch; its text starts
//!   5pt left of the unfocused copy.
//! - `REPRO=switch cargo run`: every 1.5s the focus moves between the short
//!   input and the first long one; focused after the short one, the long
//!   input's text starts 2pt left of the unfocused copy.

use std::{cell::Cell, time::Duration};

use nsh_base::theme;
use nsh_core::element::{ElementRef, Input, element_ref, input, set_interval};
use nsh_core::prelude::*;

#[native_module("input-repro/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
}

const LONG: &str = "Close half (average +5%, follows the average)";

fn field(value: &'static str, focus: bool, r: &ElementRef) -> Input {
    input()
        .element_ref(r)
        .value(value)
        .auto_focus(focus)
        .h(px(28.0))
        .px(px(10.0))
        .text_sm()
        .rounded(px(6.0))
        .border(px(1.0))
        .border_color(move || theme().border)
}

#[component]
pub fn App() -> impl IntoElement {
    let switch = std::env::var("REPRO").is_ok_and(|v| v == "switch");
    let (long, copy, short) = (element_ref(), element_ref(), element_ref());
    if switch {
        let (long, short) = (long.clone(), short.clone());
        let n = Cell::new(0u32);
        let _ = set_interval(Duration::from_millis(1500), move || {
            let k = n.get();
            n.set(k + 1);
            if k % 2 == 0 { short.focus() } else { long.focus() }
        });
    }
    view()
        .flex_1()
        .p(px(24.0))
        .gap(px(12.0))
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(text("Long text: focused (or not) vs never focused"))
        .child(view().w(px(260.0)).child(field(LONG, !switch, &long)))
        .child(view().w(px(260.0)).child(field(LONG, false, &copy)))
        .child(text("Short text"))
        .child(view().w(px(140.0)).child(field("short", false, &short)))
}
