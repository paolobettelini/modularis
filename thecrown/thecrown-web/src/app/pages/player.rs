use leptos::form::ActionForm;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_params_map, use_query_map};
use thecrown_protocol::PlayerStatus;

use crate::app::components::{ErrorPanel, SearchBox, StatusCard};
use crate::app::models::PlayerProfile;
use crate::app::server_api::{RequestTransferToInstance, get_player_profile};

#[component]
pub fn PlayerLookupPage() -> impl IntoView {
    let query = use_query_map();
    let profile = Resource::new(
        move || query.read().get("q").unwrap_or_default(),
        get_player_profile,
    );

    view! {
        <main class="page-shell player-directory-page">
            <header class="page-heading compact-heading">
                <h1>"Players"</h1>
                <p>"Search by username or UUID."</p>
            </header>
            <SearchBox compact=false/>

            {move || {
                let identity = query.read().get("q").unwrap_or_default();
                if identity.trim().is_empty() {
                    return view! {
                        <section class="directory-placeholder">
                            <p>"Enter a username or UUID to open a profile."</p>
                        </section>
                    }.into_any();
                }

                view! {
                    <Suspense fallback=move || view! { <div class="profile-card skeleton profile-skeleton"></div> }>
                        {move || profile.get().map(|result| match result {
                            Ok(Some(player)) => view! { <PlayerProfileCard player/> }.into_any(),
                            Ok(None) => view! {
                                <section class="notice-panel">
                                    <h2>"Player not found"</h2>
                                    <p>"No matching player is stored in TheCrown."</p>
                                </section>
                            }.into_any(),
                            Err(error) => view! {
                                <ErrorPanel title="Player lookup failed" message=error.to_string()/>
                            }.into_any(),
                        })}
                    </Suspense>
                }.into_any()
            }}
        </main>
    }
}

#[component]
pub fn PlayerRoutePage() -> impl IntoView {
    let params = use_params_map();
    let profile = Resource::new(
        move || params.read().get("identity").unwrap_or_default(),
        get_player_profile,
    );

    view! {
        <main class="page-shell player-profile-page">
            <A href="/player" attr:class="back-link">"← Players"</A>
            <Suspense fallback=move || view! { <div class="profile-card skeleton profile-skeleton"></div> }>
                {move || profile.get().map(|result| match result {
                    Ok(Some(player)) => view! { <PlayerProfileCard player/> }.into_any(),
                    Ok(None) => view! {
                        <section class="notice-panel">
                            <h1>"Player not found"</h1>
                        </section>
                    }.into_any(),
                    Err(error) => view! {
                        <ErrorPanel title="Player profile unavailable" message=error.to_string()/>
                    }.into_any(),
                })}
            </Suspense>
        </main>
    }
}

#[component]
fn PlayerProfileCard(player: PlayerProfile) -> impl IntoView {
    let transfer_action = ServerAction::<RequestTransferToInstance>::new();
    let transfer_value = transfer_action.value();
    let current_instance = match &player.status {
        PlayerStatus::Online { instance_id, .. } => Some(instance_id.clone()),
        PlayerStatus::Offline => None,
    };

    view! {
        <article class="profile-card">
            <div class="profile-hero">
                <div class="avatar-block" aria-hidden="true">
                    {player.username.chars().next().unwrap_or('?').to_uppercase().to_string()}
                </div>
                <div class="profile-title">
                    <h1>{player.username.clone()}</h1>
                    <code>{player.uuid.clone()}</code>
                </div>
                <div class="record-card">
                    <span>"Parkour record"</span>
                    <strong>{player.parkour_record}</strong>
                </div>
            </div>

            <StatusCard status=player.status.clone()/>

            {current_instance.map(|instance_id| {
                let displayed_instance = instance_id.clone();
                view! {
                    <section class="transfer-panel">
                        <div>
                            <h2>"Join instance"</h2>
                            <p>{format!("Send your in-game player to {displayed_instance}.")}</p>
                        </div>
                        <ActionForm action=transfer_action>
                            <input type="hidden" name="instance_id" value=instance_id/>
                            <button class="button secondary" type="submit">"Join"</button>
                        </ActionForm>
                        <div class="action-result" aria-live="polite">
                            {move || transfer_value.get().map(|result| match result {
                                Ok(result) if result.accepted => view! {
                                    <p class="action-message success">{result.message}</p>
                                }.into_any(),
                                Ok(result) => view! {
                                    <p class="action-message error">{result.message}</p>
                                }.into_any(),
                                Err(error) => view! {
                                    <p class="action-message error">{error.to_string()}</p>
                                }.into_any(),
                            })}
                        </div>
                    </section>
                }
            })}
        </article>
    }
}
