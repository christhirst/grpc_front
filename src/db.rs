use leptos::server;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    nonce: String,
    pkce_verifier: String,
    //last: &'a str,
}

#[server]
pub async fn session_save(code: Option<String>) -> Result<(), ServerFnError> {
    // For an in memory database
    use surrealdb::engine::local::Mem;
    use surrealdb::Surreal;

    // Create database connection in memory
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Option<Session> = db
        .create("person")
        .content(Session {
            nonce: String::from("nonce"),
            pkce_verifier: String::from("pkce_verifier"),
        })
        .await?;
    dbg!(created);

    todo!();
}
