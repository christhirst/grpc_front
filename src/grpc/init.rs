/* use proto;*/
//use crate::grpc_server;
//use proto::;
use leptos::{prelude::ServerFnError, server};
use reactive_stores::Store;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/* #[tonic::async_trait]
impl Indicator for IndicatorService {} */

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: i64 = |row| row.id.clone())]
    pub rows: Vec<Root>,
}

//#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Store, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GrpcRequest {
    View,
    List,
    Create,
    Update,
    Delete,
}

#[derive(Store, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub user_id: i64,
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

#[server]
pub async fn grpc_connector(title: String, gr: GrpcRequest) -> Result<Vec<Root>, ServerFnError> {
    /* pub mod backend {
        //include!("../backend.rs");
        tonic::include_proto!("backend");
    }
    use backend::restapi_client;
    restapi_client::RestapiClient::connect("http://[::1]:8080")
        .await
        .unwrap(); */

    //println!("Response body: {:?}", title);
    //https://jsonplaceholder.typicode.com/users
    // Define the URL of the endpoint
    let url = "https://jsonplaceholder.typicode.com/todos";

    // Send a POST request with the JSON payload
    let client = Client::new();
    let response = client.get(url).send().await.unwrap();
    let usr: Vec<Root> = response.json().await.unwrap();
    println!("Response body: {:?}", usr.is_empty());
    // Check if the request was successful
    /* if response.status().is_success() {
        // Parse the response body as a string
        let body = response.text().await?;

    } else {
        println!("Request failed with status: {}", response.status());
    } */
    /* let oo = Root {
        user_id: 1,
        id: 1,
        title: String::from("test"),
        completed: true,
    }; */

    Ok(usr)
}
