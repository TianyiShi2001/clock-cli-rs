mod countdown;
mod stopwatch;
use crate::core::{countdown::Timer, stopwatch::StopWatch};
use crate::utils::PrettyDuration;
use countdown::TimerView;
use cursive::{traits::*, views::Dialog, Cursive};
use stopwatch::StopWatchView;

pub fn stopwatch() {
    let mut siv = cursive::default();
    let stopwatch = StopWatchView::new();
    siv.add_layer(
        stopwatch
            .with_laps(8)
            .on_stop(|s: &mut Cursive, stopwatch| s.add_layer(Dialog::info(summarize(&stopwatch))))
            .with_name("stopwatch"),
    );
    siv.set_fps(15);
    siv.run();
}

fn summarize(stopwatch: &StopWatch) -> String {
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
