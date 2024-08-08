use crate::components::UserChild;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct UserProfileParams {
    id: Option<usize>,
}

#[component]
pub fn Users() -> impl IntoView {
    view! {
        <h1>Users</h1>
        <Outlet />
    }
}

// Child Component of Users
#[component]
pub fn UserProfile() -> impl IntoView {
    let params = use_params::<UserProfileParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

    view! {
        <Show
            when=move || id().is_some()
            fallback=|| view! {<></>}
        >
            <h1>User Profile
                <br />
                 {id()}
            </h1>
            // Example of Reusable Component in a Page View
            <UserChild />
        </Show>
    }
}
