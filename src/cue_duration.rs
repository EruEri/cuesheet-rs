// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
//  Copyright (C) 2023 Yves Ndiaye                                                            //
//                                                                                            //
// This Source Code Form is subject to the terms of the Mozilla Public                        //
// License, v. 2.0. If a copy of the MPL was not distributed with this                        //
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                                  //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationFormat {
    MinSec(u32, u32),
    MinSecMil(u32, u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CueDuration {
    min: u32,
    sec: u32,
    frame: u32,
}

impl Display for CueDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CueDuration { min, sec, frame } = self;
        write!(f, "{:02}:{}:{}", min, sec, frame)
    }
}

impl Add for CueDuration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let CueDuration {
            min: lmin,
            sec: lsec,
            frame: lframe,
        } = self;
        let CueDuration {
            min: rmin,
            sec: rsec,
            frame: rframe,
        } = rhs;
        let sum_frame = lframe + rframe;
        let frame = sum_frame % 75;
        let retain_second = sum_frame / 75;
        let sum_second = lsec + rsec + retain_second;
        let seconds = sum_second % 60;
        let retain_minute = sum_second / 60;
        let minutes = lmin + rmin + retain_minute;
        Self {
            min: minutes,
            sec: seconds,
            frame: frame,
        }
    }
}

impl CueDuration {
    pub fn zero() -> Self {
        Self {
            min: 0,
            sec: 0,
            frame: 0,
        }
    }
}

impl Default for DurationFormat {
    fn default() -> Self {
        Self::MinSecMil(0, 0, 0)
    }
}

impl DurationFormat {
    pub fn minute_seconde_format(min: u32, sec: u32) -> Self {
        Self::MinSec(min, sec % 60)
    }

    pub fn minute_seconde_millieme_format(min: u32, sec: u32, mil: u32) -> Self {
        Self::MinSecMil(min, sec % 60, mil % 1000)
    }

    pub fn to_duration(&self) -> CueDuration {
        match self {
            DurationFormat::MinSec(min, sec) => CueDuration {
                min: *min,
                sec: *sec,
                frame: 0,
            },
            DurationFormat::MinSecMil(min, sec, mil) => {
                let frame = mil * 75 / 1000;
                CueDuration {
                    min: *min,
                    sec: *sec,
                    frame: frame,
                }
            }
        }
    }
}
