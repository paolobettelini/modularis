use crate::parkour::TheCrownRuntime;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use bevy::prelude::*;
use leptos::{
    prelude::{ClassAttribute, ElementChild, GlobalAttributes, RenderHtml},
    view,
};
use server_player_registry_api::ServerPlayerRegistry;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Write,
    sync::{Arc, RwLock},
    thread,
};

const DASHBOARD_ADDRESS: &str = "127.0.0.1:8080";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct DashboardSnapshot {
    instances: Vec<InstanceSnapshot>,
    players: BTreeMap<String, PlayerSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InstanceSnapshot {
    id: u64,
    scope: String,
    players: Vec<PlayerSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerSnapshot {
    name: String,
    online: bool,
    instance_id: Option<u64>,
    last_instance_id: Option<u64>,
}

#[derive(Resource, Clone, Default)]
pub(crate) struct TheCrownDashboard {
    snapshot: Arc<RwLock<DashboardSnapshot>>,
}

impl TheCrownDashboard {
    fn read(&self) -> DashboardSnapshot {
        self.snapshot
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn update(&self, instances: Vec<InstanceSnapshot>, online: Vec<PlayerSnapshot>) {
        let mut snapshot = self
            .snapshot
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        for player in snapshot.players.values_mut() {
            player.online = false;
            player.instance_id = None;
        }
        for player in online {
            snapshot.players.insert(player.name.clone(), player);
        }
        snapshot.instances = instances;
    }
}

pub(crate) fn sync_dashboard(
    dashboard: Res<TheCrownDashboard>,
    runtime: Res<TheCrownRuntime>,
    registry: Res<ServerPlayerRegistry>,
) {
    let registered = registry.players();
    let registered_by_id = registered
        .iter()
        .map(|player| (player.id, player))
        .collect::<HashMap<_, _>>();
    let mut players_by_instance = runtime
        .instances
        .iter()
        .map(|instance| (instance.id, Vec::new()))
        .collect::<BTreeMap<_, Vec<PlayerSnapshot>>>();
    let mut online = Vec::new();

    for (player_id, arena) in &runtime.players {
        let Some(player) = registered_by_id.get(player_id) else {
            continue;
        };
        let snapshot = PlayerSnapshot {
            name: player.name.clone(),
            online: true,
            instance_id: Some(arena.instance_id),
            last_instance_id: Some(arena.instance_id),
        };
        players_by_instance
            .entry(arena.instance_id)
            .or_default()
            .push(snapshot.clone());
        online.push(snapshot);
    }

    for players in players_by_instance.values_mut() {
        players.sort_by(|left, right| left.name.cmp(&right.name));
    }
    online.sort_by(|left, right| left.name.cmp(&right.name));

    let instances = runtime
        .instances
        .iter()
        .map(|instance| InstanceSnapshot {
            id: instance.id,
            scope: instance.scope.0.clone(),
            players: players_by_instance.remove(&instance.id).unwrap_or_default(),
        })
        .collect();
    dashboard.update(instances, online);
}

pub(crate) fn spawn_dashboard_server(dashboard: TheCrownDashboard) {
    thread::Builder::new()
        .name("thecrown-dashboard".to_string())
        .spawn(move || {
            let result = actix_web::rt::System::new().block_on(async move {
                let shared = web::Data::new(dashboard);
                let server = HttpServer::new(move || {
                    App::new()
                        .app_data(shared.clone())
                        .route("/", web::get().to(dashboard_page))
                        .route("/player/{name}", web::get().to(player_page))
                })
                .bind(DASHBOARD_ADDRESS)?;
                info!("TheCrown dashboard listening at http://{DASHBOARD_ADDRESS}");
                server.run().await
            });
            match result {
                Ok(()) => info!("TheCrown dashboard stopped"),
                Err(error) => {
                    error!("TheCrown dashboard failed at http://{DASHBOARD_ADDRESS}: {error}")
                }
            }
        })
        .expect("failed to spawn the TheCrown dashboard thread");
}

async fn dashboard_page(dashboard: web::Data<TheCrownDashboard>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_dashboard(dashboard.read()))
}

async fn player_page(
    name: web::Path<String>,
    dashboard: web::Data<TheCrownDashboard>,
) -> impl Responder {
    let name = name.into_inner();
    let snapshot = dashboard.read();
    let player = snapshot.players.get(&name).cloned().or_else(|| {
        snapshot
            .players
            .values()
            .find(|player| player.name.eq_ignore_ascii_case(&name))
            .cloned()
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_player(&name, player))
}

fn render_dashboard(snapshot: DashboardSnapshot) -> String {
    let online_count = snapshot
        .players
        .values()
        .filter(|player| player.online)
        .count();
    let instance_count = snapshot.instances.len();
    let cards = snapshot
        .instances
        .into_iter()
        .map(|instance| {
            let player_count = instance.players.len();
            let players = instance
                .players
                .into_iter()
                .map(|player| {
                    let href = format!("/player/{}", encode_path_segment(&player.name));
                    view! {
                        <li class="player-row">
                            <a href=href>
                                <span class="online-dot"></span>
                                <span>{player.name}</span>
                            </a>
                        </li>
                    }
                })
                .collect::<Vec<_>>();
            let empty = (player_count == 0).then(|| {
                view! { <p class="empty-state">"No players in this instance."</p> }
            });
            view! {
                <article class="instance-card">
                    <div class="instance-heading">
                        <div>
                            <p class="eyebrow">"Parkour instance"</p>
                            <h2>{format!("parkour-{}", instance.id)}</h2>
                        </div>
                        <span class="count-badge">{player_count}</span>
                    </div>
                    <p class="scope-id">{instance.scope}</p>
                    {empty}
                    <ul class="player-list">{players}</ul>
                </article>
            }
        })
        .collect::<Vec<_>>();

    let view = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta http-equiv="refresh" content="2"/>
                <title>"TheCrown server dashboard"</title>
                <style>{DASHBOARD_CSS}</style>
            </head>
            <body>
                <main class="page-shell">
                    <header class="hero">
                        <div>
                            <p class="eyebrow accent">"Development dashboard"</p>
                            <h1>"TheCrown"</h1>
                            <p class="subtitle">
                                "Live parkour instances and connected players."
                            </p>
                        </div>
                        <div class="summary">
                            <div><strong>{online_count}</strong><span>"players online"</span></div>
                            <div><strong>{instance_count}</strong><span>"active instances"</span></div>
                        </div>
                    </header>
                    <section class="instance-grid">{cards}</section>
                </main>
            </body>
        </html>
    };
    format!("<!doctype html>{}", view.to_html())
}

fn render_player(requested_name: &str, player: Option<PlayerSnapshot>) -> String {
    let (name, status, status_class, details) = match player {
        Some(player) if player.online => {
            let instance = player
                .instance_id
                .map(|id| format!("parkour-{id}"))
                .unwrap_or_else(|| "assignment pending".to_string());
            (
                player.name,
                "Online",
                "status online",
                format!("Currently assigned to {instance}."),
            )
        }
        Some(player) => {
            let details = player
                .last_instance_id
                .map(|id| format!("Last seen in parkour-{id}."))
                .unwrap_or_else(|| "No previous instance is recorded.".to_string());
            (player.name, "Offline", "status offline", details)
        }
        None => (
            requested_name.to_string(),
            "Offline",
            "status offline",
            "This player has not been seen since the server started.".to_string(),
        ),
    };

    let view = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta http-equiv="refresh" content="2"/>
                <title>{format!("{name} · TheCrown")}</title>
                <style>{DASHBOARD_CSS}</style>
            </head>
            <body>
                <main class="page-shell player-page">
                    <a class="back-link" href="/">"← Back to instances"</a>
                    <article class="player-card">
                        <p class="eyebrow">"Player"</p>
                        <h1>{name}</h1>
                        <span class=status_class>{status}</span>
                        <p class="player-details">{details}</p>
                    </article>
                </main>
            </body>
        </html>
    };
    format!("<!doctype html>{}", view.to_html())
}

fn encode_path_segment(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            write!(encoded, "%{byte:02X}").expect("writing to a String cannot fail");
        }
    }
    encoded
}

const DASHBOARD_CSS: &str = r#"
:root {
    color-scheme: dark;
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: #090b12;
    color: #f4f1ff;
}
* { box-sizing: border-box; }
body {
    margin: 0;
    min-height: 100vh;
    background:
        radial-gradient(circle at 10% 0%, rgba(122, 92, 255, .20), transparent 34rem),
        radial-gradient(circle at 90% 10%, rgba(65, 214, 166, .10), transparent 30rem),
        #090b12;
}
a { color: inherit; }
.page-shell {
    width: min(1180px, calc(100% - 40px));
    margin: 0 auto;
    padding: 64px 0 80px;
}
.hero {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 32px;
    margin-bottom: 42px;
}
.eyebrow {
    margin: 0 0 9px;
    color: #aaa4bb;
    font-size: .72rem;
    font-weight: 800;
    letter-spacing: .15em;
    text-transform: uppercase;
}
.eyebrow.accent { color: #9d89ff; }
h1, h2, p { margin-top: 0; }
h1 {
    margin-bottom: 10px;
    font-size: clamp(2.8rem, 7vw, 5.6rem);
    line-height: .92;
    letter-spacing: -.06em;
}
h2 { margin-bottom: 0; font-size: 1.35rem; }
.subtitle { margin-bottom: 0; color: #aaa4bb; font-size: 1.05rem; }
.summary {
    display: flex;
    gap: 10px;
}
.summary div {
    min-width: 150px;
    padding: 18px 20px;
    border: 1px solid #282431;
    border-radius: 16px;
    background: rgba(17, 18, 27, .80);
}
.summary strong, .summary span { display: block; }
.summary strong { font-size: 1.55rem; }
.summary span { margin-top: 3px; color: #888293; font-size: .78rem; }
.instance-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(290px, 1fr));
    gap: 16px;
}
.instance-card, .player-card {
    border: 1px solid #282431;
    border-radius: 20px;
    background: linear-gradient(145deg, rgba(24, 22, 35, .96), rgba(14, 15, 23, .96));
    box-shadow: 0 18px 50px rgba(0, 0, 0, .20);
}
.instance-card { padding: 24px; }
.instance-heading {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 18px;
}
.count-badge {
    display: grid;
    width: 38px;
    height: 38px;
    place-items: center;
    border-radius: 12px;
    background: #7a5cff;
    font-weight: 800;
}
.scope-id {
    margin: 12px 0 19px;
    color: #6f697a;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: .73rem;
    overflow-wrap: anywhere;
}
.player-list { display: grid; gap: 8px; margin: 0; padding: 0; list-style: none; }
.player-row a {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 11px 12px;
    border: 1px solid transparent;
    border-radius: 11px;
    background: rgba(255, 255, 255, .035);
    text-decoration: none;
    transition: border-color .15s, background .15s, transform .15s;
}
.player-row a:hover {
    border-color: rgba(157, 137, 255, .6);
    background: rgba(122, 92, 255, .12);
    transform: translateY(-1px);
}
.online-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #48d7a4;
    box-shadow: 0 0 12px rgba(72, 215, 164, .65);
}
.empty-state { margin: 0; color: #6f697a; font-size: .88rem; }
.player-page { width: min(760px, calc(100% - 40px)); }
.back-link {
    display: inline-block;
    margin-bottom: 22px;
    color: #a998ff;
    font-weight: 700;
    text-decoration: none;
}
.player-card { padding: clamp(28px, 6vw, 58px); }
.player-card h1 { margin-bottom: 24px; font-size: clamp(2.4rem, 8vw, 5rem); }
.status {
    display: inline-flex;
    align-items: center;
    padding: 8px 12px;
    border-radius: 999px;
    font-size: .8rem;
    font-weight: 850;
    letter-spacing: .05em;
    text-transform: uppercase;
}
.status.online { background: rgba(72, 215, 164, .14); color: #62e2b4; }
.status.offline { background: rgba(255, 100, 118, .12); color: #ff7b8b; }
.player-details { margin: 24px 0 0; color: #aaa4bb; font-size: 1.05rem; }
@media (max-width: 720px) {
    .page-shell { width: min(100% - 24px, 1180px); padding-top: 36px; }
    .hero { align-items: stretch; flex-direction: column; }
    .summary { display: grid; grid-template-columns: 1fr 1fr; }
    .summary div { min-width: 0; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_names_are_safe_path_segments() {
        assert_eq!(encode_path_segment("Player 1/β"), "Player%201%2F%CE%B2");
    }

    #[test]
    fn unknown_player_page_is_offline() {
        let page = render_player("Nobody", None);
        assert!(page.contains("Nobody"));
        assert!(page.contains("Offline"));
    }
}
