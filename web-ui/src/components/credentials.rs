use leptos::*;
use crate::endpoints::Venue;
use crate::extension_api::{VenueCredentials, store_credentials, get_credentials, delete_credentials as delete_creds};

#[component]
pub fn CredentialsPanel() -> impl IntoView {
    let (selected_venue, set_selected_venue) = create_signal(Venue::Coinbase);
    let (api_key, set_api_key) = create_signal(String::new());
    let (api_secret, set_api_secret) = create_signal(String::new());
    let (passphrase, set_passphrase) = create_signal(String::new());
    let (status_message, set_status_message) = create_signal(String::new());

    // Load credentials when venue changes
    create_effect(move |_| {
        let venue = selected_venue.get();
        wasm_bindgen_futures::spawn_local(async move {
            match get_credentials(venue).await {
                Ok(Some(creds)) => {
                    set_api_key.set(creds.api_key.unwrap_or_default());
                    set_api_secret.set(creds.api_secret.unwrap_or_default());
                    set_passphrase.set(creds.passphrase.unwrap_or_default());
                }
                _ => {
                    set_api_key.set(String::new());
                    set_api_secret.set(String::new());
                    set_passphrase.set(String::new());
                }
            }
        });
        set_status_message.set(String::new());
    });

    let save_handler = move |_| {
        let venue = selected_venue.get();
        let credentials = VenueCredentials {
            api_key: if !api_key.get().is_empty() { Some(api_key.get()) } else { None },
            api_secret: if !api_secret.get().is_empty() { Some(api_secret.get()) } else { None },
            passphrase: if !passphrase.get().is_empty() { Some(passphrase.get()) } else { None },
        };

        wasm_bindgen_futures::spawn_local(async move {
            match store_credentials(venue, credentials).await {
                Ok(_) => set_status_message.set("✓ Credentials saved successfully".to_string()),
                Err(e) => set_status_message.set(format!("✗ Failed to save: {}", e)),
            }
        });
    };

    let delete_handler = move |_| {
        let venue = selected_venue.get();
        wasm_bindgen_futures::spawn_local(async move {
            match delete_creds(venue).await {
                Ok(_) => {
                    set_api_key.set(String::new());
                    set_api_secret.set(String::new());
                    set_passphrase.set(String::new());
                    set_status_message.set("✓ Credentials deleted".to_string());
                },
                Err(e) => set_status_message.set(format!("✗ Failed to delete: {}", e)),
            }
        });
    };

    view! {
        <div class="panel">
            <h2>"API Credentials"</h2>
            <p style="margin-bottom: 16px; color: #6b7280;">
                "Manage your exchange API credentials. In the Chrome extension, they are stored securely using Chrome's storage API."
            </p>

            <div class="form-group">
                <label>"Exchange"</label>
                <select on:change=move |ev| {
                    let value = event_target_value(&ev);
                    let venue = match value.as_str() {
                        "BinanceSpot" => Venue::BinanceSpot,
                        "BinanceUsdm" => Venue::BinanceUsdm,
                        "BinanceCoinm" => Venue::BinanceCoinm,
                        "Coinbase" => Venue::Coinbase,
                        "Deribit" => Venue::Deribit,
                        "OKX" => Venue::OKX,
                        "CryptoCom" => Venue::CryptoCom,
                        "BingX" => Venue::BingX,
                        "Bitmart" => Venue::Bitmart,
                        _ => Venue::Coinbase,
                    };
                    set_selected_venue.set(venue);
                }>
                    {Venue::all().into_iter().map(|venue| {
                        let venue_name = format!("{:?}", venue);
                        let display_name = venue.display_name();
                        view! {
                            <option value=venue_name.clone()>{display_name}</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
            </div>

            <div class="form-group">
                <label>"API Key"</label>
                <input 
                    type="text"
                    placeholder="Enter your API key"
                    prop:value=move || api_key.get()
                    on:input=move |ev| set_api_key.set(event_target_value(&ev))
                />
            </div>

            <div class="form-group">
                <label>"API Secret"</label>
                <input 
                    type="password"
                    placeholder="Enter your API secret"
                    prop:value=move || api_secret.get()
                    on:input=move |ev| set_api_secret.set(event_target_value(&ev))
                />
            </div>

            <div class="form-group">
                <label>"Passphrase (optional)"</label>
                <input 
                    type="password"
                    placeholder="Enter your passphrase (if required by exchange)"
                    prop:value=move || passphrase.get()
                    on:input=move |ev| set_passphrase.set(event_target_value(&ev))
                />
            </div>

            <div style="display: flex; gap: 8px; margin-bottom: 16px;">
                <button on:click=save_handler>"Save Credentials"</button>
                <button 
                    on:click=delete_handler 
                    style="background: #dc2626;"
                    onmouseover="this.style.background='#b91c1c'"
                    onmouseout="this.style.background='#dc2626'"
                >
                    "Delete"
                </button>
            </div>

            {move || {
                let message = status_message.get();
                if !message.is_empty() {
                    let class = if message.starts_with("✓") { "success" } else { "error" };
                    view! {
                        <div class=format!("status-message {}", class)>
                            {message}
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}
        </div>
    }
}