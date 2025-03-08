use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::list::api::get;
use crate::routes::oidc::view::Oidc;
use crate::routes::saml::view::ListMeta;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/grpc_front.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            //<main>
            <NavBar/>
            <h1>"Welcome to Leptos!"</h1>
                <Routes fallback=|| "Page not found..".into_view()>
                    <Route path=StaticSegment("/") view=HomePage/>
                    <Route path=StaticSegment("/metadata") view=ListMeta/>
                    <Route path=StaticSegment("/oidc") view=Oidc/>
                </Routes>


            //</main>
        </Router>
    }
}
// Groups
//TODO route1: metadata: Import from file, Import from api, clean, list, git,
//TODO route2: OIDC, list, detail view, add, resource server, template userinfo
//TODO userinfo

#[component]
pub fn BusyButton() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                get("So much to do!".to_string()).await.unwrap();
            });
        }>
            "Add Todo"
        </button>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
                //<AddTodo/>
                //<ButtonClick/>
            </body>
        </html>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    /* let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    logging::log!("where do I run?"); */

    view! {

    <h1>Home</h1>
    //<BusyButton/>
            //<button on:click=on_click>"Click Me: " {count}{rr}</button>
        }
}

#[component]
fn Metadata() -> impl IntoView {
    // Creates a reactive value to update the button
    //logging::log!("where do I run?");
    /* let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    view! {
        <button on:click=on_click>"Click Me: " {count}{rr}</button>
    } */

    view! {
    <h1>Home</h1>
    //<BusyButton/>
            //<button on:click=on_click>"Click Me: " {count}{rr}</button>
        }
}

/* #[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    todo!()
} */

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li>
                    <a href="/">Homepage</a>
                </li>
                <li>
                    <a href="/metadata">Metadata</a>
                </li>
                <li>
                    <a href="/oidc">OIDC</a>
                </li>
            </ul>
        </nav>
    }
}

/* #[component]
fn AddTodo() -> impl IntoView {
    let add_todo = ServerAction::<AddTodo>::new();
    // holds the latest *returned* value from the server
    let value = add_todo.value();
    // check if the server has returned an error
    let _has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <ActionForm action=add_todo>
            <label>
                "Add a Todo"
                // `title` matches the `title` argument to `add_todo`
                <input type="text" name="title"/>
            </label>
            <input type="submit" value="Add"/>
        </ActionForm>
    }
} */

#[component]
fn ButtonClick() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            // on:click will run whenever the `click` event fires
            // every event handler is defined as `on:{eventname}`

            // we're able to move `set_count` into the closure
            // because signals are Copy and 'static

            on:click=move |_| *set_count.write() += 1
        >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal Rust string
            "Click me: "
            {count}
        </button>
        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            // you can use signals directly in the view, as a shorthand
            // for a function that just wraps the getter
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            // NOTE: if you just write {count.get()}, this will *not* be reactive
            // it simply gets the value of count once
            {count.get()}
        </p>
    }
}
