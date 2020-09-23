// use chrono::Duration;
// use cursive::{traits::*, views::Dialog, Cursive};

// fn summarize(stopwatch: &StopWatch::StopWatch) -> String {
//     let elapsed = stopwatch.elapsed;
//     let average = stopwatch.elapsed / stopwatch.laps.len() as i32;
//     let max = stopwatch.laps.iter().max().unwrap();
//     let min = stopwatch.laps.iter().min().unwrap();
//     format!(
//         "Elapsed time: {}\nAverage: {}\nMax: {}\nMin: {}",
//         elapsed.pretty(),
//         average.pretty(),
//         max.pretty(),
//         min.pretty()
//     )
// }

//pub mod countdown;
mod stopwatch;
use crate::core::stopwatch::StopWatch;
use crate::utils::PrettyDuration;
use cursive::{traits::*, views::Dialog, Cursive};
use stopwatch::StopWatchView;

pub fn main() {
    let mut siv = cursive::default();
    let stopwatch = StopWatchView::new();
    siv.add_layer(
        stopwatch
            .with_laps(8)
            .on_stop(|s: &mut Cursive, stopwatch| s.add_layer(Dialog::info(summarize(&stopwatch))))
            .with_name("stopwatch"),
    );
    siv.add_layer(Dialog::info(
        "Press 'Space' to start/pause/resume the stopwatch\nPress 'l' to record lap time\nPress 'Enter' to stop",
    ));
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
