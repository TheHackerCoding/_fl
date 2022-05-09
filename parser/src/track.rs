use crate::playlistitem::PlaylistItem;

#[derive(Clone)]
pub struct Track {
    pub name: String,
    pub color: u32,
    pub items: Vec<PlaylistItem>
}

impl Track {
    pub fn default() -> Self {
        Track {
            name: "".to_string(),
            color: 0,
            items: vec![]
        }
    }
}
