use crate::{
    channel::Channel, channeldata::ChannelData, enums::Event, generatordata::GeneratorData,
    insert::Insert, insertslot::InsertSlot, pattern::Pattern, project::Project,
};
use anyhow::{bail, Result};
use binary_rw::{BinaryReader, Endian, MemoryStream};
use std::str;

pub struct ProjectParser {
    project: Project,
    currentPattern: Pattern,
    currentChannel: Channel,
    currentInsert: Insert,
    currentInsertSlot: InsertSlot,
    verbose: bool,
    versionMajor: i32,
}

impl ProjectParser {
    pub fn new(verbose: bool) -> Self {
        ProjectParser {
            project: Project::new(),
            currentPattern: Pattern::default(),
            currentChannel: Channel::default(),
            currentInsert: Insert::new(),
            currentInsertSlot: InsertSlot::default(),
            verbose,
            versionMajor: 0,
        }
    }

    pub fn parse(&mut self, data: Vec<u8>) -> Result<()> {
        self.currentInsert = self.project.inserts[0].clone();
        let mut dat = data.clone();
        let mut stream = MemoryStream::from(data);
        let mut bin = BinaryReader::new(&mut stream, Endian::default());
        self.parseHeader(&mut bin)?;
        self.parseFldt(&mut bin)?;
        while bin.tell()? < dat.len() {
            self.parseEvent(&mut bin)?;
        }
        Ok(())
    }

    fn parseHeader(&mut self, data: &mut BinaryReader) -> Result<()> {
        if str::from_utf8(&data.read_bytes(4)?)? != "FLhd" {
            // Err(FLParse::Custom("invalid magic number".to_string()));
            bail!("invalid magic number");
        }
        let headerLength = data.read_i32()?;
        if headerLength != 6 {
            bail!("expected header length 6, got {:?}", headerLength);
        }
        let _type = data.read_i64()?;
        if _type != 0 {
            bail!("type {:?} is not supported", _type);
        }
        let channelCount = data.read_i16()?;
        if channelCount < 1 || channelCount > 1000 {
            bail!("invalid number of channels: {}", channelCount);
        }
        for i in 0..channelCount {
            self.project.channels.push(Channel::custom(
                i.into(),
                Box::new(GeneratorData::new()),
            ))
        }
        self.project.ppq = data.read_i16()?.into();
        if self.project.ppq < 0 {
            bail!("invalid PPQ: {}", self.project.ppq);
        }
        Ok(())
    }

    fn parseFldt(&mut self, data: &mut BinaryReader) -> Result<()> {
        let mut id = "";
        let mut len: i32 = 0;
        let mut dat: Vec<u8> = vec![];
        while id != "FLdt" {
            data.read_bytes(len as usize);
            dat = data.read_bytes(4)?.clone();
            id = str::from_utf8(&dat)?;
            len = data.read_i32()?;

            if len < 0 || len > 0x10000000 {
                bail!("invalid chunck length: {}", len);
            }
        }
        Ok(())
    }

    fn parseEvent(&mut self, data: &mut BinaryReader) -> Result<()> {
        let startPos = data.tell()?;
        let eventId = data.read_bytes(1)?[0] as i32;
        let event = Event::try_from(eventId).unwrap();
        self.output(format!("{:?} {} at {}", event, eventId, startPos));
        match eventId {
            i if i < Event::Word.into() => self.parseByteEvent(eventId, data)?,
            _ => self.parseDataEvent(eventId, data)?,
        }
        Ok(())
    }

    fn parseByteEvent(&mut self, eventId: i32, data: &mut BinaryReader) -> Result<()> {
        // let dat = self.currentChannel.data;
        let data = data.read_bytes(1)?[0] as i32;
        self.output(format! {"byte: {:?}", data});
        // oh god this must be rust hell
        // let mut genData = self.currentChannel.data.as_ref().unwrap().as_any().downcast_ref::<GeneratorData>().as_mut().unwrap();
        // let mut genData: 
        // let mut genData: Option<&GeneratorData> = match self.currentChannel.data.as_ref() {
        //     Some(x) => Some(x.as_any().downcast_ref::<GeneratorData>().as_mut().unwrap().clone()),
        //     None => None,
        // };
        // let mut genData = &mut *self.currentChannel.data.as_ref().unwrap().as_any().downcast_ref::<GeneratorData>().as_mut().unwrap().to_owned().to_owned();
        let genData = &mut self.currentChannel.data;
        // fn parse<'a>(x: Option<&'a mut Box<&'a mut (dyn ChannelData + 'a)>>) -> &'a GeneratorData {
        //     x.unwrap()
        //         .as_mut()
        //         .as_any()
        //         .downcast_ref::<GeneratorData>()
        //         .unwrap()
        // }
        // fn parse(x: Option<&Box<dyn ChannelData>>) -> &GeneratorData {
        //     x.unwrap().to_owned().as_any().downcast_ref().unwrap()
        // }
        match Event::try_from(eventId).unwrap() {
            Event::ByteMainVol => self.project.mainVolume = data,
            Event::ByteUseLoopPoints => {
                if stringify!(genData.as_ref()) != "NullChannelData" {
                    // let mut gen = &*genData.as_mut().as_any().downcast_ref::<GeneratorData>().as_mut().unwrap().clone().to_owned();
                    let x = genData.as_mut();
                    let y = x.as_any().downcast_ref::<GeneratorData>();
                    y.unwrap().insert = data;
                    // gen.insert = data;
                    // let _gen = gen.to_owned();
                }
                // if genData.is_some() {
                //     // let mut gen = parse(genData);
                //     genData.unwrap().change_insert(data);
                // }
                // genData.insert = data;
            }
            _ => bail!("major paring error"),
        }
        Ok(())
    }

    fn parseDataEvent(&mut self, eventId: i32, data: &mut BinaryReader) -> Result<()> {
        todo!();
    }

    fn output(&mut self, msg: String) {
        if self.verbose {
            println!("{}", msg);
        }
    }
}
