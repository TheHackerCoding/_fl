use std::any::Any;

pub trait ChannelData {
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct NullChannelData;

impl ChannelData for NullChannelData {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}