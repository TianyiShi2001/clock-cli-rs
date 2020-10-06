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

use chrono::Duration;

pub type BoxedError = Box<dyn std::error::Error>;

pub trait PrettyDuration {
    fn pretty(&self) -> String;
    fn pretty_s(&self) -> String;
}
impl PrettyDuration for Duration {
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx`
    fn pretty(&self) -> String {
        let s = self.num_seconds();
        let ms = self.num_milliseconds() - 1000 * s;
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
    }

    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx`
    fn pretty_s(&self) -> String {
        let mut s = self.num_seconds();
        let ms = self.num_milliseconds() - 1000 * s;
        if ms > 500 {
            s += 1;
        }
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        format!("{:02}:{:02}:{:02}", h, m, s)
    }
}
