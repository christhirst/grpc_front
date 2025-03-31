use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct IdpPartner {
    pub metadata_b64: Option<Vec<u8>>,
    pub metadata_url: Option<String>,
    pub partner_type: Option<String>,
    pub tenant_name: Option<String>,
    pub tenant_url: Option<String>,
    pub partner_name: Option<String>,
    pub name_id_format: Option<String>,
    pub sso_profile: Option<String>,
    pub attribute_ldap: Option<String>,
    pub attribute_saml: Option<String>,
    pub fa_welcome_page: Option<String>,
    pub generate_new_keys: Option<String>,
    pub validity_new_keys: Option<String>,
    pub preverify: Option<bool>,
    pub provider_id: Option<String>,
    pub sso_url: Option<String>,
    pub sso_soap_url: Option<String>,
    pub logout_request_url: Option<String>,
    pub logout_response_url: Option<String>,
    pub assertion_consumer_url: Option<String>,
    pub succinct_id: Option<String>,
    pub signing_cert: Option<String>,
    pub encryption_cert: Option<String>,
    pub signature_digest_algorithm: Option<String>,
    pub signing_keystore_access_template_id: Option<String>,
    pub encryption_keystore_access_template_id: Option<String>,
    pub admin_fed_instance_type: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPPartnerDataSet {
    pub metadata_b64: Option<String>,
    pub metadata_url: Option<String>,
    pub partner_type: Option<String>,
    pub tenant_name: Option<String>,
    pub tenant_url: Option<String>,
    pub partner_name: Option<String>,
    pub name_id_format: Option<NameIDFormat>,
    pub sso_profile: Option<SSOProfile>,
    pub generate_new_keys: Option<GenerateNewKeys>,
    pub validity_new_keys: Option<String>,
    pub preverify: Option<String>,
    pub provider_id: Option<String>,
    pub sso_url: Option<String>,
    pub sso_soap_url: Option<String>,
    pub logout_request_url: Option<String>,
    pub logout_response_url: Option<String>,
    pub assertion_consumer_url: Option<String>,
    pub signing_cert: Option<String>,
    pub encryption_cert: Option<String>,
    pub last_name_attr_name: Option<String>,
    pub first_name_attr_name: Option<String>,
    pub user_name_attr_name: Option<String>,
    pub email_attr_name: Option<String>,
    pub static_attr_name: Option<String>,
    pub static_attr_value: Option<String>,
    pub custom_attrs_str: Option<String>,
    pub signature_digest_algorithm: Option<String>,
    pub signing_keystore_access_template_id: Option<String>,
    pub encryption_keystore_access_template_id: Option<String>,
    pub admin_fed_instance_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NameIDFormat {
    Emailaddress,
    Unspecified,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SSOProfile {
    Httppost,
    Artifact,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenerateNewKeys {
    True,
    False,
}
