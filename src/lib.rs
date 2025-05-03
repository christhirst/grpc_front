pub mod app;
pub mod components;
pub mod db;
pub mod grpc;
pub mod list;
//pub mod settings;
mod oidc;
mod routes;
pub mod settings;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
