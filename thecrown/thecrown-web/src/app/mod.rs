mod components;
mod models;
mod pages;
mod server_api;

pub use models::*;
pub use server_api::*;

use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, MetaTags, Title, provide_meta_context};
use leptos_router::{components::{Route, Router, Routes}, path};

use components::{NotFoundPanel, SiteFooter, SiteHeader};
use pages::{HomePage, NetworkPage, PlayerLookupPage, PlayerRoutePage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="TheCrown"/>
        <Router>
            <div class="app-shell">
                <SiteHeader/>
                <Routes fallback=NotFoundPanel>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/network") view=NetworkPage/>
                    <Route path=path!("/player") view=PlayerLookupPage/>
                    <Route path=path!("/player/:identity") view=PlayerRoutePage/>
                </Routes>
                <SiteFooter/>
            </div>
        </Router>
    }
}

#[cfg(feature = "server")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    use leptos::prelude::{AutoReload, HydrationScripts};

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="theme-color" content="#111214"/>
                <meta name="description" content="TheCrown network status and player tools."/>
                <link rel="icon" type="image/svg+xml" href="/assets/favicon.svg"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                <HashedStylesheet options=options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
