use std::collections::{BTreeMap, BTreeSet};

use crate::{
    cue_cd_text::CueCdText, cue_duration::{CueDuration, DurationFormat}, cue_track_flag::CueTrackFlag,
    cue_track_mode::CueTrackMode,
};

#[derive(Debug, Clone)]
pub struct CueTrack {
    track: (u32, CueTrackMode),
    flags: BTreeSet<CueTrackFlag>,
    pregap: Option<CueDuration>,
    postgap: Option<CueDuration>,
    cd_texts: BTreeSet<CueCdText>,
    rems: BTreeMap<String, String>,
    indexes: BTreeMap<u32, CueDuration>,
}

pub struct CueTrackBuilder {
    track: CueTrack,
}

impl CueTrackBuilder {
    pub fn new(track_index: u32, mode: CueTrackMode) -> Self {
        let track = CueTrack {
            track:(track_index, mode),
            flags: BTreeSet::new(),
            pregap: None,
            postgap: None,
            cd_texts: BTreeSet::new(),
            rems: BTreeMap::new(),
            indexes: BTreeMap::new(),
        };
        Self { track: track }
    }

    pub fn add_index(&mut self, index: u32, duration: DurationFormat) -> &mut Self {
        let _ = self.track.indexes.insert(index, duration.to_duration());
        self
    }

    pub fn add_flag(&mut self, flag: CueTrackFlag) -> &mut Self {
        let _ = self.track.flags.insert(flag);
        self
    }

    pub fn add_pregap(&mut self, duration: DurationFormat) -> &mut Self {
        let _ = self.track.pregap = Some(duration.to_duration());
        self
    }

    pub fn add_postgap(&mut self, duration: DurationFormat) -> &mut Self {
        let _ = self.track.postgap = Some(duration.to_duration());
        self
    }

    pub fn set_arranger(&mut self, arranger: &str) -> &mut Self {
        let arranger = CueCdText::Arrager(arranger.to_owned());
        let _ = self.track.cd_texts.insert(arranger);
        self
    }

    pub fn add_composer(&mut self, composer: &str) -> &mut Self {
        let composer = CueCdText::Composer(composer.to_string());
        let _ = self.track.cd_texts.insert(composer);
        self
    }

    pub fn add_disc_id(&mut self, composer: &str) -> &mut Self {
        let disc_id = CueCdText::DiscId(composer.to_owned());
        let _ = self.track.cd_texts.insert(disc_id);
        self
    }

    pub fn add_genre(&mut self, genre: &str) -> &mut Self {
        let genre = CueCdText::Genre(genre.to_string());
        let _ = self.track.cd_texts.insert(genre);
        self
    }

    pub fn add_iscr(&mut self, iscr: &str) -> &mut Self {
        let iscr = CueCdText::ISrc(iscr.to_string());
        let _ = self.track.cd_texts.insert(iscr);
        self
    }

    pub fn add_message(&mut self, message: &str) -> &mut Self {
        let message = CueCdText::Message(message.to_string());
        let _ = self.track.cd_texts.insert(message);
        self
    }
    
    pub fn add_performer(&mut self, performer: &str) -> &mut Self {
        let performer = CueCdText::Performer(performer.to_string());
        let _ = self.track.cd_texts.insert(performer);
        self
    }

    pub fn add_songwriter(&mut self, songwriter: &str) -> &mut Self {
        let songwriter = CueCdText::SongWriter(songwriter.to_string());
        let _ = self.track.cd_texts.insert(songwriter);
        self
    }

    pub fn add_title(&mut self, title: &str) -> &mut Self {
        let title = CueCdText::Title(title.to_owned());
        let _ = self.track.cd_texts.insert(title);
        self
    } 

    pub fn add_rem(&mut self, key: &str, value: &str) -> &mut Self {
        let key = key.to_ascii_uppercase();
        let _ = self.track.rems.insert(key, value.to_owned());
        self
    }


    pub fn track(self) -> CueTrack {
        self.track
    }
}
