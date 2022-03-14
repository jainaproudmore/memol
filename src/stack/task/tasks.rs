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

    pub fn r(self) -> Tasks {
        Tasks {
            tasks: self.tasks.into_iter().rev().collect::<Vec<_>>(),
        }
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

    pub fn all(&self) -> &[Task] {
        &self.tasks[0..self.tasks.len()]
    }

    pub fn all_count(&self, count: usize) -> &[Task] {
        let count = std::cmp::max(0, self.tasks.len() as isize - count as isize) as usize;
        &self.tasks[count..self.tasks.len()]
    }

    pub fn clear(&mut self) {
        self.tasks = Vec::new();
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
    fn it_all() {
        let mut tasks = Tasks::new();
        tasks.push(Task::new("Task1", 1));
        tasks.push(Task::new("Task2", 2));
        tasks.push(Task::new("Task3", 3));
        assert_eq!(
            tasks.all(),
            &[
                Task::new("Task1", 1),
                Task::new("Task2", 2),
                Task::new("Task3", 3),
            ]
        );
    }

    #[test]
    fn it_all_count() {
        let mut tasks = Tasks::new();
        tasks.push(Task::new("Task1", 1));
        tasks.push(Task::new("Task2", 2));
        tasks.push(Task::new("Task3", 3));
        assert_eq!(
            tasks.all_count(2),
            &[Task::new("Task2", 2), Task::new("Task3", 3),]
        );
    }

    #[test]
    fn it_all_count_over_size() {
        let mut tasks = Tasks::new();
        tasks.push(Task::new("Task1", 1));
        tasks.push(Task::new("Task2", 2));
        assert_eq!(
            tasks.all_count(3),
            &[Task::new("Task1", 1), Task::new("Task2", 2),]
        );
    }

    #[test]
    fn it_reverse() {
        let mut tasks = Tasks::new();
        tasks.push(Task::new("Task1", 1));
        tasks.push(Task::new("Task2", 2));
        tasks.push(Task::new("Task3", 3));
        assert_eq!(
            tasks.r().tasks,
            vec![
                Task::new("Task3", 3),
                Task::new("Task2", 2),
                Task::new("Task1", 1),
            ]
        );
    }

    #[test]
    fn it_clear() {
        let mut tasks = Tasks::new();
        tasks.push(Task::new("Task1", 1));
        tasks.push(Task::new("Task2", 2));
        tasks.push(Task::new("Task3", 3));
        tasks.clear();
        assert_eq!(tasks.tasks, vec![]);
    }
}
