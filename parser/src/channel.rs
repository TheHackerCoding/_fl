use crate::channeldata::ChannelData;

pub struct Channel<'a> {
    pub id: i32,
    pub name: String,
    pub color: u32,
    pub data: dyn ChannelData,
}

impl Channel<'_> {
    pub fn default() -> Self {
        Channel {
            id: 0,
            name: "".to_string(),
            color: 0x4080FF,
            data: None
        }
    }

    pub fn custom(id: i32, data: Option<Box<dyn ChannelData>>) -> Self {
        Channel {
            id,
            name: "".to_string(),
            color: 0x4080FF,
            data
        }
    }
}
