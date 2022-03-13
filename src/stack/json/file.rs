use super::super::task::Tasks;
use std::io::{Error, Read, Result, Write};
use std::path::Path;

/// for saving json
pub struct JsonFile<'a> {
    path: &'a Path,
    tasks: Tasks,
}

impl<'a> JsonFile<'a> {
    pub fn init(path: &'a Path) -> Result<Self> {
        let tasks = Self::read(path)?;
        Ok(JsonFile { path, tasks })
    }

    fn read(path: &Path) -> Result<Tasks> {
        let f = std::fs::File::open(path);
        if let Err(_) = f {
            return Ok(Tasks::new());
        }

        let mut f = f.unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        f.flush()?;

        if let Ok(t) = Tasks::from_json(&buf) {
            Ok(t)
        } else {
            Ok(Tasks::new())
        }
    }

    pub fn sync(&self) -> Result<()> {
        // if file has already created , not exec error.
        let mut f = std::fs::File::create(self.path)?;

        let json = Tasks::to_json(&self.tasks)?;
        f.write_all(&json.as_bytes())?;
        f.flush()?;
        Ok(())
    }

    pub fn tasks(&mut self) -> &mut Tasks {
        &mut self.tasks
    }
}
