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
        <div class="bg-white p-6 rounded-lg shadow-lg">
            <h2 class="text-2xl font-bold text-blue-600 mb-4">"Hello, " {move || store.user().name().get()}</h2>
            <div class="mb-6">
                <UserForm user=store.user()/>
            </div>
            <hr class="my-6 border-gray-200"/>
            <form 
                class="mb-6"
                on:submit=move |ev| {
                    ev.prevent_default();
                    if let Some(input) = input_ref.get() {
                        let new_todo = Todo::new(input.value());
                        store.todos().write().push(new_todo);
                        input.set_value("");
                    }
                }
            >
                <div class="flex items-center space-x-2">
                    <label class="font-medium">"Add a Todo"</label>
                    <input 
                        type="text" 
                        class="flex-grow p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500" 
                        node_ref=input_ref
                    />
                    <input 
                        type="submit" 
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded cursor-pointer"
                        value="Add"
                    />
                </div>
            </form>
            <ol class="space-y-2 mb-6 list-decimal pl-6">
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
            <div class="mt-6 p-4 bg-gray-100 rounded overflow-auto max-h-60">
                <pre class="text-xs text-gray-700">{move || serde_json::to_string_pretty(&*store.read())}</pre>
            </div>
        </div>
    }
}

#[component]
fn UserForm(#[prop(into)] user: Field<User>) -> impl IntoView {
    let error = RwSignal::new(None);

    view! {
        <div class="bg-gray-50 p-4 rounded-md border border-gray-200">
            {move || error.get().map(|n| view! { <p class="text-red-500 mb-3 font-medium">{n}</p> })}
            <form 
                class="space-y-4"
                on:submit:target=move |ev| {
                    ev.prevent_default();
                    match User::from_event(&ev) {
                        Ok(new_user) => {
                            error.set(None);
                            user.patch(new_user);
                        }
                        Err(e) => error.set(Some(e.to_string())),
                    }
                }
            >
                <div class="flex flex-col space-y-1">
                    <label class="font-medium text-gray-700">
                        "Name"
                    </label>
                    <input 
                        type="text" 
                        name="name" 
                        class="p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                        prop:value=move || user.name().get()
                    />
                </div>
                <div class="flex flex-col space-y-1">
                    <label class="font-medium text-gray-700">
                        "Email"
                    </label>
                    <input 
                        type="email" 
                        name="email" 
                        class="p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                        prop:value=move || user.email().get()
                    />
                </div>
                <div>
                    <input 
                        type="submit" 
                        value="Update User"
                        class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded cursor-pointer"
                    />
                </div>
            </form>
        </div>
    }
}

#[component]
fn TodoRow(store: Store<Todos>, #[prop(into)] todo: Field<Todo>) -> impl IntoView {
    let status = todo.status();
    let title = todo.label();

    let editing = RwSignal::new(true);

    view! {
        <li class=move || {
            if status.done() {
                "line-through text-gray-500"
            } else {
                ""
            }
        }>

            <div class="flex items-center space-x-2 p-2 bg-white rounded-md shadow-sm">
                <p
                    class="flex-grow px-2 py-1 cursor-pointer hover:bg-gray-100 rounded"
                    class:hidden=move || editing.get()
                    on:click=move |_| {
                        editing.update(|n| *n = !*n);
                    }
                >
                    {move || title.get()}
                </p>
                <input
                    class="flex-grow px-2 py-1 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    class:hidden=move || !editing.get()
                    type="text"
                    prop:value=move || title.get()
                    on:change=move |ev| {
                        title.set(event_target_value(&ev));
                    }
                />

                <button 
                    class=move || {
                        let base = "px-3 py-1 rounded font-medium text-white";
                        if todo.status().done() {
                            format!("{} bg-green-500 hover:bg-green-700", base)
                        } else if status.scheduled() || status.scheduled_for() {
                            format!("{} bg-yellow-500 hover:bg-yellow-700", base)
                        } else {
                            format!("{} bg-blue-500 hover:bg-blue-700", base)
                        }
                    }
                    on:click=move |_| {
                        status.write().next_step()
                    }
                >
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

                <button 
                    class="px-2 py-1 bg-red-500 hover:bg-red-700 text-white rounded"
                    on:click=move |_| {
                        let id = todo.id().get();
                        store.todos().write().retain(|todo| todo.id != id);
                    }
                >
                    "X"
                </button>

                <input
                    class="px-2 py-1 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    class:hidden=move || !todo.status().scheduled_for()
                    type="date"
                    prop:value=move || {
                        todo.status()
                            .scheduled_for_date()
                            .map(|n| n.get().to_string())
                            .unwrap_or_default() // Noneの場合は空文字列を返す
                    }
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
            </div>

        </li>
    }
}

#[component]
pub fn TodoItemsAmount() -> impl IntoView {
    let store = use_context::<Store<Todos>>().expect("TodoStore should be provided");

    view! {
        <div class="bg-black text-white p-3 rounded-md font-semibold">"Total " {move || store.todos().read().len()} " items"</div>
    }
}
