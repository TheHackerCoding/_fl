use std::collections::HashMap;
use crate::{channel::Channel, note::Note};
pub struct Pattern {
    pub id: i32,
    pub name: String,
    pub notes: HashMap<Channel, Vec<Note>>,
}

impl Pattern {
    pub fn default() -> Self {
        Pattern {
            id: 0,
            name: "".to_string(),
            notes: HashMap::new(),
        }
    }
}
