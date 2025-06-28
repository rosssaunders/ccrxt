use std::sync::Arc;

use axum::{
    Form, Router,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sha2::Sha256;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone, Default)]
struct Credentials {
    api_key: SecretString,
    api_secret: SecretString,
}

#[derive(Default)]
struct AppState {
    creds: Mutex<Option<Credentials>>,
}

#[derive(Deserialize)]
struct CredentialsForm {
    api_key: String,
    api_secret: String,
}

fn sign_query(query: &str, secret: &SecretString) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.expose_secret().as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(query.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

async fn index() -> Html<&'static str> {
    Html(
        "<html><head><title>CCRXT Web UI</title></head><body>
        <h1>Crypto Exchange Web UI</h1>
        <form action='/set_keys' method='post'>
            <label>API Key: <input name='api_key' type='text'/></label><br/>
            <label>API Secret: <input name='api_secret' type='text'/></label><br/>
            <button type='submit'>Save Keys</button>
        </form>
        <form action='/binance/server_time' method='post'>
            <button type='submit'>Get Binance Server Time</button>
        </form>
        <form action='/binance/account' method='post'>
            <button type='submit'>Get Binance Account Info</button>
        </form>
        </body></html>",
    )
}

#[axum::debug_handler]
async fn set_keys(
    state: axum::extract::State<Arc<AppState>>,
    Form(form): Form<CredentialsForm>,
) -> impl IntoResponse {
    let creds = Credentials {
        api_key: SecretString::new(form.api_key.into()),
        api_secret: SecretString::new(form.api_secret.into()),
    };
    *state.creds.lock().await = Some(creds);
    info!("Stored API credentials");
    axum::response::Redirect::to("/")
}

async fn binance_server_time() -> Html<String> {
    let resp = match reqwest::get("https://fapi.binance.com/fapi/v1/time").await {
        Ok(r) => r,
        Err(e) => return Html(format!("<p>Error: {e}</p><p><a href='/'>Back</a></p>")),
    };
    let json: serde_json::Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => return Html(format!("<p>Error: {e}</p><p><a href='/'>Back</a></p>")),
    };
    let pretty = match serde_json::to_string_pretty(&json) {
        Ok(p) => p,
        Err(e) => return Html(format!("<p>Error: {e}</p><p><a href='/'>Back</a></p>")),
    };
    Html(format!("<pre>{}</pre><p><a href='/'>Back</a></p>", pretty))
}

#[axum::debug_handler]
async fn binance_account(state: axum::extract::State<Arc<AppState>>) -> Html<String> {
    let creds_guard = state.creds.lock().await;
    let Some(creds) = &*creds_guard else {
        return Html("<p>No API keys set. <a href='/'>Back</a></p>".to_string());
    };
    use chrono::Utc;

    let timestamp = Utc::now().timestamp_millis();
    let recv_window = 5000u64;
    let query = format!("timestamp={timestamp}&recvWindow={recv_window}");
    let signature = sign_query(&query, &creds.api_secret);
    let url = format!("https://fapi.binance.com/fapi/v2/account?{query}&signature={signature}");
    let client = reqwest::Client::new();
    let resp = match client
        .get(&url)
        .header("X-MBX-APIKEY", creds.api_key.expose_secret())
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => return Html(format!("<p>Error: {e}</p><p><a href='/'>Back</a></p>")),
    };
    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Html(format!("<p>Error: {e}</p><p><a href='/'>Back</a></p>")),
    };
    Html(format!("<pre>{}</pre><p><a href='/'>Back</a></p>", text))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let state = Arc::new(AppState::default());
    let app = Router::new()
        .route("/", get(index))
        .route("/set_keys", post(set_keys))
        .route("/binance/server_time", post(binance_server_time))
        .route("/binance/account", post(binance_account))
        .with_state(state);

    use tokio::net::TcpListener;

    info!("Starting web UI on http://localhost:3000");
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind failed");
    axum::serve(listener, app).await.expect("server failed");
}
