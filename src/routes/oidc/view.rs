use std::default;

use leptos::prelude::*;
use reactive_stores::Store;

use crate::app::BusyButton;
use crate::grpc::init::grpc_connector;
use crate::grpc::init::Data;
use crate::grpc::init::DataStoreFields;
use crate::grpc::init::GrpcRequest;
use crate::grpc::init::Root;
use crate::grpc::init::RootStoreFields;
//use crate::list::api::get;
//use crate::list::api::RootStoreFields;
//use crate::list::api::Data;
//use crate::list::api::Root;
//use crate::list::api::DataStoreFields;
//use crate::list::api::RootStoreFields;

fn load(async_data: Resource<Result<Vec<Root>>>) -> Vec<Root> {
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
}

#[component]
pub fn Oidc() -> impl IntoView {
    let rq = GrpcRequest::List;
    let (count, set_count) = signal(0);
    let (t, tt) = signal(0);
    let async_data = Resource::new(
        move || t.get(),
        // every time `count` changes, this will run
        |t| grpc_connector(String::from("value"), GrpcRequest::List),
    );
    let u = vec![Root {
        user_id: 1,
        id: 66,
        title: String::from("test66"),
        completed: true,
    }];

    let uee = vec![Root {
        user_id: 1,
        id: 66,
        title: String::from("test66"),
        completed: true,
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
    let data = Store::new(Data { rows: u.clone() });

    //data.write().rows..set(pp[0].clone());
    //.write()..rows().write().push(pp[0].clone());
    let values = vec![0, 1, 2];

    let oo = Root {
        user_id: 3,
        id: 3,
        title: String::from("test"),
        completed: true,
    };
    let mut count: i64 = 2;
    view! {

        <h1>OIDC</h1><BusyButton/>
        <button on:click=move |_| {
            // allows iterating over the entries in an iterable store field
            use reactive_stores::StoreFieldIterator;

            let pp = async_data.get().unwrap_or(Ok(u.clone())).unwrap_or(uee.clone()).clone();
            data.set(Data { rows: pp });
        }>
            "Update OIDC Clients"
        </button>

        <h1>"My Data"</h1>
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
        <For
        each=move || data.rows()
        key=|row| row.read().id.clone()
        children=|child| {
            let value = child.id();
            view! { <h1>testlist</h1>
                <p>{move || value.get()}</p>
             }
        }
    />

        </Suspense>



        // iterate over the rows and display each value
        //<Suspense></Suspense>



    }
}
