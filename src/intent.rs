use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Intent {
    Greeting,
    Weather,
    Search,
    Unknown,
}
