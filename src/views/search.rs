use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct SearchQueryParams {
    q: Option<String>,
}

async fn fetch_results(query: String) -> String {
    query
}

#[component]
pub fn Search() -> impl IntoView {
    let query = use_query::<SearchQueryParams>();
    let search_query = move || {
        query.with(|query| {
            query
                .as_ref()
                .map(|query| query.q.clone())
                .unwrap_or_default()
        })
    };

    let query = search_query().unwrap_or_default();

    // http://localhost:8080/search?q=Foo
    view! {
        <Show
            when=move || search_query().is_some()
            fallback=|| view! {
            <h1>"Search"
                <br />
                <FormExample query="".to_string() />
            </h1>
            }
        >
            <h1>"Search"
                <br />
                <FormExample query=query.clone()/>
                // {search_query()}
            </h1>
        </Show>

    }
}

#[component]
pub fn FormExample(query: String) -> impl IntoView {
    let search_results = create_resource(move || query.clone(), fetch_results);

    let value = move || {
        search_results
            .get()
            .unwrap_or_else(|| "".to_string())
    };

    view! {
        // NOTE: A little janky, adjusts the search term, but doesn't submit it really..
        <Form method="GET" action="">
            <input 
                type="search" 
                name="q" 
                value={value}
            />
            <input type="submit"/>
        </Form>
        <p>"You searched for: " {value}</p>
        /*
        <Transition fallback=move|| ()>
        </Transition>
        */
    }
}
