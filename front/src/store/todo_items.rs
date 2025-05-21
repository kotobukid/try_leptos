use std::sync::atomic::{AtomicUsize, Ordering};
use chrono::{Local, NaiveDate};
use reactive_stores::{Patch, Store};
use serde::{Deserialize, Serialize};

// ID starts higher than 0 because we have a few starting todos by default
static NEXT_ID: AtomicUsize = AtomicUsize::new(3);

#[derive(Debug, Store, Serialize, Deserialize)]
pub struct Todos {
    user: User,
    #[store(key: usize = |todo| todo.id)]
    todos: Vec<Todo>,
}

#[derive(Debug, Store, Patch, Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
}

#[derive(Debug, Store, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    label: String,
    status: Status,
}

#[derive(Debug, Default, Clone, Store, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Pending,
    Scheduled,
    ScheduledFor {
        date: NaiveDate,
    },
    Done,
}

impl Status {
    pub fn next_step(&mut self) {
        *self = match self {
            Status::Pending => Status::ScheduledFor {
                date: Local::now().naive_local().into(),
            },
            Status::Scheduled | Status::ScheduledFor { .. } => Status::Done,
            Status::Done => Status::Done,
        };
    }
}

impl Todo {
    pub fn new(label: impl ToString) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            label: label.to_string(),
            status: Status::Pending,
        }
    }
}

pub fn data() -> Todos {
    Todos {
        user: User {
            name: "Bob".to_string(),
            email: "bob@example.com".into(),
        },
        todos: vec![
            Todo {
                id: 0,
                label: "Create reactive store".to_string(),
                status: Status::Pending,
            },
            Todo {
                id: 1,
                label: "???".to_string(),
                status: Status::Pending,
            },
            Todo {
                id: 2,
                label: "Profit".to_string(),
                status: Status::Pending,
            },
        ],
    }
}