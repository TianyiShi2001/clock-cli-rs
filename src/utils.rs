use chrono::Duration;

pub trait PrettyDuration {
    fn pretty(&self) -> String;
}
impl PrettyDuration for Duration {
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx`
    fn pretty(&self) -> String {
        let s = self.num_seconds();
        let ms = self.num_milliseconds() - 1000 * s;
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
    }
}
