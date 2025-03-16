use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Form, Route, Router, Routes},
    StaticSegment,
};

use server_fn::{
    client::{browser::BrowserClient, Client},
    codec::{
        Encoding, FromReq, FromRes, GetUrl, IntoReq, IntoRes, MultipartData, MultipartFormData,
        Postcard, Rkyv, SerdeLite, StreamingText, TextStream,
    },
    request::{browser::BrowserRequest, ClientReq, Req},
    response::{browser::BrowserResponse, ClientRes, Res},
};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::routes::saml::view::ListMeta;
use crate::{grpc::init::grpc_connector, list::api::get};
use crate::{grpc::init::GrpcRequest, routes::oidc::view::Oidc};

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
                    <Route path=StaticSegment("/rs") view=Oidc/>
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

#[component]
pub fn Upload() -> impl IntoView {
    view! {
        <html lang="en">
        <body>

    </body>
    </html>
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

#[component]
pub fn FormExample() -> impl IntoView {
    println!("{:?}", "data1");
    // reactive access to URL query strings
    //let query = use_query_map();
    // search stored as ?q=
    //let search = move || query.read().get("q").unwrap_or_default();
    // a resource driven by the search string
    //let search_results = Resource::new(search, |_| fetch_results());
    #[server(
        input = MultipartFormData,
    )]
    pub async fn file_length(data: MultipartData) -> Result<usize, ServerFnError> {
        let mut data = data.into_inner().unwrap();
        println!("{:?}", data);
        // this will just measure the total number of bytes uploaded
        let mut count = 0;
        let mut file_data = Vec::new();

        while let Ok(Some(mut field)) = data.next_field().await {
            println!("\n[NEXT FIELD]\n");
            let name = field.name().unwrap_or_default().to_string();
            println!("  [NAME] {name}");
            while let Ok(Some(chunk)) = field.chunk().await {
                file_data.extend_from_slice(&chunk);
                let len = chunk.len();
                count += len;
                println!("      [CHUNK] {len}");

                // in a real server function, you'd do something like saving the file here
            }
        }

        /* let string = String::from_utf8_lossy(&file_data);
        println!("{}", string); */

        grpc_connector(GrpcRequest::Create, file_data);

        Ok(count)
    }

    let upload_action = Action::new_local(|data: &FormData| {
        println!("{:?}", data);
        // `MultipartData` implements `From<FormData>`
        file_length(data.clone().into())
    });

    view! {
        <h3>File Upload</h3>
        <p>Uploading files is fairly easy using multipart form data.</p>
        <form on:submit=move |ev: SubmitEvent| {
            println!("{:?}", "data3");
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch_local(form_data);
        }>
            <input type="file" name="file_to_upload"/>
            <input type="submit"/>
        </form>

        <p>
        {move || {
            if upload_action.input_local().read().is_none() && upload_action.value().read().is_none()
            {
                "Upload a file.".to_string()
            } else if upload_action.pending().get() {
                "Uploading...".to_string()
            } else if let Some(Ok(value)) = upload_action.value().get() {
                value.to_string()
            } else {
                format!("{:?}", upload_action.value().get())
            }
        }}

    </p>





        /* <Form method="POST" action="">
            <input type="file" name="fileupload" required/>
        </Form> */

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
    <FormExample/>
    //<Upload/>
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
                <li>
                    <a href="/rs">ResourceServer</a>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn ResourceServer() -> impl IntoView {
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
