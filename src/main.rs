mod app;

use leptos::mount::mount_to_body;

use crate::app::App;

fn main() {
    // Initialize the logger and panic hook
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App);
}