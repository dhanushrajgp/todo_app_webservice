use serde::ser::{Serialize, Serializer};
#[derive(Clone,Debug,Eq)]
pub enum TaskStatus {
    DONE,
    PENDING,
}
impl TaskStatus {
    pub fn new(status: &String) -> TaskStatus {
        TaskStatus::from_string(status.as_str().to_string())
    }
    pub fn _stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
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

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str(&self._stringify().as_str())?)
    }
}

impl PartialEq for TaskStatus {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TaskStatus::DONE => {
                match other {
                    &TaskStatus::DONE => return true,
                    &TaskStatus::PENDING => false
                }
            },
            TaskStatus::PENDING => {
                match other {
                    &TaskStatus::DONE => return false,
                    &TaskStatus::PENDING => true
                }
            }
        }
    }
}
