//! `blocky_formats` reads Hytale/Blockbench `.blockymodel` and `.blockyanim` JSON files.
//!
//! The crate intentionally separates:
//!
//! - raw-ish serde structs that mirror the JSON format;
//! - a flattened runtime model that is easier to bind to entities/bones;
//! - simple animation sampling helpers.
//!
//! It does **not** try to be a full Bevy asset loader because Bevy versions move quickly.
//! Enable the `glam` feature to convert vectors/quaternions to the same math types Bevy uses.

mod anim;
mod error;
mod math;
mod model;
mod runtime;
mod sample;

pub use anim::*;
pub use error::*;
pub use math::*;
pub use model::*;
pub use runtime::*;
pub use sample::*;
