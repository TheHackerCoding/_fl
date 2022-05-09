#[derive(Clone)]
pub struct Plugin {
    pub midiInPort: i32,
    pub midiOutPort: i32,
    pub pitchBendRange: u8,
    pub flags: u32,
    pub numInputs: i32,
    pub numOutputs: i32,

    pub inputInfo: Vec<PluginIoInfo>,
    pub outputInfo: Vec<PluginIoInfo>,

    pub infoKind: i32,
    pub vstNumber: u32,
    pub vstId: String,
    pub guid: Vec<u8>,
    pub state: Vec<u8>,
    pub name: String,
    pub filename: String,
    pub vendorName: String,
}

impl Plugin {
    pub fn new() -> Self {
        Plugin {
            midiInPort: 0,
            midiOutPort: 0,
            pitchBendRange: 0,
            flags: 0,
            numInputs: 0,
            numOutputs: 0,
            inputInfo: vec![],
            outputInfo: vec![],
            infoKind: 0,
            vstNumber: 0,
            vstId: "".to_string(),
            guid: vec![],
            state: vec![],
            name: "".to_string(),
            filename: "".to_string(),
            vendorName: "".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct PluginIoInfo {
    pub mixerOffset: i32,
    pub flags: u32,
}

impl PluginIoInfo {
    pub fn default() -> Self {
        PluginIoInfo {
            mixerOffset: 0,
            flags: 0,
        }
    }
}
