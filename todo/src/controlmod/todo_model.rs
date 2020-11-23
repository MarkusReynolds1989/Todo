use chrono::{DateTime, Utc};
use core::fmt;
use std::fmt::Formatter;

pub struct Task {
    task_id: i32,
    name: String,
    description: String,
    is_done: bool,
    is_past_due: bool,
    due_date: DateTime<Utc>,
}

impl Task {
    pub fn new(
        task_id: i32,
        name: String,
        description: String,
        is_done: bool,
        is_past_due: bool,
        due_date: DateTime<Utc>,
    ) -> Task {
        Task {
            task_id,
            name,
            description,
            is_done,
            is_past_due,
            due_date,
        }
    }
}

// Just for logging.
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id: {}\nname: {}\ndescription: {}\nis_done: {}\nis_past_due: {}\ndue_date: {}\n",
            self.task_id.to_string(),
            self.name,
            self.description,
            self.is_done.to_string(),
            self.is_past_due.to_string(),
            self.due_date
        )
    }
}
