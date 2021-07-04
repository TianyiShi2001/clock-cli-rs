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

use hhmmss::Hhmmss;
// use chrono::{DateTime, Duration, Local};
use clock_core::stopwatch::{Stopwatch, StopwatchData};
use cursive::{
    event::{Callback, Event, EventResult, Key, MouseEvent},
    view::View,
    Cursive, Printer, Vec2, With,
};
use std::rc::Rc;

#[derive(Default)]
pub struct StopwatchView {
    stopwatch: Stopwatch,
    on_stop: Option<Rc<dyn Fn(&mut Cursive, StopwatchData)>>,
    show_laps: usize,
    show_laps_offset: usize,
}

impl StopwatchView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_laps(mut self, n: usize) -> Self {
        self.show_laps = n;
        self
    }

    pub fn set_on_stop<F, R>(&mut self, cb: F)
    where
        F: 'static + Fn(&mut Cursive, StopwatchData) -> R,
    {
        self.on_stop = Some(Rc::new(move |s, t| {
            cb(s, t);
        }));
    }

    pub fn on_stop<F, R>(self, cb: F) -> Self
    where
        F: 'static + Fn(&mut Cursive, StopwatchData) -> R,
    {
        self.with(|s| s.set_on_stop(cb))
    }

    fn stop(&mut self) -> EventResult {
        let data = self.stopwatch.stop();
        if self.on_stop.is_some() {
            let cb = self.on_stop.clone().unwrap();
            EventResult::Consumed(Some(Callback::from_fn_once(move |s| cb(s, data))))
        } else {
            EventResult::Consumed(None)
        }
    }

    fn increment_show_lap_offset(&mut self) {
        if self.stopwatch.data.laps.len() > self.show_laps + self.show_laps_offset {
            self.show_laps_offset += 1;
        }
    }

    fn decrement_show_lap_offset(&mut self) {
        if self.show_laps_offset > 0 {
            self.show_laps_offset -= 1;
        }
    }
}
impl View for StopwatchView {
    fn draw(&self, printer: &Printer) {
        printer.print((4, 0), &self.stopwatch.read().hhmmssxxx());

        let len = self.stopwatch.data.laps.len() - self.show_laps_offset;
        let mut i = 0;
        while i < std::cmp::min(len, self.show_laps) {
            i += 1;

            printer.print(
                (0, i),
                &[
                    format!("Lap {:02}: ", len - i + 1),
                    self.stopwatch.data.laps[len - i].hhmmssxxx(),
                ]
                .concat(),
            );
        }
        if len != i {
            printer.print((0, self.show_laps), ":                           ");
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
            Event::Key(Key::Enter) | Event::Char('q') | Event::Key(Key::Backspace) => {
                self.show_laps_offset = 0; // FUTURE: maybe unneeded?
                return self.stop();
            }
            Event::Char('l') => {
                self.stopwatch.lap();
                self.show_laps_offset = 0;
            }
            Event::Key(Key::Up) => {
                self.decrement_show_lap_offset();
            }
            Event::Key(Key::Down) => {
                self.increment_show_lap_offset();
            }
            Event::Mouse { event, .. } => {
                match event {
                    MouseEvent::WheelUp => {
                        self.decrement_show_lap_offset();
                    }
                    MouseEvent::WheelDown => {
                        self.increment_show_lap_offset();
                    }
                    _ => return EventResult::Ignored,
                }
                return EventResult::Consumed(None);
            }
            _ => return EventResult::Ignored,
        }
        EventResult::Consumed(None)
    }
}
