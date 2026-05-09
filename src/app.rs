use crate::server::todos::{
    add_todo,
    clear_completed,
    delete_todo,
    edit_todo,
    list_todos,
    toggle_all,
    toggle_todo,
};
use crate::todo::{Filter, Todo};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_location,
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
    let all_completed = Signal::derive(move || {
        todo_items.with(|items| !items.is_empty() && items.iter().all(|todo| todo.completed))
    });
    let toggle_all_todos = move |ev| {
        let completed = event_target_checked(&ev);

        leptos::task::spawn_local(async move {
            if toggle_all(completed).await.is_ok() {
                set_todo_refresh.update(|value| *value += 1);
            }
        });
    };

    view! {
        <section class="todoapp">
            <Header refresh_list=set_todo_refresh/>
            <section class="main">
                <input
                    id="toggle-all"
                    class="toggle-all"
                    type="checkbox"
                    prop:checked=move || all_completed.get()
                    on:change=toggle_all_todos
                />
                <label for="toggle-all">"Mark all as complete"</label>
                <TodoList items=todo_items refresh_list=set_todo_refresh/>
            </section>
            <Footer items=todo_items refresh_list=set_todo_refresh/>
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
fn Footer(
    #[prop(into)] items: Signal<Vec<Todo>>,
    refresh_list: WriteSignal<u64>,
) -> impl IntoView {
    let location = use_location();
    let remaining_count = Signal::derive(move || {
        items.with(|todos| todos.iter().filter(|todo| !todo.completed).count())
    });
    let completed_count = Signal::derive(move || {
        items.with(|todos| todos.iter().filter(|todo| todo.completed).count())
    });
    let has_completed = Signal::derive(move || completed_count.get() > 0);
    let selected_filter = Signal::derive(move || {
        filter_from_hash(&location.hash.get())
    });

    let clear = move |_| {
        leptos::task::spawn_local(async move {
            if clear_completed().await.is_ok() {
                refresh_list.update(|value| *value += 1);
            }
        });
    };

    view! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{move || remaining_count.get().to_string()}</strong>
                {move || {
                    if remaining_count.get() == 1 {
                        " item left"
                    } else {
                        " items left"
                    }
                }}
            </span>
            <ul class="filters">
                <li>
                    <a
                        class=move || {
                            if selected_filter.get() == Filter::All {
                                "selected"
                            } else {
                                ""
                            }
                        }
                        href="#/"
                    >
                        "All"
                    </a>
                </li>
                <li>
                    <a
                        class=move || {
                            if selected_filter.get() == Filter::Active {
                                "selected"
                            } else {
                                ""
                            }
                        }
                        href="#/active"
                    >
                        "Active"
                    </a>
                </li>
                <li>
                    <a
                        class=move || {
                            if selected_filter.get() == Filter::Completed {
                                "selected"
                            } else {
                                ""
                            }
                        }
                        href="#/completed"
                    >
                        "Completed"
                    </a>
                </li>
            </ul>
            <Show when=move || has_completed.get()>
                <button class="clear-completed" on:click=clear>
                    "Clear completed"
                </button>
            </Show>
        </footer>
    }
}

fn filter_from_hash(hash: &str) -> Filter {
    match hash {
        "#/active" => Filter::Active,
        "#/completed" => Filter::Completed,
        _ => Filter::All,
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
    let todo_id = todo.id;
    let todo_completed = todo.completed;
    let original_title = todo.title.clone();
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let (editing, set_editing) = signal(false);
    let (draft, set_draft) = signal(todo.title.clone());
    let (ignore_blur, set_ignore_blur) = signal(false);

    Effect::new(move |_| {
        if editing.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
                let cursor = input.value().len() as u32;
                let _ = input.set_selection_range(cursor, cursor);
            }
        }
    });

    let toggle = move |_| {
        let id = todo_id;
        let completed = !todo_completed;

        leptos::task::spawn_local(async move {
            if toggle_todo(id, completed).await.is_ok() {
                refresh_list.update(|value| *value += 1);
            }
        });
    };

    let destroy = move |_| {
        let id = todo_id;

        leptos::task::spawn_local(async move {
            if delete_todo(id).await.is_ok() {
                refresh_list.update(|value| *value += 1);
            }
        });
    };

    let save_edit = {
        let id = todo_id;
        let set_editing = set_editing;
        let set_ignore_blur = set_ignore_blur;
        move |next_title: String| {
            leptos::task::spawn_local(async move {
                if edit_todo(id, next_title).await.is_ok() {
                    set_editing.set(false);
                    set_ignore_blur.set(false);
                    refresh_list.update(|value| *value += 1);
                }
            });
        }
    };

    let start_editing = {
        let title = original_title.clone();
        move |_| {
            set_draft.set(title.clone());
            set_ignore_blur.set(false);
            set_editing.set(true);
        }
    };

    let on_edit_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            set_ignore_blur.set(true);
            save_edit(draft.get_untracked());
        } else if ev.key() == "Escape" {
            set_ignore_blur.set(true);
            set_draft.set(original_title.clone());
            set_editing.set(false);
        }
    };

    let on_edit_blur = move |_| {
        if ignore_blur.get_untracked() {
            set_ignore_blur.set(false);
            return;
        }

        save_edit(draft.get_untracked());
    };

    let item_class = move || {
        match (todo_completed, editing.get()) {
            (true, true) => "completed editing",
            (true, false) => "completed",
            (false, true) => "editing",
            (false, false) => "",
        }
    };

    view! {
        <li class=item_class>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=todo_completed
                    on:change=toggle
                />
                <label on:dblclick=start_editing>{todo.title.clone()}</label>
                <button class="destroy" on:click=destroy></button>
            </div>
            <input
                node_ref=input_ref
                class="edit"
                prop:value=move || draft.get()
                on:input=move |ev| set_draft.set(event_target_value(&ev))
                on:keydown=on_edit_keydown
                on:blur=on_edit_blur
            />
        </li>
    }
}
