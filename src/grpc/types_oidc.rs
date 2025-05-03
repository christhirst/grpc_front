use leptos::prelude::*;
use leptos::IntoView;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
#[cfg(feature = "ssr")]
pub mod backend {
    //include!("../backend.rs");
    tonic::include_proto!("backend");
}

use super::types_saml::IdpPartner;
#[cfg(feature = "ssr")]
use backend::{IdpListResponse, IdpPartnerPrt, OidcListResponse};

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
        Some(ClientType::ConfidentialClient) => view! { <p>ConfidentialClient</p> }.into_any(),
        Some(ClientType::PublicClient) => view! { <p>PublicClient</p> }.into_any(),
        Some(ClientType::MobileClient) => view! { <p>MobileClient</p> }.into_any(),
        None => view! { <p>None</p> }.into_any(),
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
        Some(UsePKCE::NonStrict) => view! { <p>NonStrict</p> }.into_any(),
        Some(UsePKCE::Strict) => view! { <p>Strict</p> }.into_any(),
        None => view! { <p>None</p> }.into_any(),
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
    view! {
               <p>{s.into_iter().map(|n|view! {
                   <ul>
                   <li>{n.url}</li>
                   </ul>
               }).collect::<Vec<_>>()}</p>
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

#[cfg(feature = "ssr")]
impl TryFrom<OidcListResponse> for Vec<OidcClient> {
    type Error = anyhow::Error;
    fn try_from(value: OidcListResponse) -> Result<Self, Self::Error> {
        let mut oidc_clients = vec![];
        let mut attr: Vec<crate::grpc::types_oidc::Attributes> = vec![];

        for c in value.oidc_clients.into_iter() {
            for a in c.attributes.into_iter() {
                attr.push(crate::grpc::types_oidc::Attributes {
                    attr_name: a.attr_name,
                    attr_value: a.attr_value,
                    attr_type: a.attr_type,
                });
            }
            let mut red: Vec<crate::grpc::types_oidc::RedirectURI> = vec![];
            for r in c.redirect_ur_is.into_iter() {
                red.push(RedirectURI {
                    url: r.url,
                    is_https: r.is_https,
                });
            }

            let oc = OidcClient {
                name: Some(c.name),
                description: Some(c.description),
                id_domain: Some(c.id_domain),
                id: c.id,
                secret: Some(c.secret),
                client_type: Some(c.client_type),
                grant_types: Some(c.grant_types),
                scopes: Some(c.scopes),
                use_pkce: Some(c.use_pkce),
                default_scope: Some(c.default_scope),
                redirect_uris: Some(red),
                attributes: Some(attr.clone()),
                token_endpoint_auth_method: Some(c.token_endpoint_auth_method),
                issue_tls_client_certificate_bound_access_tokens: Some(
                    c.issue_tls_client_certificate_bound_access_tokens,
                ),
                tls_client_auth_subject_dn: Some(c.tls_client_auth_subject_dn),
                tls_client_auth_san_dns: Some(c.tls_client_auth_sandns),
                tls_client_auth_san_uri: Some(c.tls_client_auth_sanuri),
                tls_client_auth_san_ip: Some(c.tls_client_auth_sanip),
                tls_client_auth_san_email: Some(c.tls_client_auth_san_email),
                access_token_custom_claims: Some(c.access_token_custom_claims),
                id_token_custom_claims: Some(c.id_token_custom_claims),
                user_info_custom_claims: Some(c.user_info_custom_claims),
                old_secret_retention_time_in_days: Some(c.old_secret_retention_time_in_days),
            };
            oidc_clients.push(oc);
        }

        Ok(oidc_clients)
    }
}

#[cfg(feature = "ssr")]
impl TryFrom<IdpPartnerPrt> for IdpPartner {
    type Error = anyhow::Error;

    fn try_from(value: IdpPartnerPrt) -> Result<Self, Self::Error> {
        IdpPartner {
            metadata_b64: value.metadata_b64,
            metadata_url: value.metadata_url,
            partner_type: value.partner_type,
            tenant_name: value.tenant_name,
            tenant_url: value.tenant_url,
            partner_name: value.partner_name,
            name_id_format: value.name_id_format,
            sso_profile: value.sso_profile,
            attribute_ldap: value.attribute_ldap,
            attribute_saml: value.attribute_saml,
            fa_welcome_page: value.fa_welcome_page,
            generate_new_keys: value.generate_new_keys,
            validity_new_keys: value.validity_new_keys,
            preverify: value.preverify,
            provider_id: value.provider_id,
            sso_url: value.sso_url,
            sso_soap_url: value.sso_soapurl,
            logout_request_url: value.logout_request_url,
            logout_response_url: value.logout_response_url,
            assertion_consumer_url: value.assertion_consumer_url,
            succinct_id: value.succinct_id,
            signing_cert: value.signing_cert,
            encryption_cert: value.encryption_cert,
            signature_digest_algorithm: value.signature_digest_algorithm,
            signing_keystore_access_template_id: value.signing_keystore_access_template_id,
            encryption_keystore_access_template_id: value.encryption_keystore_access_template_id,

            ..Default::default() /*
                                 signing_keystore_access_template_id: None
                                     value
                                         .signing_keystore_access_template_id
                                         .unwrap_or_default(),
                                 ),
                                 encryption_keystore_access_template_id: None
                                     value
                                         .encryption_keystore_access_template_id
                                         .unwrap_or_default(),
                                 ),
                                 admin_fed_instance_type: Nonevalue.admin_fed_instance_type.unwrap_or_default()), */
        };
        todo!()
    }
}

#[cfg(feature = "ssr")]
impl TryFrom<IdpListResponse> for Vec<IdpPartner> {
    type Error = anyhow::Error;
    fn try_from(value: IdpListResponse) -> Result<Self, Self::Error> {
        let mut idp_partners = vec![];
        for c in value.idp_partner_list.into_iter() {
            let idp = IdpPartner {
                metadata_b64: c.metadata_b64,
                metadata_url: c.metadata_url,
                partner_type: c.partner_type,
                tenant_name: c.tenant_name,
                tenant_url: c.tenant_url,
                partner_name: c.partner_name,
                name_id_format: c.name_id_format,
                sso_profile: c.sso_profile,
                attribute_ldap: c.attribute_ldap,
                attribute_saml: c.attribute_saml,
                fa_welcome_page: c.fa_welcome_page,
                generate_new_keys: c.generate_new_keys,
                validity_new_keys: c.validity_new_keys,
                preverify: c.preverify,
                provider_id: c.provider_id,
                sso_url: c.sso_url,
                sso_soap_url: c.sso_soapurl,
                logout_request_url: c.logout_request_url,
                logout_response_url: c.logout_response_url,
                assertion_consumer_url: c.assertion_consumer_url,
                succinct_id: c.succinct_id,
                signing_cert: c.signing_cert,
                encryption_cert: c.encryption_cert,
                signature_digest_algorithm: c.signature_digest_algorithm,

                ..Default::default()
            };
            idp_partners.push(idp);
        }
        Ok(idp_partners)
    }
}
