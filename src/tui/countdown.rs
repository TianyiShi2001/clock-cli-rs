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

//! # Countdown Timer TUI
//!
//! ## Expected Behavior
//!
//! On start, the user sets the expected duration (HH:MM:SS). Left/right arrow keys and the TAB key move the focus. A list of frequently/recently used durations is provided. Press "Enter" to start.
//!
//! When the timer is running, press "Space" to pause/resume and "Enter" to cancel.
//!
//! When the timer finishes (when counting to 00:00:00 or cancelled), the callback set with `on_finish()` is called.

use crate::utils::PrettyDuration;
// use chrono::{DateTime, Duration, Local};
use chrono::Duration;
use clock_core::timer::Timer;
use cursive::{
    event::{Callback, Event, EventResult, Key, MouseEvent},
    theme::ColorStyle,
    view::View,
    Cursive, Printer, Vec2, With,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone)]
enum TimerViewState {
    Config,
    Running,
    Finished,
}

struct TimerViewConfig {
    h: u8,
    m: u8,
    s: u8,
    focus: u8, // match focus % 3 {0 => h, 1 => m, 2 => s}
    input_buffer: Vec<u8>,
}

pub struct TimerView {
    timer: Timer,
    remaining: Duration,
    state: TimerViewState,
    config: TimerViewConfig,
    on_finish: Option<Rc<dyn Fn(&mut Cursive, Timer)>>,
}

impl TimerView {
    pub fn new(h: u8, m: u8, s: u8) -> Self {
        let config = TimerViewConfig {
            h,
            m,
            s,
            focus: 1,
            input_buffer: Vec::new(),
        };
        Self {
            timer: Timer::new(Duration::zero()),
            remaining: Duration::zero(),
            config,
            state: TimerViewState::Config,
            on_finish: None,
        }
    }

    pub fn start(&mut self) {
        let seconds =
            self.config.h as i64 * 3600 + self.config.m as i64 * 60 + self.config.s as i64;
        self.timer = Timer::new(Duration::seconds(seconds));
        self.state = TimerViewState::Running;
        self.timer.pause_or_resume();
    }

    /// Sets a callback to be used when `<Enter>` is pressed or counting to 00:00:00
    ///
    /// The elapsed time will be given to the callback.
    ///
    /// See also cursive::views::select_view::SelectView::set_on_submit
    pub fn set_on_finish<F, R>(&mut self, cb: F)
    where
        F: 'static + Fn(&mut Cursive, Timer) -> R,
    {
        self.on_finish = Some(Rc::new(move |s, t| {
            cb(s, t);
        }));
    }

    pub fn on_finish<F, R>(self, cb: F) -> Self
    where
        F: 'static + Fn(&mut Cursive, Timer) -> R,
    {
        self.with(|s| s.set_on_finish(cb))
    }

    fn finish(&mut self) -> EventResult {
        self.state = TimerViewState::Finished;
        let duration = self.timer.total;
        let timer = self.timer.clone();
        self.timer = Timer::new(duration);
        // TODO: remove clone
        // let timer = std::mem::replace(&mut self.timer, RefCell::new(Timer::new(duration))); // reset the timer data, but not other configurations related to the `View`
        if self.on_finish.is_some() {
            let cb = self.on_finish.clone().unwrap();
            EventResult::Consumed(Some(Callback::from_fn_once(move |s| cb(s, timer))))
        } else {
            EventResult::Consumed(None)
        }
    }

    fn draw_running(&self, printer: &Printer) {
        printer.print((0, 0), &self.remaining.pretty());
    }

    fn draw_finished(&self, printer: &Printer) {
        printer.print((0, 0), "FINISHED!");
    }

    fn draw_config(&self, printer: &Printer) {
        fn format(n: u8) -> String {
            format!("{:02}", n)
        }
        let (h, m, s) = (
            format(self.config.h),
            format(self.config.m),
            format(self.config.s),
        );

        if self.config.focus % 3 == 0 {
            printer.with_color(ColorStyle::highlight(), |printer| printer.print((0, 0), &h));
        } else {
            printer.print((0, 0), &h);
        }
        printer.print((2, 0), ":");
        if self.config.focus % 3 == 1 {
            printer.with_color(ColorStyle::highlight(), |printer| printer.print((3, 0), &m));
        } else {
            printer.print((3, 0), &m);
        }
        printer.print((5, 0), ":");
        if self.config.focus % 3 == 2 {
            printer.with_color(ColorStyle::highlight(), |printer| printer.print((6, 0), &s));
        } else {
            printer.print((6, 0), &s);
        }
    }

    fn get_selection(&self) -> u8 {
        match self.config.focus % 3 {
            0 => self.config.h,
            1 => self.config.m,
            2 => self.config.s,
            _ => unreachable!(),
        }
    }

    fn set_selection(&mut self, v: u8) {
        match self.config.focus % 3 {
            0 => self.config.h = v,
            1 => self.config.m = v,
            2 => self.config.s = v,
            _ => unreachable!(),
        }
    }

    fn move_focus_right(&mut self) {
        self.config.focus += 1;
        self.config.input_buffer.clear();
    }

    fn move_focus_left(&mut self) {
        self.config.focus -= 1;
        self.config.input_buffer.clear();
    }

    fn read_buffer(&self) -> u8 {
        let buffer = &self.config.input_buffer;
        let n = match buffer.len() {
            0 => 0,
            1 => buffer[0],
            2 => buffer[0] * 10 + buffer[1],
            _ => unreachable!(),
        };
        match self.config.focus % 3 {
            0 => n,
            1 | 2 => {
                if n < 60 {
                    n
                } else {
                    59
                }
            }
            _ => unreachable!(),
        }
    }
}
impl View for TimerView {
    fn draw(&self, printer: &Printer) {
        match self.state {
            TimerViewState::Running => self.draw_running(printer),
            TimerViewState::Config => self.draw_config(printer),
            TimerViewState::Finished => self.draw_finished(printer),
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        // the required size depends on how many lap times the user want to diaplay
        Vec2::new(12, 1) // columns, rows (width, height)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match self.state {
            TimerViewState::Running => {
                match event {
                    // pause/resume the timer when pressing "Space"
                    Event::Char(' ') => {
                        self.timer.pause_or_resume();
                    }
                    Event::Refresh => {
                        self.remaining = self.timer.read();
                        if self.remaining.num_milliseconds() < 10 {
                            return self.finish();
                        }
                    }
                    // calcel
                    Event::Key(Key::Enter) => {
                        return self.finish();
                    }
                    _ => {
                        if self.timer.remaining.num_milliseconds() < 10 {
                            self.state = TimerViewState::Finished;
                            return self.finish();
                        }
                    } //return EventResult::Ignored,
                }
            }
            TimerViewState::Finished => match event {
                Event::Char(' ') | Event::Key(Key::Enter) => {
                    self.state = TimerViewState::Config;
                }
                _ => return EventResult::Ignored,
            },
            TimerViewState::Config => match event {
                Event::Char(c) => {
                    if c.is_numeric() {
                        self.config.input_buffer.push(c.to_digit(10).unwrap() as u8);
                    }
                    self.set_selection(self.read_buffer());
                    if self.config.input_buffer.len() == 2 {
                        self.move_focus_right();
                    }
                }
                Event::Key(Key::Right) | Event::Key(Key::Tab) => {
                    self.move_focus_right();
                }
                Event::Key(Key::Left) => self.move_focus_left(),
                Event::Key(Key::Enter) => {
                    self.start();
                }
                // Event::Mouse {
                //     offset,
                //     position,
                //     event,
                // } => match event {
                //     MouseEvent::WheelDown => self.set_selection(self.get_selection() - 1),
                //     MouseEvent::WheelUp => self.set_selection(self.get_selection() + 1),
                // },
                _ => return EventResult::Ignored,
            },
        }
        EventResult::Consumed(None)
    }
}
