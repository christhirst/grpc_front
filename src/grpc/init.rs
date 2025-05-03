//use backend::NameRequest;
//use backend::rest_request_client;
use leptos::{prelude::ServerFnError, server};
use tracing::info;
//use prost::bytes;
use reactive_stores::Store;

use crate::{grpc::types_oidc::*, settings::Settings};
use serde::{Deserialize, Serialize};
//use std::convert::TryInto;

use super::types_saml::IdpPartner;

/* #[cfg(feature = "ssr")]
pub mod backend {
    //include!("../backend.rs");
    tonic::include_proto!("backend");
} */
#[cfg(feature = "ssr")]
use backend::{restrequest_client, IdpPartnerPrt, ViewRequest};

//use crate::grpc::types_oidc::RedirectURI;

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: String = |row| row.id.clone())]
    pub rows: Vec<OidcClient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrpcRequest {
    View(String),
    List,
    Create,
    Update,
    Delete(String),
}

#[allow(dead_code)]
fn setting() -> (String, String) {
    info!("setting");
    /*#region settings */
    let settings = Settings::new().unwrap();
    let grpc = format!(
        "{}://{}:{}",
        settings.grpc.scheme, settings.grpc.server, settings.grpc.port
    );
    /*#endregion */
    (grpc.clone(), grpc)
}

#[server]
pub async fn grpc_connector_saml(
    gr: GrpcRequest,
    idp: IdpPartner,
) -> Result<Vec<IdpPartner>, ServerFnError> {
    info!("grpc_connector_saml");

    let (grpc, _notimplemented) = setting();

    let created: IdpPartnerPrt = idp.try_into().unwrap();

    impl Into<IdpPartnerPrt> for IdpPartner {
        fn into(self) -> IdpPartnerPrt {
            IdpPartnerPrt {
                partner_name: self.partner_name,
                metadata_b64: self.metadata_b64,
                ..Default::default()
            }
        }
    }

    let idp_partner = match restrequest_client::RestrequestClient::connect(grpc).await {
        Ok(mut client) => {
            let grpc = match gr {
                GrpcRequest::View(idp) => {
                    let name_request = backend::NameRequest { name: idp.clone() };
                    let resp = client.idp_view(name_request).await?.into_inner();
                    let resp: IdpPartner = resp.try_into().unwrap();
                    vec![resp]
                }
                GrpcRequest::List => {
                    let list_request = backend::ViewRequest {
                        list: true,
                        name: String::from("value"),
                    };
                    let resp = client.idp_list(list_request).await?.into_inner();
                    let resp = resp.try_into().unwrap();
                    resp
                }
                GrpcRequest::Create => {
                    let resp = client.idp_create(created).await?.into_inner();
                    let resp: IdpPartner = resp.try_into().unwrap();
                    vec![resp]
                }
                GrpcRequest::Update => {
                    let resp = client.idp_edit(created).await?.into_inner();
                    let resp: IdpPartner = resp.try_into().unwrap();
                    vec![resp]
                }
                GrpcRequest::Delete(idp) => {
                    let name_request = backend::NameRequest { name: idp.clone() };
                    let resp = client.idp_delete(name_request).await?.into_inner();
                    let resp: IdpPartner = IdpPartner {
                        ..Default::default()
                    };
                    vec![resp]
                }
            };
            Ok(grpc)
        }
        Err(_) => {
            todo!()
        }
    };
    idp_partner
}

#[server]
pub async fn grpc_connector_oidc(
    gr: GrpcRequest,
    _bytes: Vec<u8>,
) -> Result<Vec<OidcClient>, ServerFnError> {
    info!("grpc_connector_oidc");

    /*#region TryFrom */

    /* #endregion */

    let (grpc, _notimplemented) = setting();

    match gr {
        GrpcRequest::Create => {
            println!("Create");
            println!("Create");
        }
        _ => {
            println!("others");
            println!("others");
        }
    }

    let oidc_clients = match restrequest_client::RestrequestClient::connect(grpc).await {
        Ok(mut client) => {
            println!("grpc call");
            let request = ViewRequest {
                list: true,
                name: String::from("value"),
            };
            let resp = client.oidc_list(request).await?.into_inner();
            let resp = resp.try_into().unwrap();
            resp
        }
        Err(_) => {
            /* #region FakeData */
            let example_oidc_client = OidcClient {
                name: Some("Example OIDC Client".to_string()),
                description: Some("An example OIDC client for demonstration purposes".to_string()),
                id_domain: Some("example.com".to_string()),
                id: "example-client-id".to_string(),
                secret: Some("example-client-secret".to_string()),
                client_type: Some(String::from("ConfidentialClient")),
                grant_types: Some(vec![
                    String::from("AuthorizationCode"),
                    String::from("RefreshToken"),
                ]),
                scopes: Some(vec!["openid".to_string()]),
                use_pkce: Some(String::from("Strict")),
                default_scope: Some("openid profile".to_string()),
                redirect_uris: Some(vec![RedirectURI {
                    url: "https://example.com/callback".to_string(),
                    is_https: true,
                }]),
                attributes: Some(vec![crate::grpc::types_oidc::Attributes {
                    attr_name: String::from("example_attribute"),
                    attr_value: String::from("example_attribute"),
                    attr_type: String::from("example_attribute"),
                }]),
                token_endpoint_auth_method: Some(String::from("ClientSecretBasic")),
                issue_tls_client_certificate_bound_access_tokens: Some(true),
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
            /* #endregion */

            let mut nn = example_oidc_client.clone();
            nn.id = String::from("value");

            vec![example_oidc_client.clone(), nn]
        }
    };
    Ok(oidc_clients)
}
