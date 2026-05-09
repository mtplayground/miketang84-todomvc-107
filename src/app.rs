use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <title>"ZeroClaw TodoMVC"</title>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { <p>"Page not found."</p> }.into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);

    view! {
        <section>
            <h1>"Hello from ZeroClaw"</h1>
            <p>"This TodoMVC app is server-rendered with Axum and hydrates in the browser."</p>
            <button on:click=move |_| count.update(|value| *value += 1)>
                {move || format!("Hydrated clicks: {}", count.get())}
            </button>
        </section>
    }
}
