use leptos::*;
use crate::endpoints::Venue;
use crate::extension_api::{VenueCredentials, store_credentials, get_credentials, delete_credentials as delete_creds};

#[derive(Clone)]
struct StoredAccount {
    venue: Venue,
    has_credentials: bool,
}

#[component]
pub fn CredentialsPanel() -> impl IntoView {
    let (accounts, set_accounts) = create_signal(Vec::<StoredAccount>::new());
    let (show_modal, set_show_modal) = create_signal(false);
    let (modal_venue, set_modal_venue) = create_signal(Venue::Coinbase);
    let (api_key, set_api_key) = create_signal(String::new());
    let (api_secret, set_api_secret) = create_signal(String::new());
    let (passphrase, set_passphrase) = create_signal(String::new());
    let (status_message, set_status_message) = create_signal(String::new());

    // Load all accounts on component mount
    create_effect(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let mut stored_accounts = Vec::new();
            for venue in Venue::all() {
                let has_creds = match get_credentials(venue).await {
                    Ok(Some(_)) => true,
                    _ => false,
                };
                stored_accounts.push(StoredAccount {
                    venue,
                    has_credentials: has_creds,
                });
            }
            set_accounts.set(stored_accounts);
        });
    });

    let open_modal = move |venue: Venue| {
        set_modal_venue.set(venue);
        set_api_key.set(String::new());
        set_api_secret.set(String::new());
        set_passphrase.set(String::new());
        set_status_message.set(String::new());
        
        // Load existing credentials if they exist
        wasm_bindgen_futures::spawn_local(async move {
            match get_credentials(venue).await {
                Ok(Some(creds)) => {
                    set_api_key.set(creds.api_key.unwrap_or_default());
                    set_api_secret.set(creds.api_secret.unwrap_or_default());
                    set_passphrase.set(creds.passphrase.unwrap_or_default());
                }
                _ => {}
            }
        });
        
        set_show_modal.set(true);
    };

    let close_modal = move |_| {
        set_show_modal.set(false);
    };

    let save_handler = move |_| {
        let venue = modal_venue.get();
        let credentials = VenueCredentials {
            api_key: if !api_key.get().is_empty() { Some(api_key.get()) } else { None },
            api_secret: if !api_secret.get().is_empty() { Some(api_secret.get()) } else { None },
            passphrase: if !passphrase.get().is_empty() { Some(passphrase.get()) } else { None },
        };

        wasm_bindgen_futures::spawn_local(async move {
            match store_credentials(venue, credentials).await {
                Ok(_) => {
                    set_status_message.set("✓ Credentials saved successfully".to_string());
                    // Refresh accounts list
                    let mut stored_accounts = Vec::new();
                    for venue in Venue::all() {
                        let has_creds = match get_credentials(venue).await {
                            Ok(Some(_)) => true,
                            _ => false,
                        };
                        stored_accounts.push(StoredAccount {
                            venue,
                            has_credentials: has_creds,
                        });
                    }
                    set_accounts.set(stored_accounts);
                },
                Err(e) => set_status_message.set(format!("✗ Failed to save: {}", e)),
            }
        });
    };

    let delete_handler = move |venue: Venue| {
        wasm_bindgen_futures::spawn_local(async move {
            match delete_creds(venue).await {
                Ok(_) => {
                    // Refresh accounts list
                    let mut stored_accounts = Vec::new();
                    for venue in Venue::all() {
                        let has_creds = match get_credentials(venue).await {
                            Ok(Some(_)) => true,
                            _ => false,
                        };
                        stored_accounts.push(StoredAccount {
                            venue,
                            has_credentials: has_creds,
                        });
                    }
                    set_accounts.set(stored_accounts);
                },
                Err(_) => {}
            }
        });
    };

    view! {
        <div class="panel">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                <h2>"Account Management"</h2>
                <button 
                    on:click=move |_| open_modal(Venue::Coinbase)
                    style="background: #059669; font-size: 14px; padding: 8px 16px;"
                >
                    "+ Add Account"
                </button>
            </div>
            
            <p style="margin-bottom: 20px; color: #6b7280;">
                "Manage your exchange API credentials. Credentials are stored securely using Chrome's storage API."
            </p>

            <div class="accounts-list">
                {move || {
                    let account_list = accounts.get();
                    if account_list.is_empty() {
                        view! {
                            <div style="text-align: center; color: #6b7280; padding: 40px 20px;">
                                "No accounts configured yet. Click \"Add Account\" to get started."
                            </div>
                        }.into_view()
                    } else {
                        account_list.into_iter().map(|account| {
                            let venue_copy = account.venue;
                            let venue_copy2 = account.venue;
                            view! {
                                <div class="account-item">
                                    <div class="account-info">
                                        <span class="venue-name">{account.venue.display_name()}</span>
                                        <span class=format!("status-indicator {}", if account.has_credentials { "configured" } else { "not-configured" })>
                                            {if account.has_credentials { "✓ Configured" } else { "Not configured" }}
                                        </span>
                                    </div>
                                    <div class="account-actions">
                                        <button 
                                            on:click=move |_| open_modal(venue_copy)
                                            style="background: #3b82f6; font-size: 12px; padding: 6px 12px;"
                                        >
                                            {if account.has_credentials { "Edit" } else { "Configure" }}
                                        </button>
                                        {if account.has_credentials {
                                            view! {
                                                <button 
                                                    on:click=move |_| delete_handler(venue_copy2)
                                                    style="background: #dc2626; font-size: 12px; padding: 6px 12px; margin-left: 8px;"
                                                >
                                                    "Delete"
                                                </button>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>().into_view()
                    }
                }}
            </div>

            // Modal
            {move || {
                if show_modal.get() {
                    view! {
                        <div class="modal-overlay" on:click=close_modal>
                            <div class="modal-content" on:click=|ev| ev.stop_propagation()>
                                <div class="modal-header">
                                    <h3>{format!("Configure {}", modal_venue.get().display_name())}</h3>
                                    <button class="close-button" on:click=close_modal>"×"</button>
                                </div>
                                
                                <div class="modal-body">
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
                                            set_modal_venue.set(venue);
                                        }>
                                            {Venue::all().into_iter().map(|venue| {
                                                let venue_name = format!("{:?}", venue);
                                                let display_name = venue.display_name();
                                                let is_selected = venue == modal_venue.get();
                                                view! {
                                                    <option value=venue_name.clone() selected=is_selected>{display_name}</option>
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

                                <div class="modal-footer">
                                    <button on:click=close_modal style="background: #6b7280;">
                                        "Cancel"
                                    </button>
                                    <button on:click=save_handler style="margin-left: 8px;">
                                        "Save Credentials"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}
        </div>
    }
}