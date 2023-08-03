// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
//  Copyright (C) 2023 Yves Ndiaye                                                            //
//                                                                                            //
// This Source Code Form is subject to the terms of the Mozilla Public                        //
// License, v. 2.0. If a copy of the MPL was not distributed with this                        //
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                                  //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) mod cue_cd_text;
pub(crate) mod cue_duration;
pub(crate) mod cue_file_format;
pub(crate) mod cue_sheet;
pub(crate) mod cue_track;
pub(crate) mod cue_track_flag;
pub(crate) mod cue_track_mode;
pub(crate) mod util;

pub use crate::cue_cd_text::CueCdText;
pub use crate::cue_duration::{CueDuration, DurationFormat};
pub use crate::cue_file_format::CueFileFormat;
pub use crate::cue_sheet::{CueSheet, CueSheetBuilder};
pub use crate::cue_track::{ComputeKind, CueTrack};
pub use crate::cue_track_flag::CueTrackFlag;
pub use crate::cue_track_mode::CueTrackMode;

#[cfg(test)]
mod test {
    use crate::CueTrack;


    #[test]
    fn test_iridescent_vision() {
        let mut dizzy = CueTrack::new(1, crate::CueTrackMode::AUDIO);
        let _ = dizzy
        .add_performer("喜多村英梨")
        .add_composer("Village wood")
        .add_title("D!zzy...&")
        .add_index(1, crate::DurationFormat::MinSecMil(3, 54, 992));

        let mut synapse = CueTrack::new(2, crate::CueTrackMode::AUDIO);
        let _ = synapse
        .add_performer("喜多村英梨")
        .add_composer("古宇田亘d")
        .add_title("SynApsE")
        .add_index(1, crate::DurationFormat::MinSecMil(3, 15, 827));

        let mut egoism = CueTrack::new(3, crate::CueTrackMode::AUDIO);
        let _ = egoism
        .add_performer("喜多村英梨")
        .add_composer("Village wood")
        .add_title("ヱゴヰズム")
        .add_index(1, crate::DurationFormat::MinSecMil(3, 22, 147));

        let mut anger = CueTrack::new(4, crate::CueTrackMode::AUDIO);
        let _ = anger
        .add_performer("喜多村英梨")
        .add_composer("Han")
        .add_title("ANGER%")
        .add_index(1, crate::DurationFormat::MinSecMil(5, 12, 449));

        let mut kindan_kajitsu = CueTrack::new(5, crate::CueTrackMode::AUDIO);
        let _ = kindan_kajitsu
        .add_performer("喜多村英梨")
        .add_composer("Han")
        .add_title("禁断果実")
        .add_index(1, crate::DurationFormat::MinSecMil(3, 12, 565));

        let mut belief_in_oneself = CueTrack::new(6, crate::CueTrackMode::AUDIO);
        let _ = belief_in_oneself
        .add_performer("喜多村英梨")
        .add_composer("Han")
        .add_title("Belief in Oneself")
        .add_index(1, crate::DurationFormat::MinSecMil(3, 29, 798));
    
        let mut shine_going_up = CueTrack::new(7, crate::CueTrackMode::AUDIO);
        let _ = shine_going_up
        .add_performer("喜多村英梨")
        .add_composer("YutoMinami")
        .add_title("SH!NE GO!NG UP")
        .add_index(1, crate::DurationFormat::MinSecMil(4, 23, 883));

        let mut holy_shit = CueTrack::new(8, crate::CueTrackMode::AUDIO);
        let _ = holy_shit
        .add_performer("喜多村英梨")
        .add_composer("Village wood")
        .add_title("HOLy×SH!T")
        .add_index(1, crate::DurationFormat::MinSecMil(4, 29, 836));

        let mut eternity = CueTrack::new(9, crate::CueTrackMode::AUDIO);
        let _ = eternity
        .add_performer("喜多村英梨")
        .add_composer("Village wood")
        .add_title("ETERNiTY")
        .add_index(1, crate::DurationFormat::MinSecMil(4, 32, 006));

        let mut nijiiro = CueTrack::new(10, crate::CueTrackMode::AUDIO);
        let _ = nijiiro
        .add_performer("喜多村英梨")
        .add_composer("古宇田亘")
        .add_title("虹色")
        .add_index(1, crate::DurationFormat::MinSecMil(3, 59, 187));

        let mut egoism_mix = CueTrack::new(11, crate::CueTrackMode::AUDIO);
        let _ = egoism_mix
        .add_performer("喜多村英梨")
        .add_composer("Village wood")
        .add_title("ヱゴヰズム (- Pf MiX -)")
        .add_index(1, crate::DurationFormat::MinSecMil(3, 33, 684));

        let mut kindan_kajitsu_mix = CueTrack::new(12, crate::CueTrackMode::AUDIO);
        let _ = kindan_kajitsu_mix
        .add_performer("喜多村英梨")
        .add_composer("Han")
        .add_title("禁断果実 (- Pf MiX -)")
        .add_index(1, crate::DurationFormat::MinSecMil(3, 22, 723));

        let mut shine_going_up_mix = CueTrack::new(13, crate::CueTrackMode::AUDIO);
        let _ = shine_going_up_mix
        .add_performer("喜多村英梨")
        .add_composer("YutoMinami")
        .add_title("SH!NE GO!NG UP (- Pf MiX -)")
        .add_index(1, crate::DurationFormat::MinSecMil(4, 40, 965));

    
    }
}