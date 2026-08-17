use leptos::prelude::*;

use crate::app::components::{ErrorPanel, InstanceCard};
use crate::app::server_api::get_network_overview;

#[component]
pub fn NetworkPage() -> impl IntoView {
    let overview = Resource::new(|| (), |_| get_network_overview());

    view! {
        <main class="page-shell">
            <header class="page-heading">
                <h1>"Network"</h1>
                <p>"Current game servers and instances."</p>
            </header>

            <Suspense fallback=move || view! {
                <div class="network-loading">
                    <div class="instance-card skeleton"></div>
                    <div class="instance-card skeleton"></div>
                    <div class="instance-card skeleton"></div>
                </div>
            }>
                {move || overview.get().map(|result| match result {
                    Ok(data) => {
                        let hub_count = data.hubs.len();
                        let parkour_count = data.parkour.len();
                        view! {
                            <div class="network-summary-bar">
                                <div><strong>{data.online_players}</strong><span>"players online"</span></div>
                                <div><strong>{data.physical_servers}</strong><span>"game servers"</span></div>
                                <div><strong>{data.active_instances}</strong><span>"instances"</span></div>
                            </div>

                            <section class="mode-section">
                                <div class="section-heading inline">
                                    <h2>"Hub"</h2>
                                    <span class="section-count">{hub_count}</span>
                                </div>
                                {if data.hubs.is_empty() {
                                    view! { <p class="empty-state">"No hub instances online."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="instance-grid">
                                            {data.hubs.into_iter().map(|entry| view! { <InstanceCard entry/> }).collect_view()}
                                        </div>
                                    }.into_any()
                                }}
                            </section>

                            <section class="mode-section">
                                <div class="section-heading inline">
                                    <h2>"Parkour"</h2>
                                    <span class="section-count">{parkour_count}</span>
                                </div>
                                {if data.parkour.is_empty() {
                                    view! { <p class="empty-state">"No parkour instances online."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="instance-grid">
                                            {data.parkour.into_iter().map(|entry| view! { <InstanceCard entry/> }).collect_view()}
                                        </div>
                                    }.into_any()
                                }}
                            </section>
                        }.into_any()
                    }
                    Err(error) => view! {
                        <ErrorPanel title="Network unavailable" message=error.to_string()/>
                    }.into_any(),
                })}
            </Suspense>
        </main>
    }
}
