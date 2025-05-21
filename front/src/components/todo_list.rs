use crate::store::todo_items::{
    StatusStoreFields, Todo, TodoStoreFields, Todos, TodosStoreFields, User, UserStoreFields,
};
use chrono::NaiveDate;
use leptos::{logging::warn, prelude::*};
use reactive_stores::{Field, Patch, Store};

#[component]
pub fn TodoList() -> impl IntoView {
    let store = use_context::<Store<Todos>>().expect("TodoStore should be provided");

    let input_ref = NodeRef::new();

    view! {
        <div>
            <p>"Hello, " {move || store.user().name().get()}</p>
            <UserForm user=store.user()/>
            <hr/>
            <form on:submit=move |ev| {
                ev.prevent_default();
                if let Some(input) = input_ref.get() {
                    let new_todo = Todo::new(input.value());
                    store.todos().write().push(new_todo);
                }
            }>
                <label>"Add a Todo" <input type="text" node_ref=input_ref/></label>
                <input type="submit"/>
            </form>
            <ol>
                // because `todos` is a keyed field, `store.todos()` returns a struct that
                // directly implements IntoIterator, so we can use it in <For/> and
                // it will manage reactivity for the store fields correctly
                <For
                    each=move || store.todos()
                    key=|row| row.id().get()
                    let:todo
                >
                    <TodoRow store todo/>
                </For>

            </ol>
            <pre>{move || serde_json::to_string_pretty(&*store.read())}</pre>
        </div>
    }
}

#[component]
fn UserForm(#[prop(into)] user: Field<User>) -> impl IntoView {
    let error = RwSignal::new(None);

    view! {
        {move || error.get().map(|n| view! { <p>{n}</p> })}
        <form on:submit:target=move |ev| {
            ev.prevent_default();
            match User::from_event(&ev) {
                Ok(new_user) => {
                    error.set(None);
                    user.patch(new_user);
                }
                Err(e) => error.set(Some(e.to_string())),
            }
        }>
            <label>
                "Name" <input type="text" name="name" prop:value=move || user.name().get()/>
            </label>
            <label>
                "Email" <input type="email" name="email" prop:value=move || user.email().get()/>
            </label>
            <input type="submit"/>
        </form>
    }
}

#[component]
fn TodoRow(store: Store<Todos>, #[prop(into)] todo: Field<Todo>) -> impl IntoView {
    let status = todo.status();
    let title = todo.label();

    let editing = RwSignal::new(true);

    view! {
        <li style:text-decoration=move || {
            status.done().then_some("line-through").unwrap_or_default()
        }>

            <p
                class:hidden=move || editing.get()
                on:click=move |_| {
                    editing.update(|n| *n = !*n);
                }
            >

                {move || title.get()}
            </p>
            <input
                class:hidden=move || !editing.get()
                type="text"
                prop:value=move || title.get()
                on:change=move |ev| {
                    title.set(event_target_value(&ev));
                }
            />

            <button on:click=move |_| {
                status.write().next_step()
            }>
                {move || {
                    if todo.status().done() {
                        "Done"
                    } else if status.scheduled() || status.scheduled_for() {
                        "Scheduled"
                    } else {
                        "Pending"
                    }
                }}

            </button>

            <button on:click=move |_| {
                let id = todo.id().get();
                store.todos().write().retain(|todo| todo.id != id);
            }>"X"</button>
            <input
                type="date"
                prop:value=move || {
                    todo.status().scheduled_for_date().map(|n| n.get().to_string())
                }

                class:hidden=move || !todo.status().scheduled_for()
                on:change:target=move |ev| {
                    if let Some(date) = todo.status().scheduled_for_date() {
                        let value = ev.target().value();
                        match NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                            Ok(new_date) => {
                                date.set(new_date);
                            }
                            Err(e) => warn!("{e}"),
                        }
                    }
                }
            />

        </li>
    }
}

#[component]
pub fn TodoItemsAmount() -> impl IntoView {
    let store = use_context::<Store<Todos>>().expect("TodoStore should be provided");

    view! {
        <div style="background-color: black; color: white; padding: 10px;">"Total " {store.todos().read().len()} " items"</div>
    }
}
