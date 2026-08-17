use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="page-shell home-page">
            <section class="home-intro">
                <p class="eyebrow">"Modularis network"</p>
                <h1>"TheCrown"</h1>
            </section>
        </main>
    }
}
