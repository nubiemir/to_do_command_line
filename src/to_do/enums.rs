use std::fmt::{self, write};

#[derive(Debug)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::DONE => write!(f, "DONE"),
            Self::PENDING => write!(f, "PENDING"),
        }
    }
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            TaskStatus::DONE => "Done".to_string(),
            TaskStatus::PENDING => "Pending".to_string(),
        }
    }
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}
