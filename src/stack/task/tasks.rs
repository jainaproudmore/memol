use super::Task;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Result};

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks { tasks: Vec::new() }
    }

    pub fn from_json(src: &str) -> Result<Self> {
        from_str(src)
    }

    pub fn to_json(tasks: &Self) -> Result<String> {
        to_string(tasks)
    }

    pub fn pop(&mut self) -> Option<Task> {
        self.tasks.pop()
    }

    pub fn push(&mut self, v: Task) {
        self.tasks.push(v)
    }

    pub fn peek(&self) -> Option<&Task> {
        self.tasks.last()
    }

    pub fn top(&self) -> Option<&Task> {
        self.peek()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() -> Tasks {
        let mut task = Tasks::new();
        task.tasks = vec![
            Task::new("Task1", 1),
            Task::new("Task2", 2),
            Task::new("Task3", 3),
        ];
        task
    }

    #[test]
    fn it_pop() {
        let mut tasks = init();
        assert_eq!(tasks.pop(), Some(Task::new("Task3", 3)));
    }

    #[test]
    fn it_push() {
        let mut tasks = init();
        tasks.push(Task::new("Task4", 4));
        assert_eq!(
            tasks.tasks,
            vec![
                Task::new("Task1", 1),
                Task::new("Task2", 2),
                Task::new("Task3", 3),
                Task::new("Task4", 4),
            ]
        );
    }

    #[test]
    fn it_peek() {
        let tasks = init();
        assert_eq!(tasks.peek(), Some(&Task::new("Task3", 3)));
    }

    #[test]
    fn it_top() {
        let tasks = init();
        assert_eq!(tasks.top(), Some(&Task::new("Task3", 3)));
    }

    #[test]
    fn it_empty() {
        let tasks = init();
        assert_eq!(tasks.is_empty(), false);

        let tasks = Tasks::new();
        assert_eq!(tasks.is_empty(), true);
    }
}
