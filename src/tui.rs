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

mod stopwatch;
mod timer;
use crate::{notify::notify, utils::PrettyDuration};
use clock_core::{
    stopwatch::{Stopwatch, StopwatchData},
    timer::{Timer, TimerData},
};
use cursive::{traits::*, views::Dialog, Cursive};
pub use stopwatch::StopwatchView;
pub use timer::TimerView;

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

fn timer_on_finish(data: TimerData) {
    let expected_duration = data.duration_expected().pretty_s();
    let actual_duration = data.duration_actual().pretty_s();
    let msg = &format!(
        "Expected: {}\nActual: {}",
        &expected_duration, &actual_duration,
    );

    notify(msg).unwrap();

    match notify(msg) {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[allow(dead_code)]
fn timer_on_finish_debug(s: &mut Cursive, data: TimerData) {
    s.add_layer(Dialog::info(format!("{:?}", data)));
}

pub fn timer(h: u8, m: u8, s: u8) {
    let mut siv = cursive::default();
    let timer = TimerView::new(h, m, s);
    siv.add_layer(timer.on_finish(|s: &mut Cursive, timer| timer_on_finish(timer)));
    //siv.set_fps(15);
    siv.set_autorefresh(true);
    siv.run();
}
