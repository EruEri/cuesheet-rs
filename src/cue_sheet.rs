// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
//  Copyright (C) 2023 Yves Ndiaye                                                            //
//                                                                                            //
// This Source Code Form is subject to the terms of the Mozilla Public                        //
// License, v. 2.0. If a copy of the MPL was not distributed with this                        //
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                                  //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::OpenOptions,
    io::Write,
    ops::Add,
};

use crate::{
    cue_cd_text::CueCdText,
    cue_duration::CueDuration,
    cue_file_format::CueFileFormat,
    cue_track::{ComputeKind, CueTrack},
    util::cue_format_string_value,
};

#[derive(Debug)]
pub struct CueSheet {
    catalog: Option<String>,
    cd_text_file: Option<String>,
    cd_texts: BTreeSet<CueCdText>,
    rems: BTreeMap<String, String>,
    file: (String, CueFileFormat),
    tracks: BTreeSet<CueTrack>,
}

impl CueSheet {
    fn repr_catalog(&self) -> String {
        self.catalog
            .iter()
            .map(|s| {
                let s = cue_format_string_value(&s);
                format!("CATALOG {}\n", s)
            })
            .collect::<String>()
    }

    fn repr_cdtextfile(&self) -> String {
        self.cd_text_file
            .iter()
            .map(|s| {
                let s = cue_format_string_value(&s);
                format!("CDTEXTFILE {}\n", s)
            })
            .collect::<String>()
    }

    fn repr_cdtexts(&self) -> String {
        match self.cd_texts.is_empty() {
            true => String::new(),
            false => {
                let s = self
                    .cd_texts
                    .iter()
                    .map(CueCdText::to_string)
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("{}\n", s)
            }
        }
    }

    fn repr_rems(&self) -> String {
        let mapper = |(key, value)| format!("REM {} {}\n", key, value);
        match self.rems.is_empty() {
            true => String::new(),
            false => self.rems.iter().map(mapper).collect::<Vec<_>>().join(""),
        }
    }

    fn repr_file(&self) -> String {
        let (ref name, ref format) = self.file;
        format!("FILE \"{}\" {}\n", name, format)
    }

    fn repr_tracks(&self, sum: bool) -> String {
        self.tracks
            .iter()
            .scan(CueDuration::zero(), |state, track| {
                let offset = *state;
                *state = track.track_offset().add(*state);
                let compute = if sum {
                    Some(ComputeKind::Set(offset))
                } else {
                    None
                };
                Some(track.repr(true, compute))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn repr(&self, sum: bool) -> String {
        let str_catalog = self.repr_catalog();
        let str_cd_text_file = self.repr_cdtextfile();
        let str_cd_texts = self.repr_cdtexts();
        let str_rems = self.repr_rems();
        let str_file = self.repr_file();
        let str_tracks = self.repr_tracks(sum);
        format!(
            "{}{}{}{}{}{}",
            str_catalog, str_cd_text_file, str_cd_texts, str_rems, str_file, str_tracks
        )
    }
}

impl CueSheet {
    pub fn new(filename: &str, format: CueFileFormat) -> Self {
        let sheet = Self {
            catalog: None,
            cd_text_file: None,
            cd_texts: BTreeSet::new(),
            rems: BTreeMap::new(),
            file: (filename.to_string(), format),
            tracks: BTreeSet::new(),
        };
        sheet
    }

    pub fn add_catalog(&mut self, catalog: &str) -> &mut Self {
        self.catalog = Some(catalog.to_string());
        self
    }

    pub fn add_arranger(&mut self, arranger: &str) -> &mut Self {
        let arranger = CueCdText::Arrager(arranger.to_owned());
        let _ = self.cd_texts.insert(arranger);
        self
    }

    pub fn add_composer(&mut self, composer: &str) -> &mut Self {
        let composer = CueCdText::Composer(composer.to_string());
        let _ = self.cd_texts.insert(composer);
        self
    }

    pub fn add_disc_id(&mut self, composer: &str) -> &mut Self {
        let disc_id = CueCdText::DiscId(composer.to_owned());
        let _ = self.cd_texts.insert(disc_id);
        self
    }

    pub fn add_genre(&mut self, genre: &str) -> &mut Self {
        let genre = CueCdText::Genre(genre.to_string());
        let _ = self.cd_texts.insert(genre);
        self
    }

    pub fn add_iscr(&mut self, iscr: &str) -> &mut Self {
        let iscr = CueCdText::ISrc(iscr.to_string());
        let _ = self.cd_texts.insert(iscr);
        self
    }

    pub fn add_message(&mut self, message: &str) -> &mut Self {
        let message = CueCdText::Message(message.to_string());
        let _ = self.cd_texts.insert(message);
        self
    }

    pub fn add_performer(&mut self, performer: &str) -> &mut Self {
        let performer = CueCdText::Performer(performer.to_string());
        let _ = self.cd_texts.insert(performer);
        self
    }

    pub fn add_songwriter(&mut self, songwriter: &str) -> &mut Self {
        let songwriter = CueCdText::SongWriter(songwriter.to_string());
        let _ = self.cd_texts.insert(songwriter);
        self
    }

    pub fn add_title(&mut self, title: &str) -> &mut Self {
        let title = CueCdText::Title(title.to_owned());
        let _ = self.cd_texts.insert(title);
        self
    }

    pub fn add_rem(&mut self, key: &str, value: &str) -> &mut Self {
        let key = key.to_ascii_uppercase();
        let _ = self.rems.insert(key, value.to_owned());
        self
    }

    pub fn add_track(&mut self, track: CueTrack) -> &mut Self {
        let _ = self.tracks.insert(track);
        self
    }

    pub fn export<P: AsRef<std::path::Path>>(
        &self,
        sum: bool,
        outfile: P,
    ) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .read(false)
            .open(outfile)?;
        let str_repr = self.repr(sum);
        file.write_all(str_repr.as_bytes())
    }
}
