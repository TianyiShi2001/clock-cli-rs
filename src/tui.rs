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

mod countdown;
mod stopwatch;
use crate::utils::PrettyDuration;
use clock_core::{
    stopwatch::{Stopwatch, StopwatchData},
    timer::Timer,
};
use countdown::TimerView;
use cursive::{traits::*, views::Dialog, Cursive};
use stopwatch::StopwatchView;

pub fn stopwatch() {
    let mut siv = cursive::default();
    let stopwatch = StopwatchView::new();
    siv.add_layer(
        stopwatch
            .with_laps(8)
            .on_stop(|s: &mut Cursive, stopwatch| s.add_layer(Dialog::info(summarize(&stopwatch))))
            .with_name("stopwatch"),
    );
    siv.set_fps(15);
    siv.run();
}

fn summarize(stopwatch: &StopwatchData) -> String {
    let elapsed = stopwatch.elapsed;
    let average = stopwatch.elapsed / stopwatch.laps.len() as i32;
    let max = stopwatch.laps.iter().max().unwrap();
    let min = stopwatch.laps.iter().min().unwrap();
    format!(
        "Elapsed time: {}\nAverage: {}\nMax: {}\nMin: {}",
        elapsed.pretty(),
        average.pretty(),
        max.pretty(),
        min.pretty()
    )
}

pub fn timer(h: u8, m: u8, s: u8) {
    let mut siv = cursive::default();
    let timer = TimerView::new(h, m, s);
    siv.add_layer(
        timer, // .on_finish(|s: &mut Cursive, timer| s.add_layer(Dialog::info(format!("{:?}", &timer)))),
    );
    //siv.set_fps(15);
    siv.set_autorefresh(true);
    siv.run();
}
