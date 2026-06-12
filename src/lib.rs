pub mod component;
pub mod keymap;
pub mod runtime;
pub mod style;

pub mod prelude {
    pub use crate::{
        component::Component,
        keymap::{
            CommonIntent, Key, KeyBinding, KeyPattern, Keymap, binding, common_keymap,
            text_input_modifiers,
        },
        runtime::spawn_event_reader,
        style::{ColorToken, Design, Palette, Role, Rule, StyleBuilder, style},
    };
}
