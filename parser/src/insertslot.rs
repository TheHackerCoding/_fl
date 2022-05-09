use crate::plugin::Plugin;

#[derive(Clone)]
pub struct InsertSlot {
    pub volume: i32,
    pub state: i32,
    pub dryWet: i32,
    pub pluginSettings: Vec<u8>,
    pub plugin: Plugin,
}

impl InsertSlot {
    pub fn default() -> Self {
        InsertSlot {
            volume: 100,
            state: 0,
            dryWet: -1,
            pluginSettings: vec![],
            plugin: Plugin::new(),
        }
    }
}
