use leptos::prelude::*;
//use leptos::task::spawn_local;
use reactive_stores::Store;

use crate::grpc::init::grpc_connector_oidc;
use crate::grpc::init::Data;
use crate::grpc::init::DataStoreFields;
use crate::grpc::init::GrpcRequest;
//use crate::grpc::init::Root;
//use crate::grpc::init::RootStoreFields;
//use crate::grpc::types_oidc::OidcClientStoreFields;
use crate::grpc::types_oidc::ComponentRedirectURI;
use crate::grpc::types_oidc::{Attributes, OidcClient};

#[component]
pub fn Oidc() -> impl IntoView {
    let _rq = GrpcRequest::List;
    let (_count, _set_count) = signal(0);
    let (t, tt) = signal(0);

    let async_data = Resource::new(
        move || t.get(),
        // every time `count` changes, this will run
        |_t| {
            println!("ee");
            grpc_connector_oidc(GrpcRequest::List, vec![1])
        },
    );

    /* let tt = vec![OidcClient {
        id: String::from("11"),
        ..Default::default()
    }]; */

    let t1 = vec![OidcClient {
        id: String::from("222"),
        attributes: Some(vec![Attributes {
            attr_name: String::from("example_attribute"),
            attr_value: String::from("example_attribute"),
            attr_type: String::from("example_attribute"),
        }]),
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
    /* let data = Store::new(Data {
           rows: async_data.get().unwrap_or(Ok(tt.clone())).unwrap(),
       });
    */
    let data = Store::new(Data { rows: t1.clone() });

    view! {

           <h1>OIDC</h1>
          // {println!("test")}
           <button on:click=move |_| {
            *tt.write() += 1;
               let pp = async_data.get().unwrap().unwrap();
               //_or(Ok(t1.clone())).unwrap_or_default();

               data.set(Data { rows: pp });
           }>
               "Update OIDC Clients"
           </button>

           /* <Suspense
           fallback=move || view! { <p>"Loading..."</p> }
       >
    {
        match async_data.get(){
        Some(s)=>{
        println!("{s:?}");
        println!("--------")
    },
        None=>print!("2"),
    }
    }
    </Suspense> */


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
                 <th>{move || child.attributes.clone().unwrap_or_default().into_iter()
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

/* #[component]
fn TodoRow(store: Store<Data>, #[prop(into)] todo: Field<Data>) -> impl IntoView {
    todo!()
} */
