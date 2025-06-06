pub mod app;
pub mod components;
pub mod features;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

pub fn is_dev() -> bool {
    !is_prod()
}

pub fn is_prod() -> bool {
    use std::env;
    let is_prod = env::var("IS_PROD").unwrap_or_else(|_| "0".into());
    is_prod == "1"
}
