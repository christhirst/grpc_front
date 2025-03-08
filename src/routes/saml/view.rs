use leptos::prelude::*;
use reactive_stores::Store;

use crate::app::BusyButton;
use crate::list::api::get;
use crate::list::api::Data;
use crate::list::api::DataStoreFields;
use crate::list::api::Root;
use crate::list::api::RootStoreFields;

#[component]
pub fn ListMeta() -> impl IntoView {
    let (_count, _set_count) = signal(0);
    let (t, _tt) = signal(0);
    let async_data = Resource::new(
        move || t.get(),
        // every time `count` changes, this will run
        |_t| get(String::from("value")),
    );
    let u = Root {
        user_id: 1,
        id: 66,
        title: String::from("test66"),
        completed: true,
    };

    let pp = async_data.get().unwrap_or(Ok(u.clone())).unwrap_or(u);

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
    let data = Store::new(Data {
        rows: vec![
            Root {
                user_id: 1,
                id: 1,
                title: String::from("test"),
                completed: true,
            },
            Root {
                user_id: 2,
                id: 2,
                title: String::from("test"),
                completed: true,
            },
        ],
    });

    data.rows().write().push(pp);

    let _oo = Root {
        user_id: 3,
        id: 3,
        title: String::from("test"),
        completed: true,
    };
    let mut count: i64 = 2;
    view! {

        <h1>Metadata</h1><BusyButton/>

        //<DynamicList initial_length=10/>
        /* <ul>
        {values.into_iter()
            .map(|n| view! { <li>{n}</li>})
            .collect::<Vec<_>>()}
    </ul> */

        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            // allows iterating over the entries in an iterable store field
            //use reactive_stores::StoreFieldIterator;

            // calling rows() gives us access to the rows
            /* for row in data.rows().iter_unkeyed() {
                *row.write() = oo.clone();
            } */
            count =count+1;
            data.rows().write().push(Root {
                id: count,
                user_id: 3,

        title: String::from("test"),
        completed:true,
            });


            // log the new value of the signal
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        // iterate over the rows and display each value
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


    }
}
