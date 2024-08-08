use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Navigate() -> impl IntoView {
    let navigate = use_navigate();
    create_effect(move |_| {
        let navigate = navigate.clone();
        spawn_local(async move {
            TimeoutFuture::new(3_000).await;
            navigate("/", Default::default());
        });
    });
    view! {
        <h1>Navigate Page</h1>
    }
}
