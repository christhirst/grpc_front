use leptos::{prelude::ServerFnError, server};
use reactive_stores::Store;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: i64 = |row| row.id.clone())]
    pub rows: Vec<Root>,
}
/* pub mod backend {
    include!("../backend.rs");
    //tonic::include_proto!("../backend");
} */

#[derive(Store, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Root {
    pub user_id: i64,
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

/* enum GrpcRequest {
    View,
    List,
    Create,
    Update,
    Delete,
} */

#[server]
pub async fn get(title: String) -> Result<Root, ServerFnError> {
    /* pub mod backend {
        //include!("../backend.rs");
        tonic::include_proto!("backend");
    }
    use backend::restapi_client;
    restapi_client::RestapiClient::connect("http://[::1]:8080")
        .await
        .unwrap(); */

    println!("Response body: {:?}", title);
    //https://jsonplaceholder.typicode.com/users
    // Define the URL of the endpoint
    let url = "https://jsonplaceholder.typicode.com/todos";

    // Send a POST request with the JSON payload
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.unwrap();
    let usr: Vec<Root> = response.json().await.unwrap();
    println!("Response body: {:?}", usr.len());
    // Check if the request was successful
    /* if response.status().is_success() {
        // Parse the response body as a string
        let body = response.text().await?;

    } else {
        println!("Request failed with status: {}", response.status());
    } */
    let oo = Root {
        user_id: 1,
        id: 1,
        title: String::from("test"),
        completed: true,
    };

    Ok(oo)
}

#[server]
pub async fn saml_list(title: String) -> Result<Root, ServerFnError> {
    println!("Response body: {:?}", title);
    //let mut client = VotingClient::connect("http://[::1]:8080");
    //https://jsonplaceholder.typicode.com/users
    // Define the URL of the endpoint

    let oo = Root {
        user_id: 1,
        id: 1,
        title: String::from("test"),
        completed: true,
    };

    Ok(oo)
}
