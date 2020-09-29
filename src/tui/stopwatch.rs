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

use crate::utils::PrettyDuration;
// use chrono::{DateTime, Duration, Local};
use crate::core::stopwatch::StopWatch;
use cursive::{
    event::{Callback, Event, EventResult, Key},
    view::View,
    Cursive, Printer, Vec2, With,
};
use std::rc::Rc;

pub struct StopWatchView {
    stopwatch: StopWatch,
    on_stop: Option<Rc<dyn Fn(&mut Cursive, StopWatch)>>,
    show_laps: usize,
}

impl StopWatchView {
    pub fn new() -> Self {
        Self {
            stopwatch: StopWatch::new(),
            on_stop: None,
            show_laps: 0,
        }
    }

    pub fn with_laps(mut self, n: usize) -> Self {
        self.show_laps = n;
        self
    }

    pub fn set_on_stop<F, R>(&mut self, cb: F)
    where
        F: 'static + Fn(&mut Cursive, StopWatch) -> R,
    {
        self.on_stop = Some(Rc::new(move |s, t| {
            cb(s, t);
        }));
    }

    pub fn on_stop<F, R>(self, cb: F) -> Self
    where
        F: 'static + Fn(&mut Cursive, StopWatch) -> R,
    {
        self.with(|s| s.set_on_stop(cb))
    }

    fn stop(&mut self) -> EventResult {
        let stopwatch = &mut self.stopwatch;
        if stopwatch.paused {
            stopwatch.resume(); // to record the last lap
        }
        stopwatch.lap();
        stopwatch.pause();
        let stopwatch = std::mem::replace(stopwatch, StopWatch::new()); // reset the stopwatch data, but not other configurations related to the `View`
        if self.on_stop.is_some() {
            let cb = self.on_stop.clone().unwrap();
            EventResult::Consumed(Some(Callback::from_fn_once(move |s| cb(s, stopwatch))))
        } else {
            EventResult::Consumed(None)
        }
    }
}
impl View for StopWatchView {
    fn draw(&self, printer: &Printer) {
        printer.print((4, 0), &self.stopwatch.read().pretty());
        let len = self.stopwatch.laps.len();
        for i in 1..=std::cmp::min(len, self.show_laps) {
            printer.print(
                (0, i),
                &[
                    format!("Lap {:02}: ", len - i + 1),
                    self.stopwatch.laps[len - i].pretty(),
                ]
                .concat(),
            );
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        // the required size depends on how many lap times the user want to diaplay
        Vec2::new(20, self.show_laps + 1) // columns, rows (width, height)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            // pause/resume the stopwatch when pressing "Space"
            Event::Char(' ') => {
                self.stopwatch.pause_or_resume();
            }
            Event::Key(Key::Enter) => {
                return self.stop();
            }
            Event::Char('l') => {
                self.stopwatch.lap();
            }
            _ => return EventResult::Ignored,
        }
        EventResult::Consumed(None)
    }
}
