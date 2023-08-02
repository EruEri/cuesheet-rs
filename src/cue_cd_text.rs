use std::fmt::Display;

use crate::util::cue_format_string_value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CueCdText {
    Arrager(String),
    Composer(String),
    DiscId(String),
    Genre(String),
    ISrc(String),
    Message(String),
    Performer(String),
    SongWriter(String),
    Title(String),
    TocInfo(String),
    TocInfo2(String),
    UpcEan(String),
    SizeInfo(String),
}

impl Display for CueCdText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (key, value) = match self {
            CueCdText::Arrager(n) => ("ARRANGER", n),
            CueCdText::Composer(n) => ("COMPOSER", n),
            CueCdText::DiscId(n) => ("DISC_ID", n),
            CueCdText::Genre(n) => ("GENRE", n),
            CueCdText::ISrc(n) => ("ISCR", n),
            CueCdText::Message(n) => ("MESSAGE", n),
            CueCdText::Performer(n) => ("PERFORMER", n),
            CueCdText::SongWriter(n) => ("SONGWRITER", n),
            CueCdText::Title(n) => ("TITLE", n),
            CueCdText::TocInfo(n) => ("TOC_INFO", n),
            CueCdText::TocInfo2(n) => ("TOC_INFO2", n),
            CueCdText::UpcEan(n) => ("UPC_EAN", n),
            CueCdText::SizeInfo(n) => ("SIZE_INFO", n),
        };
        let value = cue_format_string_value(&value);
        write!(f, "{} {}", key, value)
    }
}
