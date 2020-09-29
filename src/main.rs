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

use clock_cli::tui;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        tui::stopwatch();
    } else {
        let duration = args.join(" ");
        let duration: std::time::Duration = duration.parse::<humantime::Duration>().unwrap().into();
        let duration = chrono::Duration::from_std(duration).unwrap();
        let (h, m, s) = hms(duration);
        tui::timer(h as u8, m as u8, s as u8);
    }
}

fn hms(duration: chrono::Duration) -> (i64, i64, i64) {
    let s = duration.num_seconds();
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    (h, m, s)
}
