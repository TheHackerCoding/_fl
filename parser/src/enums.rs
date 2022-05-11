use num_enum::{TryFromPrimitive, IntoPrimitive};

const INT: i32 = 128;
const WORD: i32 = 64;
const UNDEF: i32 = 192;
const TEXT: i32 = UNDEF;
const DATA: i32 = 210;

#[derive(PartialEq, Eq, TryFromPrimitive, Debug, IntoPrimitive)]
#[repr(i32)]
pub enum Event {
    ByteEnabled = 0,
    ByteNoteOn = 1,
    ByteVol = 2,
    BytePan = 3,
    ByteMidiChan = 4,
    ByteMidiNote = 5,
    ByteMidiPatch = 6,
    ByteMidiBank = 7,
    ByteLoopActive = 9,
    ByteShowInfo = 10,
    ByteShuffle = 11,
    ByteMainVol = 12,
    ByteStretch = 13,
    BytePitchable = 14,
    ByteZipped = 15,
    ByteDelayFlags = 16,
    BytePatLength = 17,
    ByteBlockLength = 18,
    ByteUseLoopPoints = 19,
    ByteLoopType = 20,
    ByteChanType = 21,
    ByteMixSliceNum = 22,
    ByteEffectChannelMuted = 27,
    BytePlayTruncatedNotes = 30,

    WordNewChan = WORD,
    WordNewPat = WORD + 1,
    WordTempo = WORD + 2,
    WordCurrentPatNum = WORD + 3,
    WordPatData = WORD + 4,
    WordFx = WORD + 5,
    WordFadeStereo = WORD + 6,
    WordCutOff = WORD + 7,
    WordDotVol = WORD + 8,
    WordDotPan = WORD + 9,
    WordPreAmp = WORD + 10,
    WordDecay = WORD + 11,
    WordAttack = WORD + 12,
    WordDotNote = WORD + 13,
    WordDotPitch = WORD + 14,
    WordDotMix = WORD + 15,
    WordMainPitch = WORD + 16,
    WordRandChan = WORD + 17,
    WordMixChan = WORD + 18,
    WordResonance = WORD + 19,
    WordLoopBar = WORD + 20,
    WordStDel = WORD + 21,
    WordFx3 = WORD + 22,
    WordDotReso = WORD + 23,
    WordDotCutOff = WORD + 24,
    WordShiftDelay = WORD + 25,
    WordLoopEndBar = WORD + 26,
    WordDot = WORD + 27,
    WordDotShift = WORD + 28,
    WordLayerChans = WORD + 30,
    WordInsertIcon = WORD + 31,
    WordCurrentSlotNum = WORD + 34,

    DWordColor = INT,
    DWordPlayListItem = INT + 1,
    DWordEcho = INT + 2,
    DWordFxSine = INT + 3,
    DWordCutCutBy = INT + 4,
    DWordWindowH = INT + 5,
    DWordMiddleNote = INT + 7,
    DWordReserved = INT + 8,
    DWordMainResoCutOff = INT + 9,
    DWordDelayReso = INT + 10,
    DWordReverb = INT + 11,
    DWordIntStretch = INT + 12,
    DWordSsNote = INT + 13,
    DWordFineTune = INT + 14,
    DWordInsertColor = INT + 21,
    DWordFineTempo = INT + 28,

    TextChanName = TEXT,
    TextPatName = TEXT + 1,
    TextTitle = TEXT + 2,
    TextComment = TEXT + 3,
    TextSampleFileName = TEXT + 4,
    TextUrl = TEXT + 5,
    TextCommentRtf = TEXT + 6,
    TextVersion = TEXT + 7,
    GeneratorName = TEXT + 9,
    TextPluginName = TEXT + 11,
    TextInsertName = TEXT + 12,
    TextGenre = TEXT + 14,
    TextAuthor = TEXT + 15,
    TextMidiCtrls = TEXT + 16,
    TextDelay = TEXT + 17,

    DataTs404Params = DATA,
    DataDelayLine = DATA + 1,
    DataNewPlugin = DATA + 2,
    DataPluginParams = DATA + 3,
    DataChanParams = DATA + 5,
    DataEnvLfoParams = DATA + 8,
    DataBasicChanParams = DATA + 9,
    DataOldFilterParams = DATA + 10,
    DataOldAutomationData = DATA + 13,
    DataPatternNotes = DATA + 14,
    DataInsertParams = DATA + 15,
    DataAutomationChannels = DATA + 17,
    DataChanGroupName = DATA + 21,
    DataPlayListItems = DATA + 23,
    DataAutomationData = DATA + 24,
    DataInsertRoutes = DATA + 25,
    DataInsertFlags = DATA + 26,
    DataSaveTimestamp = DATA + 27,
}

impl Event {
    pub const Byte: Self = Self::ByteEnabled;
    pub const Word: Self = Self::WordNewChan;
    pub const Int: Self = Self::DWordColor;
    pub const Undef: Self = Self::TextChanName;
    pub const Text: Self = Self::TextChanName;
    pub const Data: Self = Self::DataTs404Params;
}

#[derive(PartialEq, Eq)]
#[repr(i32)]
pub enum PluginType {
    Vst = 8,
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i32)]
pub enum ArpDirection {
    Off = 0,
    Up = 1,
    Down = 2,
    UpDownBounce = 3,
    UpDownSticky = 4,
    Random = 5,
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i32)]
pub enum PluginChunkId {
    Midi = 1,
    Flags = 2,
    Io = 30,
    InputInfo = 31,
    OutputInfo = 32,
    PluginInfo = 50,
    VstPlugin = 51,
    Guid = 52,
    State = 53,
    Name = 54,
    Filename = 55,
    VendorName = 56,
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i32)]
pub enum InsertParam {
    SlotState = 0x00,
    SlotVolume = 0x01,
    SlotDryWet = 0x02,
    Volume = 0xC0,
    Pan = 0xC1,
    StereoSep = 0xC2,
    LowLevel = 0xD0,
    BandLevel = 0xD1,
    HighLevel = 0xD2,
    LowFreq = 0xD8,
    BandFreq = 0xD9,
    HighFreq = 0xDA,
    LowWidth = 0xE0,
    BandWidth = 0xE1,
    HighWidth = 0xE2,
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i32)]
pub enum InsertFlags {
    Default = 0,
    ReversePolarity = 1,
    SwapChannels = 1 << 1,
    Unknown3 = 1 << 2,
    Unmute = 1 << 3,
    DisableThreaded = 1 << 4,
    Unknown6 = 1 << 5,
    DockedMiddle = 1 << 6,
    DockedRight = 1 << 7,
    Unknown9 = 1 << 8,
    Unknown10 = 1 << 9,
    Separator = 1 << 10,
    Lock = 1 << 11,
    Solo = 1 << 12,
    Unknown14 = 1 << 13,
    Unknown15 = 1 << 14,
    Unknown16 = 1 << 15,
}
