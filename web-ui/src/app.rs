use leptos::*;
use crate::components::{CredentialsPanel, EndpointPanel};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <Header/>
            <MainInterface/>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="header">
            <h1>"CCRXT API Explorer"</h1>
            <p>"Test cryptocurrency exchange API endpoints with a simple web interface"</p>
        </div>
    }
}

#[component]
fn MainInterface() -> impl IntoView {
    view! {
        <div class="main-grid">
            <CredentialsPanel/>
            <EndpointPanel/>
        </div>
    }
}