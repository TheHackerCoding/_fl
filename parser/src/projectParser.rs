use crate::{
    channel::Channel, insert::Insert, insertslot::InsertSlot, pattern::Pattern, project::Project,
    track::Track,
};

pub struct ProjectParser<'a> {
    project: Project,
    currentPattern: Pattern,
    currentChannel: Channel,
    currentInsert: &'a Insert,
    currentInsertSlot: InsertSlot,
    verbose: bool,
    versionMajor: i32,
}

impl ProjectParser<'_> {
    pub fn new(verbose: bool) -> Self {
        ProjectParser {
            project: Project::new(),
            currentPattern: Pattern::default(),
            currentChannel: Channel::default(),
            currentInsert: &Insert::new(),
            currentInsertSlot: InsertSlot::default(),
            verbose,
            versionMajor: 0,
        }
    }

    pub fn parse(mut self, data: Vec<u8>) {
        self.currentInsert = &self.project.inserts[0];
    }
}
