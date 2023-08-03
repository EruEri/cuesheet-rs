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
