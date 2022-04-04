use crate::stack::task::Tasks;
use std::io::Result;

// pub type Result<T> = result::Result<T, Error>;

/// custmize persistent memo data
pub trait Persistence {
    fn read(&self) -> Result<Tasks>;
    fn sync(&self, tasks: &Tasks) -> Result<Tasks>;
}
