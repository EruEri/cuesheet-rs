use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CueTrackMode {
    AUDIO,
    CDG,
    MODE1_2048,
    MODE1_2352,
    MODE2_2336,
    MODE2_2352,
    CDI2336,
    CDI2352,
}

impl Display for CueTrackMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CueTrackMode::AUDIO => "AUDIO",
            CueTrackMode::CDG => "CDG",
            CueTrackMode::MODE1_2048 => "MODE1/2048",
            CueTrackMode::MODE1_2352 => "MODE1/2352",
            CueTrackMode::MODE2_2336 => "MODE2/2336",
            CueTrackMode::MODE2_2352 => "MODE2/2352",
            CueTrackMode::CDI2336 => "CDI/2336",
            CueTrackMode::CDI2352 => "CDI/2352",
        };
        write!(f, "{}", s)
    }
}
