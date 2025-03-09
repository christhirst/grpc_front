use leptos::prelude::*;
use reactive_stores::Store;

use crate::app::BusyButton;

#[component]
pub fn ListMeta() -> impl IntoView {
    view! {

        <h1>Metadata</h1><BusyButton/>


        <button on:click=move |_| {


            // log the new value of the signal
            leptos::logging::log!("{:?}", "test1");
        }>
            "Update Values"
        </button>



    }
}
