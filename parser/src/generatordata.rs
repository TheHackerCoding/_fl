use std::any::Any;

use crate::{plugin::Plugin, enums::ArpDirection, channeldata::ChannelData};

pub struct GeneratorData {
pub pluginSettings: Vec<u8>,
    pub plugin: Plugin,
    pub generatorName: String,
    pub volume: f64,
    pub panning: f64,
    pub bassNote: u32,
    pub insert: i32,
    pub layerParent: i32,
    pub sampleFilename: String,
    pub sampleAmp: i32,
    pub sampleReversed: bool,
    pub sampleReverseStereo: bool,
    pub sampleUseLoopPoints: bool,
    pub arpDir: ArpDirection,
    pub arpRange: i32,
    pub arpChord: i32,
    pub arpRepeat: i32,
    pub arpTime: f64,
    pub arpGate: f64,
    pub arpSlide: bool,
}

impl GeneratorData {
    pub fn new() -> Self {
        GeneratorData {
            pluginSettings: vec![],
            plugin: Plugin::new(),
            generatorName: "".to_string(),
            volume: 100.0,
            panning: 0.0,
            bassNote: 57,
            insert: -1,
            layerParent: -1,
            sampleFilename: "".to_string(),
            sampleAmp: 100,
            sampleReversed: false,
            sampleReverseStereo: false,
            sampleUseLoopPoints: false,
            arpDir: ArpDirection::Off,
            arpRange: 0,
            arpChord: 0,
            arpRepeat: 0,
            arpTime: 100.0,
            arpGate: 100.0,
            arpSlide: false,
            
        }
    }

    pub fn change_insert(&mut self, x: i32) {
        self.insert = x
    }
}

impl ChannelData for GeneratorData {
    fn as_any(self: &mut GeneratorData) -> &mut (dyn Any + 'static) {
        self
    }
}
