use crate::server::todos::{add_todo, delete_todo, list_todos, toggle_todo};
use crate::todo::Todo;
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
    let (todo_refresh, set_todo_refresh) = signal(0_u64);
    let todos = Resource::new(move || todo_refresh.get(), |_| list_todos(None));
    let todo_items = Signal::derive(move || {
        todos.get()
            .and_then(Result::ok)
            .unwrap_or_default()
    });
    let todo_count = move || {
        todo_items.with(|items| items.len())
    };

    view! {
        <section class="todoapp">
            <Header refresh_list=set_todo_refresh/>
            <section class="main">
                <input id="toggle-all" class="toggle-all" type="checkbox"/>
                <label for="toggle-all">"Mark all as complete"</label>
                <TodoList items=todo_items refresh_list=set_todo_refresh/>
            </section>
            <footer class="footer">
                <span class="todo-count">
                    <strong>{move || todo_count().to_string()}</strong>
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

#[component]
fn Header(refresh_list: WriteSignal<u64>) -> impl IntoView {
    let (title, set_title) = signal(String::new());

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() != "Enter" {
            return;
        }

        let next_title = title.get_untracked();
        let set_title = set_title;

        leptos::task::spawn_local(async move {
            if add_todo(next_title).await.is_ok() {
                set_title.set(String::new());
                refresh_list.update(|value| *value += 1);
            }
        });
    };

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                autofocus
                prop:value=move || title.get()
                on:input=move |ev| set_title.set(event_target_value(&ev))
                on:keydown=on_keydown
            />
        </header>
    }
}

#[component]
fn TodoList(
    #[prop(into)] items: Signal<Vec<Todo>>,
    refresh_list: WriteSignal<u64>,
) -> impl IntoView {
    view! {
        <ul class="todo-list">
            <For
                each=move || items.get()
                key=|todo| todo.id
                let:todo
            >
                <TodoItem todo refresh_list=refresh_list/>
            </For>
        </ul>
    }
}

#[component]
fn TodoItem(todo: Todo, refresh_list: WriteSignal<u64>) -> impl IntoView {
    let toggle = {
        let todo = todo.clone();
        move |_| {
            let id = todo.id;
            let completed = !todo.completed;

            leptos::task::spawn_local(async move {
                if toggle_todo(id, completed).await.is_ok() {
                    refresh_list.update(|value| *value += 1);
                }
            });
        }
    };

    let destroy = move |_| {
        let id = todo.id;

        leptos::task::spawn_local(async move {
            if delete_todo(id).await.is_ok() {
                refresh_list.update(|value| *value += 1);
            }
        });
    };

    let item_class = if todo.completed { "completed" } else { "" };

    view! {
        <li class=item_class>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=todo.completed
                    on:change=toggle
                />
                <label>{todo.title}</label>
                <button class="destroy" on:click=destroy></button>
            </div>
        </li>
    }
}
