use anyhow;
use leptos::prelude::*;
use leptos::IntoView;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde_with::skip_serializing_none]
#[derive(Store, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OidcClient {
    pub name: Option<String>,
    pub description: Option<String>,
    pub id_domain: Option<String>,
    pub id: String,
    pub secret: Option<String>,
    pub client_type: Option<String>,
    pub grant_types: Option<Vec<String>>,
    pub scopes: Option<Vec<String>>,
    pub use_pkce: Option<String>,
    pub default_scope: Option<String>,
    pub redirect_uris: Option<Vec<RedirectURI>>,
    pub attributes: Option<Vec<Attributes>>,
    pub token_endpoint_auth_method: Option<String>,
    pub issue_tls_client_certificate_bound_access_tokens: Option<bool>,
    pub tls_client_auth_subject_dn: Option<String>,
    pub tls_client_auth_san_dns: Option<String>,
    pub tls_client_auth_san_uri: Option<String>,
    pub tls_client_auth_san_ip: Option<String>,
    pub tls_client_auth_san_email: Option<String>,
    pub access_token_custom_claims: Option<Vec<String>>,
    pub id_token_custom_claims: Option<Vec<String>>,
    pub user_info_custom_claims: Option<Vec<String>>,
    pub old_secret_retention_time_in_days: Option<i32>,
}

/* impl From<OidcListResponse> for Vec<OidcClient> {
    fn from(response: OidcListResponse) -> Self {
        todo!()
    }
} */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attributes {
    pub attr_name: String,
    pub attr_value: String,
    pub attr_type: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientType {
    #[default]
    ConfidentialClient,
    PublicClient,
    MobileClient,
}

#[component]
pub fn ComponentClientType(s: Option<ClientType>) -> impl IntoView {
    match s {
        (ConfidentialClient) => view! { <p>ConfidentialClient</p> }.into_any(),
        (PublicClient) => view! { <p>PublicClient</p> }.into_any(),
        (MobileClient) => view! { <p>MobileClient</p> }.into_any(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UsePKCE {
    NonStrict,
    Strict,
}

#[component]
pub fn ComponentUsePKCE(s: Option<UsePKCE>) -> impl IntoView {
    match s {
        (NonStrict) => view! { <p>NonStrict</p> }.into_any(),
        (Strict) => view! { <p>Strict</p> }.into_any(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedirectURI {
    pub url: String,
    //#[serde(rename(deserialize = "isHttps"))]
    pub is_https: bool,
}

#[component]
pub fn ComponentRedirectURI(s: Vec<RedirectURI>) -> impl IntoView {
    match s {
        (NonStrict) => view! { <p>Vector1</p><p>Vector2</p> }.into_any(),
        (Strict) => view! { <p>Vector3</p><p>Vector4</p> }.into_any(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GrantType {
    Password,
    RefreshToken,
    JwtBearer,
    ClientCredentials,
    AuthorizationCode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TokenEndpointAuthMethod {
    TlsClientAuth,
    SelfSignedTlsClientAuth,
    PrivateKeyJwt,
    ClientSecretBasic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeSettings {
    // Define fields for ScopeSettings based on the schema
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceServer {
    pub name: String,               // Required field
    pub id_domain: String,          // Required field
    pub scopes: Vec<ScopeSettings>, // Required field

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_attributes: Option<Vec<String>>,
}
