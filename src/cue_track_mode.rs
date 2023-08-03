// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
//  Copyright (C) 2023 Yves Ndiaye                                                            //
//                                                                                            //
// This Source Code Form is subject to the terms of the Mozilla Public                        //
// License, v. 2.0. If a copy of the MPL was not distributed with this                        //
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                                  //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

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
