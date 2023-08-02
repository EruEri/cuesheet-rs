use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CueTrackFlag {
    PRE,
    DCP,
    F4CH,
    SCMS,
}

impl Display for CueTrackFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CueTrackFlag::PRE => "PRE",
            CueTrackFlag::DCP => "DCP",
            CueTrackFlag::F4CH => "4CH",
            CueTrackFlag::SCMS => "SCMS",
        };
        write!(f, "{}", s)
    }
}
