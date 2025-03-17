//use backend::rest_request_client;
use leptos::{prelude::ServerFnError, server};
//use prost::bytes;
use reactive_stores::Store;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::grpc::types_oidc::{
    ClientType, GrantType, OidcClient, RedirectURI, TokenEndpointAuthMethod, UsePKCE,
};

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: String = |row| row.id.clone())]
    pub rows: Vec<OidcClient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrpcRequest {
    View,
    List,
    Create,
    Update,
    Delete,
}

/* #[derive(Store, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub user_id: i64,
    pub id: i64,
    pub title: String,
    pub completed: bool,
} */

//

#[server]
pub async fn grpc_connector(
    _gr: GrpcRequest,
    bytes: Vec<u8>,
) -> Result<Vec<OidcClient>, ServerFnError> {
    println!("2222");
    pub mod backend {
        //include!("../backend.rs");
        tonic::include_proto!("backend");
    }
    use backend::{rest_request_client, OidcListResponse};
    let mut client = rest_request_client::RestRequestClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    /*  let request = tonic::Request::new(RequestName {
        name: String::from("value"),
    }); */

    //let response = client.idp_view(request).await?;
    /*let url = "https://jsonplaceholder.typicode.com/todos";
    // Send a POST request with the JSON payload
     let client = Client::new();
    let response = client.get(url).send().await.unwrap();
    let usr: Vec<Root> = response.json().await.unwrap(); */
    //println!("Response body: {:?}", usr.is_empty());

    //Ok(usr)

    let example_oidc_client = OidcClient {
        name: Some("Example OIDC Client".to_string()),
        description: Some("An example OIDC client for demonstration purposes".to_string()),
        id_domain: Some("example.com".to_string()),
        id: "example-client-id".to_string(),
        secret: Some("example-client-secret".to_string()),
        client_type: Some(ClientType::ConfidentialClient),
        grant_types: Some(vec![GrantType::AuthorizationCode, GrantType::RefreshToken]),
        scopes: Some(vec!["openid".to_string()]),
        use_pkce: Some(UsePKCE::Strict),
        default_scope: Some("openid profile".to_string()),
        redirect_uris: Some(vec![RedirectURI {
            url: "https://example.com/callback".to_string(),
            is_https: true,
        }]),
        attributes: Some(vec!["example_attribute".to_string()]),
        token_endpoint_auth_method: Some(TokenEndpointAuthMethod::ClientSecretBasic),
        issue_tls_client_certificate_bound_access_tokens: Some("true".to_string()),
        tls_client_auth_subject_dn: Some("CN=example.com".to_string()),
        tls_client_auth_san_dns: Some("example.com".to_string()),
        tls_client_auth_san_uri: Some("https://example.com".to_string()),
        tls_client_auth_san_ip: Some("192.168.1.1".to_string()),
        tls_client_auth_san_email: Some("admin@example.com".to_string()),
        access_token_custom_claims: Some(vec![
            "custom_claim_1".to_string(),
            "custom_claim_2".to_string(),
        ]),
        id_token_custom_claims: Some(vec!["custom_claim_3".to_string()]),
        user_info_custom_claims: Some(vec!["custom_claim_4".to_string()]),
        old_secret_retention_time_in_days: Some(30),
    };
    println!("{:?}", example_oidc_client.clone());
    Ok(vec![example_oidc_client])
}
