use super::task::Tasks;
use std::io::{Result, Write};

/// for saving json
struct JsonFile<'a> {
    path: &'a str,
    tasks: Tasks,
}

impl<'a> JsonFile<'a> {
    pub fn init(path: &'a str) -> Result<Self> {
        let tasks = Self::read(path)?;
        Ok(JsonFile { path, tasks })
    }

    fn read(path: &str) -> Result<Tasks> {
        let src = std::fs::read_to_string(path)?;
        let tasks = Tasks::from_json(&src)?;
        Ok(tasks)
    }

    pub fn sync(&self, task: Tasks) -> Result<()> {
        // if file has already created , not exec error.
        let mut f = std::fs::File::create(self.path)?;

        let json = Tasks::to_json(&task)?;
        f.write_all(&json.as_bytes())?;
        f.flush()?;
        Ok(())
    }

    pub fn tasks(&mut self) -> &mut Tasks {
        &mut self.tasks
    }
}
