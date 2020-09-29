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

//! # clock-cli-rs
//!
//! Command line clock utilities, with TUI interfaces, implemented in Rust. Currently, these features are implemented:
//!
//! - Stopwatch
//!   - start/pause/stop: ✅
//!   - lap time (similar to iOS's stopwatch's behaviour): ✅
//!   - report of all pause/start/lap instances (moments): WIP
//! - (Countdown) Timer
//!   - basics: ✅
//!
//! # Installation
//!
//! If you are a Rustacean 🦀️, just `cargo install clock-cli`.
//!
//! Other installation methods: WIP
//!
//! # Usage
//!
//! ## Stopwatch:
//!
//! simply run:
//!
//! ```
//! clock
//! ```
//!
//! - Press `Space` to pause/resume.
//! - Press `l` to lap.
//! - Press `return` to finish.
//!
//! ## Countdown Timer:
//!
//! Specify the duration (in natual language) to run a countdown.
//!
//! Examples:
//!
//! ```
//! clock 3 minutes
//! clock 4h3m
//! clock 1 day
//! ```
//!
//! - Press `Space` to pause/resume.
//! - Press `return` to cancel.
//!
//! # Compatibility
//!
//! Currently only works on Linux and MacOS.
//!
//! # Acknowledgement
//!
//! The TUI is based on the [**cursive**](https://github.com/gyscos/cursive) crate made by [Alexandre Bury (@glycos)](https://github.com/gyscos), who also helped me a lot during the development of this crate (see [glycose/cursive/#503](https://github.com/gyscos/cursive/pull/503))
pub mod tui;
pub mod utils;
