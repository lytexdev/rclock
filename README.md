# Rclock

## Overview
A Rust-based command-line stopwatch and alarm clock.

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

## License
This project is licensed under MIT. See the [LICENSE](LICENSE) file for details.
