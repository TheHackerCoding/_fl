use crate::{enums::InsertFlags, insertslot::InsertSlot, project::Project};

#[derive(Clone)]
pub struct Insert {
    pub id: i32,
    pub name: String,
    pub color: u32,
    pub icon: u16,
    pub flags: InsertFlags,
    pub volume: i32,
    pub pan: i32,
    pub stereoSep: i32,
    pub lowLevel: i32,
    pub bandLevel: i32,
    pub highLevel: i32,
    pub lowFreq: i32,
    pub bandFreq: i32,
    pub highFreq: i32,
    pub lowWidth: i32,
    pub bandWidth: i32,
    pub highWidth: i32,
    pub routes: Vec<bool>,
    pub routeVolumes: Vec<i32>,
    pub slots: Vec<InsertSlot>,
}

impl Insert {
    pub const MAXSLOTCOUNT: i32 = 10;

    pub fn new() -> Self {
        Insert {
            id: 0,
            name: "".to_string(),
            color: 0x000000,
            icon: 0,
            flags: InsertFlags::Default,
            volume: 100,
            pan: 0,
            stereoSep: 0,
            lowLevel: 0,
            bandLevel: 0,
            highLevel: 0,
            lowFreq: 0,
            bandFreq: 0,
            highFreq: 0,
            lowWidth: 0,
            bandWidth: 0,
            highWidth: 0,
            routes: vec![false; Project::MAXTRACKCOUNT as usize],
            routeVolumes: vec![12800, Project::MAXINSERTCOUNT],
            slots: vec![InsertSlot::default(); Self::MAXSLOTCOUNT as usize],
        }
    }
}
