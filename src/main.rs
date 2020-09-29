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
