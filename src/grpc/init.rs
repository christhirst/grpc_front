//use backend::NameRequest;
//use backend::rest_request_client;
use leptos::{prelude::ServerFnError, server};
use prost_types::value;
//use prost::bytes;
use reactive_stores::Store;

use serde::{Deserialize, Serialize};

use crate::grpc::types_oidc::OidcClient;

use super::types_saml::IdpPartner;

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
pub async fn grpc_connector_saml(
    gr: GrpcRequest,
    _bytes: Vec<u8>,
) -> Result<Vec<IdpPartner>, ServerFnError> {
    pub mod backend {
        //include!("../backend.rs");
        tonic::include_proto!("backend");
    }
    use backend::{restrequest_client, IdpPartnerPrt};
    /*#region TryFrom */
    impl TryFrom<IdpPartnerPrt> for IdpPartner {
        type Error = anyhow::Error;

        fn try_from(value: IdpPartnerPrt) -> Result<Self, Self::Error> {
            IdpPartner {
                metadata_b64: Some(value.metadata_b64),
                metadata_url: Some(value.metadata_url),
                partner_type: Some(value.partner_type),
                tenant_name: Some(value.tenant_name),
                tenant_url: Some(value.tenant_url),
                partner_name: Some(value.partner_name),
                name_id_format: Some(value.name_id_format),
                sso_profile: Some(value.sso_profile),
                attribute_ldap: Some(value.attribute_ldap),
                attribute_saml: Some(value.attribute_saml),
                fa_welcome_page: Some(value.fa_welcome_page),
                generate_new_keys: Some(value.generate_new_keys),
                validity_new_keys: Some(value.validity_new_keys),
                preverify: Some(value.preverify),
                provider_id: Some(value.provider_id),
                sso_url: Some(value.sso_url),
                sso_soap_url: Some(value.sso_soapurl),
                logout_request_url: Some(value.logout_request_url),
                logout_response_url: Some(value.logout_response_url),
                assertion_consumer_url: Some(value.assertion_consumer_url),
                succinct_id: Some(value.succinct_id),
                signing_cert: Some(value.signing_cert),
                encryption_cert: Some(value.encryption_cert),
                signature_digest_algorithm: Some(value.signature_digest_algorithm),
                signing_keystore_access_template_id: Some(
                    value.signing_keystore_access_template_id,
                ),
                encryption_keystore_access_template_id: Some(
                    value.encryption_keystore_access_template_id,
                ),
                admin_fed_instance_type: Some(value.admin_fed_instance_type),
            };
            todo!()
        }
    }
    /* #endregion */

    /*#region TryFrom */
    //impl TryFrom<IdpListResponse> for Vec<IdpPartner> {}
    /* #endregion */

    todo!()
}

#[server]
pub async fn grpc_connector_oidc(
    gr: GrpcRequest,
    _bytes: Vec<u8>,
) -> Result<Vec<OidcClient>, ServerFnError> {
    use crate::grpc::types_oidc::RedirectURI;

    pub mod backend {
        //include!("../backend.rs");
        tonic::include_proto!("backend");
    }
    use backend::{restrequest_client, OidcListResponse, ViewRequest};

    /*#region TryFrom */
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
    /* #endregion */

    /*#region settings */
    use crate::settings;
    use settings::Settings;
    let settings = Settings::new().unwrap();
    let grpc = format!(
        "{}://{}:{}",
        settings.grpc.scheme, settings.grpc.server, settings.grpc.port
    );
    /*#endregion */

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

    let oo = match restrequest_client::RestrequestClient::connect(grpc).await {
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
    Ok(oo)
}
