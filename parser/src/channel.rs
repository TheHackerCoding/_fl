pub struct Channel {
    pub id: i32,
    pub name: String,
    pub color: u32,
}

impl Channel {
    pub fn default() -> Self {
        Channel {
            id: 0,
            name: "".to_string(),
            color: 0x4080FF,
        }
    }
}
