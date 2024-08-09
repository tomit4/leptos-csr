use leptos::*;
use routes::*;

mod components;
mod routes;
mod views;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {<App/>}
    })
}
