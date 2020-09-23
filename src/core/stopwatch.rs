use chrono::{DateTime, Duration, Local};

/// A stopwatch that mimics iOS's stopwatch
///
/// ```ignore
///                  lap    lap          lap
/// start       start |      |     start  |
///   o--------x   o-----------x      o-----------x
///          pause           pause            pause(end)
/// ```
#[derive(Clone, Debug)]
pub struct StopWatch {
    pub elapsed: Duration,                   // total elapsed time
    pub lap_elapsed: Duration,               // elapsed time of the current lap
    pub pause_moments: Vec<DateTime<Local>>, // moments at which the stopwatch is paused
    pub start_moments: Vec<DateTime<Local>>, // moments at which the stopwatch resumes
    pub lap_moments: Vec<DateTime<Local>>,   // moments at which a lap time is read
    pub laps: Vec<Duration>,                 // lap times
    pub paused: bool,
}

impl StopWatch {
    /// Returns stopwatch reset to zero
    pub fn new() -> Self {
        Self {
            elapsed: Duration::zero(),
            lap_elapsed: Duration::zero(),
            start_moments: Vec::new(),
            pause_moments: Vec::new(),
            lap_moments: Vec::new(),
            laps: Vec::new(),
            paused: true, // stopped by default; start by explicitly calling `.resume()`
        }
    }

    pub fn last_start(&self) -> DateTime<Local> {
        self.start_moments[self.start_moments.len() - 1]
    }
    pub fn last_lap(&self) -> DateTime<Local> {
        self.lap_moments[self.lap_moments.len() - 1]
    }
    pub fn pause(&mut self) {
        assert!(self.paused == false, "Already paused!");
        let moment = Local::now();
        self.pause_moments.push(moment);
        self.elapsed = self.elapsed + (moment - self.last_start());
        self.lap_elapsed = self.read_lap_elapsed(moment);
        self.paused = true;
    }
    pub fn resume(&mut self) {
        assert!(self.paused == true, "Already running!");
        self.start_moments.push(Local::now());
        self.paused = false;
    }
    pub fn pause_or_resume(&mut self) {
        if self.paused {
            self.resume();
        } else {
            self.pause();
        }
    }
    pub fn lap(&mut self) -> Option<Duration> {
        // assert!(!self.paused, "Paused!");
        if self.paused {
            None
        } else {
            let moment = Local::now();
            let lap = self.read_lap_elapsed(moment);
            self.lap_moments.push(moment);
            self.laps.push(lap);
            self.lap_elapsed = Duration::zero();
            Some(lap)
        }
    }
    /// Read the total time elapsed
    pub fn read(&self) -> Duration {
        if self.paused {
            self.elapsed
        } else {
            self.elapsed + (Local::now() - self.last_start())
        }
    }
    /// Read the time elapsed in the current lap
    pub fn read_lap_elapsed(&self, moment: DateTime<Local>) -> Duration {
        self.lap_elapsed
            + if self.lap_elapsed == Duration::zero() && !self.lap_moments.is_empty() {
                moment - self.last_lap()
            } else {
                moment - self.last_start()
            }
    }
}
