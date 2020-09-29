// Copyright (C) 2020 Tianyi Shi
//
// This file is part of clock-cli-rs.
//
// clock-cli-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// clock-cli-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with clock-cli-rs.  If not, see <http://www.gnu.org/licenses/>.

use chrono::{DateTime, Duration, Local};

/// A countdown timer
#[derive(Clone, Debug)]
pub struct Timer {
    pub remaining: Duration,
    pub total: Duration,
    pub actual_finish: DateTime<Local>,
    // pub actual_finish: DateTime<Local>,
    pub start_moments: Vec<DateTime<Local>>, // moments at which the timer resumes; the first is the start monent
    pub pause_moments: Vec<DateTime<Local>>, // moments at which the timer is paused; the last is the stop moment
    pub paused: bool,
}

impl Timer {
    /// Returns stopwatch reset to zero
    pub fn new(duration: Duration) -> Self {
        Self {
            remaining: duration,
            total: duration,
            actual_finish: Local::now(),
            start_moments: Vec::new(),
            pause_moments: Vec::new(),
            paused: true, // finished by default; start by explicitly calling `.resume()`
        }
    }

    pub fn last_start(&self) -> DateTime<Local> {
        self.start_moments[self.start_moments.len() - 1]
    }
    pub fn pause(&mut self) {
        assert!(self.paused == false, "Already paused!");
        let moment = Local::now();
        self.pause_moments.push(moment);
        self.remaining = self.remaining - (moment - self.last_start());
        self.paused = true;
    }
    pub fn resume(&mut self) {
        assert!(self.paused == true, "Already running!");
        self.start_moments.push(Local::now());
        self.paused = false;
    }
    pub fn pause_or_resume(&mut self) {
        if self.paused {
            self.resume();
        } else {
            self.pause();
        }
    }
    pub fn read(&self) -> Duration {
        if self.paused {
            self.remaining
        } else {
            self.remaining - (Local::now() - self.last_start())
        }
    }
}
