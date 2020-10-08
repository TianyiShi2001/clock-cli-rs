use crate::utils::BoxedError;
use notify_rust::Notification;
use std::result::Result;

pub fn notify(msg: &str) -> Result<(), BoxedError> {
    Notification::new()
        .summary("Clock")
        .body(msg)
        .icon("clock")
        .show()?;
    Ok(())
}
