use leptos::*;

mod app;
mod components;
mod endpoints;
mod extension_api;
mod types;
mod utils;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    mount_to_body(|| view! { <App/> })
}