use crate::utils::PrettyDuration;
// use chrono::{DateTime, Duration, Local};
use crate::core::countdown::Timer;
use cursive::{
    event::{Event, EventResult, Key},
    view::View,
    Cursive, Printer, Vec2, With,
};
use std::rc::Rc;

pub struct TimerView {
    timer: Timer,
    on_stop: Option<Rc<dyn Fn(&mut Cursive, &Timer)>>,
}

impl TimerView {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            on_stop: None,
            show_laps: 0,
        }
    }

    /// Sets a callback to be used when `<Enter>` is pressed.
    ///
    /// The elapsed time will be given to the callback.
    ///
    /// See also cursive::views::select_view::SelectView::set_on_submit
    pub fn set_on_stop<F, R>(&mut self, cb: F)
    where
        F: 'static + Fn(&mut Cursive, &Timer) -> R,
    {
        self.on_stop = Some(Rc::new(move |s, t| {
            cb(s, t);
        }));
    }

    pub fn on_stop<F, R>(self, cb: F) -> Self
    where
        F: 'static + Fn(&mut Cursive, &Timer) -> R,
    {
        self.with(|s| s.set_on_stop(cb))
    }

    fn stop(&mut self) -> EventResult {
        let timer = &mut self.timer;
        if timer.paused {
            timer.resume(); // to record the last lap
        }
        timer.lap();
        timer.pause();
        let result = if self.on_stop.is_some() {
            let cb = self.on_stop.clone().unwrap();
            let timer_data = self.timer.clone(); // TODO: remove clone
            EventResult::with_cb(move |s| cb(s, &timer_data))
        } else {
            EventResult::Consumed(None)
        };
        // reset the timer data, but not other configurations related to the `View`
        self.timer = Timer::new();
        // return result
        result
    }
}
impl View for TimerView {
    fn draw(&self, printer: &Printer) {
        printer.print((4, 0), &self.timer.read().pretty());
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        // the required size depends on how many lap times the user want to diaplay
        Vec2::new(12, 1) // columns, rows (width, height)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            // pause/resume the timer when pressing "Space"
            Event::Char(' ') => {
                self.timer.pause_or_resume();
            }
            Event::Key(Key::Enter) => {
                return self.stop();
            }
            _ => return EventResult::Ignored,
        }
        EventResult::Consumed(None)
    }
}
