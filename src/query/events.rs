//! Events

use crate::{
    error::Error,
    query::{Collection, Element},
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys_query_derive::OnEvent;

/// Event handlers
#[derive(Copy, Clone, Debug, OnEvent)]
pub enum Event {
    Blur,
    Change,
    Click,
    ContextMenu,
    DblClick,
    Focus,
    #[unimplemented]
    FocusIn,
    #[unimplemented]
    FocusOut,
    #[unimplemented]
    Hover,
    KeyDown,
    KeyPress,
    KeyUp,
    Load,
    MouseDown,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseOut,
    MouseOver,
    MouseUp,
    #[unimplemented]
    Ready,
    Resize,
    Scroll,
    Select,
    Submit,
}
