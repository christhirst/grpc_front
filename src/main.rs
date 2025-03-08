//use backend::{IdpRequest, IdpResponse};
//use proto::restapt_service;

//use backend::restapi_client;

/* pub mod backend {
    tonic::include_proto!("backend");
} */

//tonic::include_proto!("backend");

#[derive(Debug, Default)]
pub struct RestService;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    /* restapi_client::RestapiClient::connect("http://[::1]:8080")
    .await
    .unwrap(); */
    use axum::Router;
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

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
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
