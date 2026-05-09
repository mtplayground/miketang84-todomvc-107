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
                <link rel="stylesheet" href="/pkg/miketang84-todomvc-107.css"/>
                <HydrationScripts options/>
                <title>"TodoMVC"</title>
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
    view! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                <input
                    class="new-todo"
                    placeholder="What needs to be done?"
                    autofocus
                />
            </header>
            <section class="main">
                <input id="toggle-all" class="toggle-all" type="checkbox"/>
                <label for="toggle-all">"Mark all as complete"</label>
                <ul class="todo-list"></ul>
            </section>
            <footer class="footer">
                <span class="todo-count">
                    <strong>"0"</strong>
                    " item left"
                </span>
                <ul class="filters">
                    <li><a class="selected" href="#/">"All"</a></li>
                    <li><a href="#/active">"Active"</a></li>
                    <li><a href="#/completed">"Completed"</a></li>
                </ul>
                <button class="clear-completed">"Clear completed"</button>
            </footer>
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
            <p>"Created with Leptos"</p>
            <p>
                "Part of "
                <a href="http://todomvc.com">"TodoMVC"</a>
            </p>
        </footer>
    }
}
