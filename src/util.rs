// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
//  Copyright (C) 2023 Yves Ndiaye                                                            //
//                                                                                            //
// This Source Code Form is subject to the terms of the Mozilla Public                        //
// License, v. 2.0. If a copy of the MPL was not distributed with this                        //
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                                  //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn cue_format_string_value(s: &str) -> String {
    match s.contains(' ') {
        true => format!("\"{}\"", s),
        false => s.to_string(),
    }
}
