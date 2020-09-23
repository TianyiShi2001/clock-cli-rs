use chrono::{DateTime, Duration, Local};

/// A countdown timer
#[derive(Clone, Debug)]
pub struct Timer {
    pub remaining: Duration,
    pub expected_stop: DateTime<Local>,
    // pub actual_stop: DateTime<Local>,
    pub start_moments: Vec<DateTime<Local>>, // moments at which the timer resumes; the first is the start monent
    pub pause_moments: Vec<DateTime<Local>>, // moments at which the timer is paused; the last is the stop moment
    pub paused: bool,
}

impl Timer {
    /// Returns stopwatch reset to zero
    pub fn new(duration: Duration) -> Self {
        let moment = Local::now();
        let expected_stop = moment + duration;
        Self {
            remaining: duration,
            expected_stop,
            start_moments: Vec::new(),
            pause_moments: Vec::new(),
            paused: true, // stopped by default; start by explicitly calling `.resume()`
        }
    }

    pub fn last_start(&self) -> DateTime<Local> {
        self.start_moments[self.start_moments.len() - 1]
    }
    pub fn pause(&mut self) {
        assert!(self.paused == false, "Already paused!");
        let moment = Local::now();
        self.pause_moments.push(moment);
        self.remaining = self.remaining - (moment - self.last_start());
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
