use super::task::Tasks;
use std::io::{Error, ErrorKind, Read, Result, Write};

/// for saving json
pub struct JsonFile<'a> {
    path: &'a str,
    tasks: Tasks,
}

impl<'a> JsonFile<'a> {
    pub fn init(path: &'a str) -> Result<Self> {
        let tasks = Self::read(path)?;
        Ok(JsonFile { path, tasks })
    }

    fn read(path: &str) -> Result<Tasks> {
        let mut f = std::fs::File::create(path)?;

        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        f.flush()?;

        let src = String::from_utf8(buf);
        match src {
            Ok(src) => {
                let tasks = Tasks::from_json(&src)?;
                Ok(tasks)
            }
            Err(e) => Err(Error::from(ErrorKind::InvalidData)),
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
