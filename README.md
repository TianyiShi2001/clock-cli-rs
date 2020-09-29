# clock-cli-rs

Command line clock utilities, with TUI interfaces, implemented in Rust. Currently, these features are implemented:

- Stopwatch
  - start/pause/stop: ✅
  - lap time (similar to iOS's stopwatch's behaviour): ✅
  - report of all pause/start/lap instances (moments): WIP
- (Countdown) Timer
  - basics: ✅

# Installation

If you are a Rustacean 🦀️, just `cargo install clock-cli`.

Other installation methods: WIP

# Usage

## Stopwatch:

simply run:

```
clock
```

## Countdown Timer:

Specify the duration (in natual language) to run a countdown.

Examples:

```
clock 3 minutes
clock 4h3m
clock 1 day
```

to launch the stopwatch.

# Compatibility

Currently only works on Linux and MacOS.

# Acknowledgement

The TUI is based on the [**cursive**](https://github.com/gyscos/cursive) crate made by [Alexandre Bury (@glycos)](https://github.com/gyscos), who also helped me a lot during the development of this crate (see [glycose/cursive/#503](https://github.com/gyscos/cursive/pull/503))