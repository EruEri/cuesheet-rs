use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CueFileFormat {
    BINARY,
    MOTOROLA,
    AIFF,
    WAVE,
    MP3,
}

impl Display for CueFileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CueFileFormat::BINARY => "BINARY",
            CueFileFormat::MOTOROLA => "MOTOROLA",
            CueFileFormat::AIFF => "AIFF",
            CueFileFormat::WAVE => "WAVE",
            CueFileFormat::MP3 => "MP3",
        };
        write!(f, "{}", s)
    }
}
