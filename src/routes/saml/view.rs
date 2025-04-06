use leptos::prelude::*;

use crate::{app::BusyButton, components::form::FormExample};

#[component]
pub fn ListMeta() -> impl IntoView {
    view! {

        <h1>Metadata</h1><BusyButton/>
        <FormExample/>

        /* <button on:click=move |_| {
            // log the new value of the signal
            leptos::logging::log!("{:?}", "test1");
        }>
            "Update Values"
        </button> */



    }
}
