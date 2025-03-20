use leptos::prelude::*;
use reactive_stores::Store;

use crate::app::BusyButton;
use crate::grpc::init::grpc_connector;
use crate::grpc::init::Data;
use crate::grpc::init::DataStoreFields;
use crate::grpc::init::GrpcRequest;
//use crate::grpc::init::Root;
//use crate::grpc::init::RootStoreFields;
use crate::grpc::types_oidc::OidcClient;
use crate::grpc::types_oidc::OidcClientStoreFields;
use crate::grpc::types_oidc::{ComponentClientType, ComponentRedirectURI, ComponentUsePKCE};

/* fn load(async_data: Resource<Result<Vec<Root>>>) -> Vec<Root> {
    let u = vec![Root {
        user_id: 0,
        id: 0,
        title: String::from(""),
        completed: false,
    }];

    let pp = async_data
        .get()
        .unwrap_or(Ok(u.clone()))
        .unwrap_or(u.clone())
        .clone();
    pp
} */

#[component]
pub fn Oidc() -> impl IntoView {
    let _rq = GrpcRequest::List;
    let (_count, _set_count) = signal(0);
    let (t, _tt) = signal(0);

    let async_data = Resource::new(
        move || t.get(),
        // every time `count` changes, this will run
        |_t| grpc_connector(GrpcRequest::List, vec![1]),
    );

    let tt = vec![OidcClient {
        id: String::from("11"),
        ..Default::default()
    }];

    let t1 = vec![OidcClient {
        id: String::from("222"),
        ..Default::default()
    }];

    //let pp = async_data.get().unwrap_or(Ok(u.clone())).unwrap_or(u);

    /*  let async_result = move || {
        async_data
            .get()
            .or_else(|| Some(Ok(String::from("value"))))
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    }; */

    //let oo = get(todo!()).await;
    // instead of a single with the rows, we create a store for Data
    let data = Store::new(Data { rows: t1.clone() });

    //data.write().rows..set(pp[0].clone());
    //.write()..rows().write().push(pp[0].clone());

    view! {

        <h1>OIDC</h1>
        //<BusyButton/>

        <button on:click=move |_| {

            // allows iterating over the entries in an iterable store field
            //use reactive_stores::StoreFieldIterator;

            let pp = async_data.get().unwrap_or(Ok(tt.clone())).unwrap();
            //.unwrap_or(tt.clone()).clone();

            data.set(Data { rows: pp });
        }>
            "Update OIDC Clients"
        </button>



        <h1>"My Data"</h1>
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
        <For
        each=move || data.rows().clone().get().clone()
        key=|state| state.clone().id.clone()
        children=move |child| {
            view! {
                <!DOCTYPE html>
                //<p>{move || child.id().get()}</p>
                //<p>{move || child.title().get()}</p>
                <div class="card">
                <div class="card-content">
                <table><tbody>
                <tr><th>Name</th>
                    <th>{move || child.name.clone()}</th>
                </tr>
                <tr>
                    <th>Description</th>
                    <th>{move || child.description.clone()}</th>
                </tr>
                <tr>
                <th>Id_domain</th>
                <th>{move || child.id_domain.clone()}</th>
             </tr>

             <tr>
                <th>Id</th>
                <th>{move || child.id.clone()}</th>
             </tr>


            <tr>
             <th>ComponentClientType</th>
             <th>child.client_type.clone()</th>
            </tr>

               <tr>
              <th>Scopes</th>
              <th> {move || child.scopes.clone().into_iter()
                .map(|n| view! { <p>{n} </p>})
                .collect::<Vec<_>>()}</th>
               </tr>

               <tr>
             <th>Default_scope</th>
             <th> {move || child.default_scope.clone().into_iter()
                .map(|n| view! {<p>{n} </p>})
                .collect::<Vec<_>>()} </th>
              </tr>

              <tr>
             <th>Use_pkce</th>
             <th>child.use_pkce.clone()</th>
              </tr>

              <tr>
              <th>Redirect_uris</th>
              <th>{move || child.redirect_uris.clone().into_iter()
                .map(|n| view! { <p><ComponentRedirectURI s={n}/> </p>})
                .collect::<Vec<_>>()} </th>
               </tr>

               <tr>
              <th>Attributes</th>
              <th>{move || child.attributes.clone().unwrap().into_iter()
              .map(|n| view! { <p>{n.attr_name} </p>})
              .collect::<Vec<_>>()} </th>
               </tr>
              </tbody>
                </table>

                </div>
              </div>
             }
        }
    />

        </Suspense>
    }
}
