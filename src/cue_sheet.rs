use std::collections::{BTreeSet, BTreeMap};

use crate::{cue_cd_text::CueCdText, cue_track::CueTrack, cue_file_format::CueFileFormat};


#[derive(Debug)]
pub struct CueSheet {
    catalog: Option<String>,
    cd_text_file: Option<String>,
    cd_texts: BTreeSet<CueCdText>,
    rems: BTreeMap<String, String>,
    file: (String, CueFileFormat),
    tracks: BTreeSet<CueTrack>
}


#[derive(Debug)]
pub struct CueSheetBuilder {
    sheet: CueSheet
}


impl CueSheetBuilder {
    pub fn new(filename: &str, format: CueFileFormat) -> Self {
        let sheet = CueSheet {
            catalog : None,
            cd_text_file : None,
            cd_texts : BTreeSet::new(),
            rems : BTreeMap::new(),
            file : (filename.to_string(),format),
            tracks : BTreeSet::new(),
        };
        Self { sheet: sheet }
    }

    pub fn add_arranger(&mut self, arranger: &str) -> &mut Self {
        let arranger = CueCdText::Arrager(arranger.to_owned());
        let _ = self.sheet.cd_texts.insert(arranger);
        self
    }

    pub fn add_composer(&mut self, composer: &str) -> &mut Self {
        let composer = CueCdText::Composer(composer.to_string());
        let _ = self.sheet.cd_texts.insert(composer);
        self
    }

    pub fn add_disc_id(&mut self, composer: &str) -> &mut Self {
        let disc_id = CueCdText::DiscId(composer.to_owned());
        let _ = self.sheet.cd_texts.insert(disc_id);
        self
    }

    pub fn add_genre(&mut self, genre: &str) -> &mut Self {
        let genre = CueCdText::Genre(genre.to_string());
        let _ = self.sheet.cd_texts.insert(genre);
        self
    }

    pub fn add_iscr(&mut self, iscr: &str) -> &mut Self {
        let iscr = CueCdText::ISrc(iscr.to_string());
        let _ = self.sheet.cd_texts.insert(iscr);
        self
    }

    pub fn add_message(&mut self, message: &str) -> &mut Self {
        let message = CueCdText::Message(message.to_string());
        let _ = self.sheet.cd_texts.insert(message);
        self
    }
    
    pub fn add_performer(&mut self, performer: &str) -> &mut Self {
        let performer = CueCdText::Performer(performer.to_string());
        let _ = self.sheet.cd_texts.insert(performer);
        self
    }

    pub fn add_songwriter(&mut self, songwriter: &str) -> &mut Self {
        let songwriter = CueCdText::SongWriter(songwriter.to_string());
        let _ = self.sheet.cd_texts.insert(songwriter);
        self
    }

    pub fn add_title(&mut self, title: &str) -> &mut Self {
        let title = CueCdText::Title(title.to_owned());
        let _ = self.sheet.cd_texts.insert(title);
        self
    } 

    pub fn add_rem(&mut self, key: &str, value: &str) -> &mut Self {
        let key = key.to_ascii_uppercase();
        let _ = self.sheet.rems.insert(key, value.to_owned());
        self
    }

    pub fn add_track(&mut self, track: CueTrack) -> &mut Self {
        let _ = self.sheet.tracks.insert(track);
        self
    }

    pub fn sheet(self) -> CueSheet {
        self.sheet
    }
}
