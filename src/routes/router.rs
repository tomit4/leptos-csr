use leptos::*;
use leptos_router::*;

use crate::views::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <nav>
        </nav>
        <main>
            <Router>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/users" view=Users>
                        <Route path=":id" view=UserProfile/>
                        <Route path="" view=|| view!{<></>}/>
                    </Route>
                    <Route path="/search" view=Search/>
                    <Route path="/stylers" view=Stylers/>
                    <Route path="/navigate" view=Navigate/>
                    <Route path="/*any" view=|| view!{<h1>404</h1>}/>
                </Routes>
            </Router>
        </main>
    }
}
