use leptos::*;
use stylance::*;

import_crate_style!(style, "src/main.module.scss");

#[component]
pub fn Stylers() -> impl IntoView {
    view! {
        <div class=style::stylers>
            <h1 class=style::red id="two">"Hello"</h1>
            <h2 class=style::green >"World"</h2>
            <h2>"Hi There"</h2>
            <h3>"Hello Kanna"</h3>
        </div>
    }
}
