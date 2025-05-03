use leptos::config::LeptosOptions;
use settings::Settings;
pub mod settings;
//use backend::{IdpRequest, IdpResponse};
//use proto::restapt_service;

//use backend::restapi_client;

use axum::extract::FromRef;

#[derive(Debug, Default)]
pub struct RestService;

#[derive(FromRef, Debug, Clone)]
pub struct MyData {
    pub value: usize,
    pub leptos_options: LeptosOptions,
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = SessionConfig::default().with_table_name("sessions_table");

    // create SessionStore and initiate the database tables
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .unwrap();

    // construct a subscriber that prints formatted traces to stdout

    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();

    //let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let settings = Settings::new();

    // Print out our settings
    println!("{settings:?}");

    /* restapi_client::RestapiClient::connect("http://[::1]:8080")
    .await
    .unwrap(); */
    use axum::Router;
    use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
    use grpc_front::app::*;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    //let (s, ss) = signal(2);

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    /* let app_state = MyData {
           value: 42,
           //leptos_options,
       };
    */
    let app = Router::new()
        .layer(SessionLayer::new(session_store))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        //.fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {

    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
