use super::super::Stackable;
use super::task::Tasks;
use std::fs::File;
use std::io::Result;
use std::path::Path;

/// for saving json
struct JsonFile {
    // path: Path,
    path: String,
}

impl JsonFile {
    pub fn new<T>(path: T) -> Self
    where
        T: Into<String>,
    {
        JsonFile { path: path.into() }
    }

    pub fn read(&self) -> Result<Tasks> {
        let mut f = File::open(self.path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        let tasks = Tasks::from_json(&buffer);
        Ok(tasks)
    }

    pub fn write(&self, task: Tasks) -> Result<()> {
        let mut f = File::open(self.path)?;
        let json = Tasks::to_json(&task).expect("what?");
        f.write_all(json)?;
        Ok(())
    }
}

impl Stackable for JsonFile {
    fn pop(&self) -> Option<Task> {
        self.tasks.pop()
    }

    fn push(&self, v: &Task) {
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
