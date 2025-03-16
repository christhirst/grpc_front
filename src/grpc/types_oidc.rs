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
    pub client_type: Option<ClientType>,
    pub grant_types: Option<Vec<GrantType>>,
    pub scopes: Option<Vec<String>>,
    pub use_pkce: Option<UsePKCE>,
    pub default_scope: Option<String>,
    pub redirect_uris: Option<Vec<RedirectURI>>,
    pub attributes: Option<Vec<String>>,
    pub token_endpoint_auth_method: Option<TokenEndpointAuthMethod>,
    pub issue_tls_client_certificate_bound_access_tokens: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientType {
    ConfidentialClient,
    PublicClient,
    MobileClient,
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UsePKCE {
    NonStrict,
    Strict,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedirectURI {
    pub url: String,
    //#[serde(rename(deserialize = "isHttps"))]
    pub is_https: bool,
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
