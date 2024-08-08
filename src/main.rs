use gloo_timers::future::TimeoutFuture;
use leptos::*;
use std::marker::PhantomData;
use std::num::ParseIntError;

/* Advanced Topic: #[component(transparent)]
 * see: https://book.leptos.dev/view/03_components.html#advanced-topic-componenttransparent */

/* Dynamic Rendering with the <For/> Component,
 * see: https://book.leptos.dev/view/04_iteration.html#dynamic-rendering-with-the-for-component */

/* Mutating Data with Actions
 * https://book.leptos.dev/async/13_actions.html */

/* Projecting Children
 * https://book.leptos.dev/interlude_projecting_children.html */

/* Global State
 * https://book.leptos.dev/15_global_state.html */

async fn load_data(value: i32) -> i32 {
    TimeoutFuture::new(1_000).await;
    value
}

async fn load_a(value: i32) -> i32 {
    TimeoutFuture::new(1_000).await;
    value
}

async fn load_b(value: i32) -> i32 {
    TimeoutFuture::new(1_000).await;
    value
}

async fn fetch_monkeys(monkey: i32) -> i32 {
    // maybe this didn't need to be async
    TimeoutFuture::new(2_000).await;
    monkey * 2
}

async fn important_api_call(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User not found",
    }
    .to_string()
}

#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}

#[component]
fn TextArea() -> impl IntoView {
    let (some_value, _set_value) = create_signal("Hello".to_string());
    view! {
        <textarea
            prop:value=move || some_value.get()
            // on:input=/* etc */
        >
            {some_value.get_untracked()}
        </textarea>
    }
}

#[component]
fn Select() -> impl IntoView {
    let (value, set_value) = create_signal(0i32);

    view! {
        <select
            on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_value.set(new_value.parse().unwrap());
            }
            prop:value=move || value.get().to_string()
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
        // a button that will cycle through the options
        <button
            on:click=move |_| set_value.update(|n| {
                if *n == 2 {
                    *n = 0;
                } else {
                    *n += 1;
                }
            })
        >
        "Next Option"
        </button>
    }
}

// Show progress toward a goal
#[component]
fn ProgressBar(
    // The maximum value of the progress bar.
    #[prop(default = 100)] max: u16,
    // How much progress should be displayed
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
        <br />
    }
}

#[component]
fn UncontrolledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit" />
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn ControlledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            on:input=move |ev| {
                set_name.set(event_target_value(&ev));
            }
        prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value): (
        ReadSignal<Result<i32, ParseIntError>>,
        WriteSignal<Result<i32, ParseIntError>>,
    ) = create_signal(Ok(0));

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=move |ev|
                set_value.set(event_target_value(&ev).parse::<i32>())
            />
            <ErrorBoundary
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered "
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>

        </label>
    }
}

#[component]
fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
            >
            "Toggle A"
        </button>
    }
}
#[component]
fn ButtonC() -> impl IntoView {
    view! {
        <button>"Toggle C"</button>
    }
}

#[component]
fn Layout() -> impl IntoView {
    view! {
        <ButtonD />
    }
}

#[component]
fn ButtonD() -> impl IntoView {
    // use_context searches up the context tree, hoping to
    // find a `WriteSignal<bool>`
    // in this case, I .expect() because I know I provided it
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle D"
        </button>
    }
}

#[component]
fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}
        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
fn HasChildren() -> impl IntoView {
    view! {
        <TakesChildren render_prop=|| view! { <p>"Hi there!"</p> }>
        "Some text"
        <span>"A span"</span>
        </TakesChildren>
    }
}

#[component]
fn WrapsChildren(children: Children) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();

    view! {
        <ul>{children}</ul>
    }
}

#[component]
fn NameSetter() -> impl IntoView {
    let (names, set_names) = create_signal(Vec::<String>::new());
    if names.with(Vec::is_empty) {
        logging::log!("names vec is empty");
    }

    let add_name = move |_| {
        let mut current_names = names.get().clone();
        current_names.push("New Name".to_string());
        set_names.set(current_names);
        // Efficient way of checking if Vector is empty
        if let Some(first_name) = names.get().first() {
            logging::log!("First name: {}", first_name);
        }
    };
    view! {
        <button
            on:click=add_name
            >
            "Add Name"
        </button>

    }
}

#[component]
fn CreateEffect() -> impl IntoView {
    let (a, _set_a) = create_signal(1);
    let (b, set_b) = create_signal(0);

    create_effect(move |_| {
        logging::debug_warn!("Value Of a From CreateEffect: {}", a.get()); // 1
        set_b.set(a.get() * 2);
        logging::debug_warn!("Value Of b From CreateEffect: {}", b.get()); // 2
    });
    let b = move || a.get() * 2;
    logging::log!("Value of b outside of create_effect: {}", b()); // 2
}

#[component]
fn Watch() -> impl IntoView {
    let (num, set_num) = create_signal(0);

    let stop = watch(
        move || num.get(),
        move |num, prev_num, _| {
            logging::debug_warn!("Number: {}; Prev: {:?}", num, prev_num);
        },
        false,
    );

    set_num.set(1); // Number: 1; Prev: Some(0)

    stop(); // stop watching

    set_num.set(2); // nothing happens
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(1);
    let double_count = move || count.get() * 2;

    let (count1, _set_count1) = create_signal(2);
    let (count2, _set_count2) = create_signal(3);

    let (tab, set_tab) = create_signal(0);

    // this will reload every time `tab` changes
    let user_data = create_resource(move || tab.get(), |tab| async move { important_api_call(tab).await });

    let a =create_resource(
        move || count1.get(),
        |count| async move {load_a(count).await}
    );
    let b =create_resource(
        move || count2.get(),
        |count| async move {load_b(count).await}
    );



    let async_data = create_resource(
        move || count.get(),
        |value| async move { load_data(value).await },
    );
    logging::log!("async_data :=> {:?}", async_data);

    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            .unwrap_or_else(|| "Loading...".into())
    };
    logging::log!("async_result :=> {:?}", async_result());

    let loading = async_data.loading();
    logging::log!("loading :=> {:?}", loading.get());
    let is_loading = move || if loading.get() { "Loading..." } else { "Idle." };

    let values = vec![0, 1, 2];

    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click={
                            move |_| set_count.update(|n| *n += 1)
                        }
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    let (toggled, set_toggled) = create_signal(false);
    provide_context(set_toggled);

    view! {
        <div class="buttons">
            <button
                on:click=move |_| set_tab.set(0)
                class:selected=move || tab.get() == 0
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab.set(1)
                class:selected=move || tab.get() == 1
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab.set(2)
                class:selected=move || tab.get() == 2
            >
                "Tab C"
            </button>
            {move || if user_data.loading().get() {
                "Loading..."
            } else {
                ""
            }}
        </div>
        <Transition
            // the fallback will show initially
            // on subsequent reloads, the current child will
            // continue showing
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <p>
                {move || user_data.get()}
            </p>
        </Transition>
        <h1>"My Data"</h1>
        <Suspense
            fallback=move || view! { <p>"Loading My Data..."</p> }
        >
            <h3>"A"</h3>
            {move || {
                 a.get().map(|a| view! { <p>"showing a" {a}</p>})
             }}
            <h3>"B"</h3>
            {move || {
                 b.get().map(|b| view! { <p>"showing b" {b}</p>})
             }}
        </Suspense>
        <Await
            future=|| fetch_monkeys(3)
            let:data
        >
            <p>{*data} " little monkeys, jumping on the bed."</p>
        </Await>
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
                // Equivalent to console.log()
                logging::log!("{}", count.get());
            }
            class:red=move | | {
                count.get() % 2 == 1
            }
        >
            "Click me: "
        </button>
        <br />
        <ProgressBar progress=count />
        <ProgressBar progress=double_count />
        <p>
            {move || count.get()}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading}
        </p>
        <SizeOf<usize>/>
        <SizeOf<String>/>
        <p>{values.clone()}</p> // displays 012
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li> })
                .collect_view()
            }
        </ul>
        <ul>
            {counter_buttons}
        </ul>
        <ControlledComponent />
        <UncontrolledComponent />
        <TextArea />
        <br />
        <Select />
        <br />
        <Show
            when=move || { count.get() > 5 }
            fallback=|| view! {<p>Small</p>}
        >
        <p>Big</p>
        </Show>
        <NumericInput />
        <br />
        <p>"Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled/>
        <ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value)/>
        <Layout />
        <br />
        <HasChildren />
        <WrapsChildren>
            "A"
            "B"
            "C"
        </WrapsChildren>
        <br />
        <NameSetter />
        <br />
        <CreateEffect />
        <br />
        <Watch />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
