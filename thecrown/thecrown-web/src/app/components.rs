use leptos::prelude::*;
use leptos_router::components::A;
use thecrown_protocol::{GameMode, PlayerStatus, ServerEntry};

use super::server_api::{entry_mode_label, status_class, status_label};

#[component]
pub fn SiteHeader() -> impl IntoView {
    let viewer = Resource::new(|| (), |_| super::server_api::get_current_viewer());
    view! {
        <header class="site-header">
            <div class="nav-shell">
                <A href="/" attr:class="brand" attr:aria-label="TheCrown home">
                    <strong>"TheCrown"</strong>
                </A>

                <nav class="main-nav" aria-label="Main navigation">
                    <A href="/">"Overview"</A>
                    <A href="/network">"Network"</A>
                    <A href="/player">"Players"</A>
                </nav>

                <div class="viewer-slot">
                    <Suspense fallback=move || view! { <span class="nav-status muted">"…"</span> }>
                        {move || {
                            viewer.get().map(|result| match result {
                                Ok(Some(player)) => view! {
                                    <A href=format!("/player/{}", player.uuid) attr:class="viewer-link">
                                        <span class="online-dot"></span>
                                        <span>{player.username}</span>
                                    </A>
                                }.into_any(),
                                Ok(None) => view! {
                                    <span class="nav-status">"/weblogin in-game"</span>
                                }.into_any(),
                                Err(_) => view! {
                                    <span class="nav-status error">"session unavailable"</span>
                                }.into_any(),
                            })
                        }}
                    </Suspense>
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="footer-shell">
                <span>"TheCrown"</span>
            </div>
        </footer>
    }
}

#[component]
pub fn SearchBox(compact: bool) -> impl IntoView {
    let class = if compact { "player-search compact" } else { "player-search" };
    view! {
        <form class=class action="/player" method="get">
            <label for="player-search">"Player"</label>
            <div class="search-row">
                <input
                    id="player-search"
                    name="q"
                    type="search"
                    maxlength="128"
                    autocomplete="off"
                    placeholder="Username or UUID"
                />
                <button class="button primary" type="submit">"Search"</button>
            </div>
        </form>
    }
}

#[component]
pub fn InstanceCard(entry: ServerEntry) -> impl IntoView {
    let mode = entry_mode_label(&entry);
    let mode_class = match entry.mode {
        GameMode::Hub => "mode-chip hub",
        GameMode::Parkour => "mode-chip parkour",
    };

    view! {
        <article class="instance-card">
            <div class="instance-card-top">
                <span class=mode_class>{mode}</span>
                <span class="instance-count">{entry.online}</span>
            </div>
            <h3>{entry.instance_id}</h3>
            <div class="instance-meta">
                <div>
                    <span>"Server"</span>
                    <strong>{entry.server_id}</strong>
                </div>
                <div>
                    <span>"Players"</span>
                    <strong>{entry.online}</strong>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn StatusCard(status: PlayerStatus) -> impl IntoView {
    let label = status_label(&status);
    let class = status_class(&status);
    let details = match &status {
        PlayerStatus::Offline => view! {
            <p class="status-details">"Not currently online."</p>
        }.into_any(),
        PlayerStatus::Online {
            server_id,
            instance_id,
            mode,
        } => {
            let mode = match mode {
                GameMode::Hub => "Hub",
                GameMode::Parkour => "Parkour",
            };
            view! {
                <div class="status-grid">
                    <div><span>"Instance"</span><strong>{instance_id.clone()}</strong></div>
                    <div><span>"Mode"</span><strong>{mode}</strong></div>
                    <div><span>"Server"</span><strong>{server_id.clone()}</strong></div>
                </div>
            }.into_any()
        }
    };

    view! {
        <section class="profile-status-panel">
            <div class="section-heading inline">
                <h2>"Status"</h2>
                <span class=class>{label}</span>
            </div>
            {details}
        </section>
    }
}

#[component]
pub fn ErrorPanel(title: &'static str, message: String) -> impl IntoView {
    view! {
        <section class="notice-panel error-panel">
            <h2>{title}</h2>
            <p>{message}</p>
        </section>
    }
}

#[component]
pub fn NotFoundPanel() -> impl IntoView {
    view! {
        <main class="page-shell narrow-page">
            <section class="notice-panel">
                <h1>"404"</h1>
                <p>"Nothing here."</p>
                <A href="/" attr:class="button secondary">"Home"</A>
            </section>
        </main>
    }
}
