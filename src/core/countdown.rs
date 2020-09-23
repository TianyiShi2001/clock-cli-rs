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
    /// Read the total time elapsed
    pub fn read(&self) -> Duration {
        if self.paused {
            self.remaining
        } else {
            self.remaining - (Local::now() - self.last_start())
        }
    }
}
