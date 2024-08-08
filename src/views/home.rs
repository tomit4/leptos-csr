use leptos::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>Home</h1>
        <A href="/users/1">Link</A>
        <br />
        <A href="/search?q=Foo">Link 2</A>
        <br />
        <a rel="external" href="https://example.com">External Link</a>
    }
}
