use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Task {
    pub value: String,
    pub timestamp: i64,
}

impl Task {
    pub fn new<T>(v: T, ts: i64) -> Self
    where
        T: Into<String>,
    {
        Task {
            value: v.into(),
            timestamp: ts,
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (at {})", self.value, self.timestamp)
    }
}
