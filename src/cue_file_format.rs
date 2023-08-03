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
