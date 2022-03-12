use super::super::Stackable;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Result};

#[derive(Deserialize, Serialize)]
pub struct Task {
    value: String,
    timestamp: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn from_json(src: &str) -> Result<Tasks> {
        from_str(src)
    }

    pub fn to_json(tasks: &Tasks) -> Result<String> {
        to_string(tasks)
    }
}

impl Stackable<Task> for Tasks {
    fn pop(&self) -> Option<Task> {
        self.tasks.pop()
    }

    fn push(&self, v: Task) {
        self.tasks.push(v)
    }

    fn peek(&self) -> Option<&Task> {
        self.tasks.last()
    }

    fn top(&self) -> Option<&Task> {
        self.peek()
    }

    fn is_empty(&self) -> bool {
        self.tasks.len() == 0
    }
}

// #[cfg(test)]
