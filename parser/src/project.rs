use crate::{channel::Channel, insert::Insert, pattern::Pattern, track::Track};

pub struct Project {
    pub mainVolume: i32,
    pub mainPitch: i32,
    pub ppq: i32,
    pub tempo: f64,
    pub projectTitle: String,
    pub comment: String,
    pub author: String,
    pub genre: String,
    pub versionString: String,
    pub version: i32,
    pub channels: Vec<Channel>,
    pub tracks: Vec<Track>,
    pub patterns: Vec<Pattern>,
    pub inserts: Vec<Insert>,
    pub playTruncatedNotes: bool,
}

impl Project {
    pub const MAXINSERTCOUNT: i32 = 127;
    pub const MAXTRACKCOUNT: i32 = 199;

    pub fn new() -> Self {
        let mut _inserts: Vec<Insert> = vec![Insert::new(); Self::MAXINSERTCOUNT as usize];
        for n in 0..Self::MAXINSERTCOUNT {
            _inserts[n as usize].id = n;
            _inserts[n as usize].name = format!("Insert {}", n);
        }
        _inserts[0].name = "Master".to_string();
        Project {
            mainVolume: 300,
            mainPitch: 0,
            ppq: 0,
            tempo: 140.0,
            projectTitle: String::new(),
            comment: String::new(),
            author: String::new(),
            genre: String::new(),
            versionString: String::new(),
            version: 0x100,
            channels: vec![],
            tracks: vec![Track::default(); Self::MAXTRACKCOUNT as usize],
            patterns: vec![],
            inserts: _inserts,
            playTruncatedNotes: false,
        }
    }
}
