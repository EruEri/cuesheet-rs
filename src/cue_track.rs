// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
//  Copyright (C) 2023 Yves Ndiaye                                                            //
//                                                                                            //
// This Source Code Form is subject to the terms of the Mozilla Public                        //
// License, v. 2.0. If a copy of the MPL was not distributed with this                        //
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                                  //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    cue_cd_text::CueCdText,
    cue_duration::{CueDuration, DurationFormat},
    cue_track_flag::CueTrackFlag,
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

#[derive(Debug, Clone, Copy)]
pub enum ComputeKind {
    Set(CueDuration),
    Sum(CueDuration),
}

impl Eq for CueTrack {}

impl PartialEq for CueTrack {
    fn eq(&self, other: &Self) -> bool {
        self.track.0 == other.track.0
    }
}

impl PartialOrd for CueTrack {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.track.0.cmp(&other.track.0))
    }
}

impl Ord for CueTrack {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.track.0.cmp(&other.track.0)
    }
}

impl CueTrackBuilder {
    pub fn new(track_index: u32, mode: CueTrackMode) -> Self {
        let track = CueTrack {
            track: (track_index, mode),
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

    pub fn add_arranger(&mut self, arranger: &str) -> &mut Self {
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

impl CueTrack {
    pub fn track_offset(&self) -> CueDuration {
        self.indexes
            .iter()
            .fold(CueDuration::zero(), |acc_duration, (_, elt_duration)| {
                acc_duration + *elt_duration
            })
    }

    fn repr_cdtexts(&self, ctab: bool) -> String {
        let abs_tab = "  ";
        let tab = if ctab { abs_tab } else { "" };
        match self.cd_texts.is_empty() {
            true => String::new(),
            false => {
                let s = self
                    .cd_texts
                    .iter()
                    .map(CueCdText::to_string)
                    .collect::<Vec<String>>()
                    .join(format!("\n{}{}", abs_tab, tab).as_str());
                format!("{}{}", tab, s)
            }
        }
    }

    fn repr_flags(&self, tab: bool) -> String {
        let abs_tab = "  ";
        let tab = if tab { abs_tab } else { "" };
        match self.flags.is_empty() {
            true => String::new(),
            false => {
                let s = self
                    .flags
                    .iter()
                    .map(CueTrackFlag::to_string)
                    .collect::<Vec<_>>()
                    .join(abs_tab);
                format!("{}{}FLAGS{}\n", abs_tab, tab, s)
            }
        }
    }

    fn repr_rems(&self, ctab: bool) -> String {
        let abs_tab = "  ";
        let tab = if ctab { abs_tab } else { "" };
        let mapper = |(key, value)| format!("{}{}REM {} {}\n", abs_tab, tab, key, value);
        match self.rems.is_empty() {
            true => String::new(),
            false => self.rems.iter().map(mapper).collect::<Vec<_>>().join(""),
        }
    }

    fn repr_pregap(&self, ctab: bool) -> String {
        let abs_tab = "  ";
        let tab = if ctab { abs_tab } else { "" };
        self.pregap
            .map(|duration| format!("{}{}PREGAP {}\n", abs_tab, tab, duration))
            .unwrap_or(String::new())
    }

    fn repr_postgap(&self, ctab: bool) -> String {
        let abs_tab = "  ";
        let tab = if ctab { abs_tab } else { "" };
        self.pregap
            .map(|duration| format!("{}{}POSTGAP {}\n", abs_tab, tab, duration))
            .unwrap_or(String::new())
    }

    fn repr_indexes(&self, ctab: bool, compute: &Option<ComputeKind>) -> String {
        let abs_tab = "  ";
        let tab = if ctab { abs_tab } else { "" };
        let compute_duration = |duration: &CueDuration, comp| match comp {
            ComputeKind::Set(d) => d,
            ComputeKind::Sum(d) => *duration + d,
        };
        let string_of_index = |(track_index, duration)| {
            let s = compute
                .map(|kind| compute_duration(duration, kind))
                .unwrap_or(duration.clone())
                .to_string();
            format!("{}{}INDEX 0{} {}", abs_tab, tab, track_index, s)
        };
        match self.indexes.is_empty() {
            true => String::new(),
            false => self
                .indexes
                .iter()
                .map(string_of_index)
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }

    pub fn repr(&self, ctab: bool, compute: Option<ComputeKind>) -> String {
        let abs_tab = "  ";
        let tab = if ctab { abs_tab } else { "" };
        let str_track = format!("{}TRACK 0{} {}\n", tab, self.track.0, self.track.1);
        let str_pregap = self.repr_pregap(ctab);
        let str_postgap = self.repr_postgap(ctab);
        let str_cd_texts = self.repr_cdtexts(ctab);
        let str_flags = self.repr_flags(ctab);
        let str_rem = self.repr_rems(ctab);
        let str_indexes = self.repr_indexes(ctab, &compute);
        format!(
            "{}{}{}{}{}{}{}",
            str_track, str_cd_texts, str_flags, str_rem, str_pregap, str_postgap, str_indexes
        )
    }
}
