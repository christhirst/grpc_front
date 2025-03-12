use leptos::{prelude::ServerFnError, server};
//use prost::bytes;
use reactive_stores::Store;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: i64 = |row| row.id.clone())]
    pub rows: Vec<Root>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

//

#[server]
pub async fn grpc_connector(_gr: GrpcRequest, bytes: Vec<u8>) -> Result<Vec<Root>, ServerFnError> {
    pub mod backend {
        //include!("../backend.rs");
        tonic::include_proto!("backend");
    }
    use backend::{restapi_client, IdpData, RequestList, RequestName, ResponseIdpList};
    let mut client = restapi_client::RestapiClient::connect("http://[::1]:8080")
        .await
        .unwrap();

    let request = tonic::Request::new(RequestName {
        name: String::from("value"),
    });

    let response = client.idp_view(request).await?;
    /*let url = "https://jsonplaceholder.typicode.com/todos";
    // Send a POST request with the JSON payload
     let client = Client::new();
    let response = client.get(url).send().await.unwrap();
    let usr: Vec<Root> = response.json().await.unwrap(); */
    //println!("Response body: {:?}", usr.is_empty());

    //Ok(usr)
    todo!()
}
