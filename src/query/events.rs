//! Events

use super::*;
use wasm_bindgen::closure::Closure;
use web_sys_query_derive::OnEvent;

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
