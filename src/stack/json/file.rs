use super::super::task::Tasks;
use std::io::{Read, Result, Write};
use std::path::Path;

/// for saving json
pub struct JsonFile<'a> {
    path: &'a Path,
}

impl<'a> JsonFile<'a> {
    pub fn new(path: &'a Path) -> Self {
        JsonFile { path }
    }

    pub fn read(&self) -> Result<Tasks> {
        let f = std::fs::File::open(self.path);
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

    pub fn sync(&self, tasks: &Tasks) -> Result<()> {
        // if file has already created , not exec error.
        let mut f = std::fs::File::create(self.path)?;

        let json = Tasks::to_json(tasks)?;
        f.write_all(&json.as_bytes())?;
        f.flush()?;
        Ok(())
    }
}
