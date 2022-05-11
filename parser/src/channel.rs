use crate::channeldata::{ChannelData, NullChannelData};

pub struct Channel {
    pub id: i32,
    pub name: String,
    pub color: u32,
    pub data: Box<dyn ChannelData>,
}

impl Channel {
    pub fn default() -> Self {
        Channel {
            id: 0,
            name: "".to_string(),
            color: 0x4080FF,
            data: Box::new(NullChannelData)
        }
    }

    pub fn custom(id: i32, data: Box<dyn ChannelData>) -> Self {
        Channel {
            id,
            name: "".to_string(),
            color: 0x4080FF,
            data
        }
    }
}
