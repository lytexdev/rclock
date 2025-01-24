# Rclock

## Overview
A Rust-based command-line stopwatch, alarm clock, and Pomodoro timer.

## Installation

### Prerequisites
- Rust
- Cargo

**Clone the repository**
```bash
git clone https://github.com/lytexdev/rclock.git
cd rclock
```

**Build the application**
```bash
cargo build --release
```
The binary will be available in `./target/release/rclock`.

## Usage

### Stopwatch
To start the stopwatch:
```bash
rclock stopwatch
```

With color options:
```bash
rclock stopwatch --color red
```
Available colors: `red`, `green`, `blue`, `yellow`, `cyan`, `magenta`, `white`.

### Alarm
To set an alarm in seconds:
```bash
rclock alarm 60
```

To set an alarm at a specific time (HH:MM format):
```bash
rclock alarm 14:30
```

With color options:
```bash
rclock alarm 60 --color blue
```
The alarm will display a big "ALARM UP!" text and play a sound when triggered. Press `Ctrl+C` to exit.

### Pomodoro Timer
The Pomodoro timer allows you to manage work and break intervals.

To start a single Pomodoro cycle (e.g., 25 minutes work, 5 minutes break):
```bash
rclock pomodoro --work 25 --break 5
```

To repeat the Pomodoro timer multiple times (e.g., 3 cycles):
```bash
rclock pomodoro --work 25 --break 5 --repeats 3
```

With color options:
```bash
rclock pomodoro --work 25 --break 5 --repeats 3 --color green
```

The timer will:
- Alternate between "Work" and "Break" phases
- Display a countdown for each phase
- Play a sound at the end of each phase
- Automatically transition to the next cycle

Press `Ctrl+C` to exit at any time.

## License
This project is licensed under MIT. See the [LICENSE](LICENSE) file for details.
