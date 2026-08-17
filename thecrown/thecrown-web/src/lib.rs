#![forbid(unsafe_code)]

pub mod app;

#[cfg(feature = "server")]
pub mod assets;
#[cfg(feature = "server")]
pub mod state;

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
