use std::collections::HashMap;
use std::process::exit;

use leptos::prelude::*;
use leptos::IntoView;
use openidconnect::HttpRequest;
use serde::Deserialize;
use tracing::info;

use crate::app::MyState;
use crate::settings::Http;
use leptos::server;

#[cfg(feature = "ssr")]
use leptos_axum::{redirect, ResponseOptions};
#[cfg(feature = "ssr")]
use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreProviderMetadata, CoreResponseType,
};

pub fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) -> T {
    let mut err_msg = format!("Error: {}", msg);
    let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
    while let Some(f) = cur_fail {
        err_msg.push_str(&format!(": {}", f));
        cur_fail = f.source();
    }
    exit(1);
}
#[derive(Deserialize, Debug)]
struct MyQuery {
    foo: String,
}

#[cfg(feature = "ssr")]
use http::{header, HeaderValue};
use openidconnect::{
    AccessTokenHash, AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse,
};

#[server]
pub async fn login() -> Result<String, ServerFnError> {
    use leptos_axum::extract;
    let (Cookie): (header::HeaderMap) = extract().await.unwrap();
    info!("Method: {:?}", Cookie);
    use axum::{extract::Query, http::Method};
    //use axum_extra;

    info!("Method: {:?}", "Cookie");

    let response = expect_context::<ResponseOptions>();

    if false {
    } else {
    };

    //info!("Query: {:?}", query);

    // If the code is present, we are in the callback phase.
    // Here you would typically exchange the authorization code for an access token.
    /* let code = code.unwrap();
    info!("Authorization code: {:?}", code);
    let token_response = client
        .exchange_code(AuthorizationCode::new(code))?
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await
        .unwrap();
    info!("Authorization code: {:?}", "code");
    // Extract the ID token claims after verifying its authenticity and nonce.
    let id_token = token_response
        .id_token()
        .ok_or_else(|| "This is not an error type")
        .unwrap();
    info!("Authorization code: {:?}", "code2");
    let id_token_verifier = client.id_token_verifier();
    let claims = id_token.claims(&id_token_verifier, &nonce)?;
    info!("Authorization code: {:?}", "code3");
    // Verify the access token hash to ensure that the access token hasn't been substituted for
    // another user's.
    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            id_token.signing_alg()?,
            id_token.signing_key(&id_token_verifier)?,
        )?;
        if actual_access_token_hash != *expected_access_token_hash {
            todo!()
        }
    }

    // The authenticated user's identity is now available. See the IdTokenClaims struct for a
    // complete listing of the available claims.
    println!(
        "User {} with e-mail address {} has authenticated successfully",
        claims.subject().as_str(),
        claims
            .email()
            .map(|email| email.as_str())
            .unwrap_or("<not provided>"),
    ); */

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let response = expect_context::<ResponseOptions>();
    let pkce_veri = pkce_verifier.secret().to_string();
    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!("edgedb-pkce-verifier={pkce_veri}; Path=/"))?,
    );
    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!("test={pkce_veri}; Path=/"))?,
    );

    info!("Provider metadata: {:?}", "login");
    let http_client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Use OpenID Connect Discovery to fetch the provider metadata.
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://dev-km1jar0kbzka23dv.us.auth0.com/".to_string())?,
        &http_client,
    )
    .await
    .unwrap();
    info!("Provider metadata: {:?}", provider_metadata.issuer());

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new("yj2Cg00IT0iVVTfKheKxGePvyqTQ7Qmi".to_string()),
        Some(ClientSecret::new(
            "aNHvo3t1gDiC5ihkNimRqaw7VnKjKo_h9lcUKFTZlv6BHfQtgmG4IH5ZBrazDKh_".to_string(),
        )),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(
        "http://localhost:3000/authorize".to_string(),
    )?);

    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();
    // Create an OpenID Connect client by specifying the client ID, client secret, authorization URL
    // and token URL.

    // Generate the full authorization URL.

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    info!("Browse to:  {:?}", auth_url);

    leptos_axum::redirect(auth_url.as_str());
    pkce_verifier.secret().to_string();

    info!("------ {:?}", "lll");
    Ok(pkce_verifier.secret().to_string())
}
use leptos_router::hooks::{use_params_map, use_query_map};

#[cfg(feature = "ssr")]
async fn cookie_get() -> String {
    use leptos_axum::extract;
    let (Cookie): (header::HeaderMap<HeaderValue>) = extract().await.unwrap();
    let c = Cookie.get("cookie").unwrap().clone();
    let c: Vec<&str> = c.to_str().unwrap().split(' ').collect();
    let c = c[0].split("=").collect::<Vec<&str>>()[1]
        .strip_suffix(';')
        .unwrap()
        .to_string();
    c
}

#[server]
pub async fn authorize() -> Result<String, ServerFnError> {
    use leptos_axum::extract;
    let (Cookie): (header::HeaderMap<HeaderValue>) = extract().await.unwrap();
    let c = Cookie.get("cookie").unwrap().clone();
    let c: Vec<&str> = c.to_str().unwrap().split(' ').collect();
    let c = c[0].split("=").collect::<Vec<&str>>()[1]
        .strip_suffix(';')
        .unwrap();

    info!("Method: {:?}", c);

    todo!()
}
//#[server]
/* pub async fn clientBuild() -> Result<(), ServerFnError> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap_or_else(|err| {
            handle_error(&err, "Failed to build HTTP client");
            unreachable!()
        });
    todo!()
} */

async fn client_custom(request: HttpRequest) {
    // our resource

    /* let http_client = reqwest::ClientBuilder::new()
           // Following redirects opens the client up to SSRF vulnerabilities.
           .redirect(reqwest::redirect::Policy::none())
           .build()
           .expect("Client should build");

       let m = match *request.method() {
           openidconnect::http::Method::GET => Method::GET,
           openidconnect::http::Method::POST => Method::POST,
           _ => panic!("Unsupported HTTP method"),
       };
    */
    todo!()
}
