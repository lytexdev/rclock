use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use clap::{Arg, Command};
use crossterm::{
    ExecutableCommand, cursor, terminal,
    style::{Color, SetForegroundColor, ResetColor},
};
use figlet_rs::FIGfont;
use chrono::{Local, NaiveTime};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};


fn display_big_text<W: Write>(stdout: &mut W, text: &str, color: Option<Color>, font: &FIGfont) {
    let big_text = font.convert(text).unwrap();

    stdout.execute(cursor::MoveTo(0, 1)).unwrap();

    if let Some(color) = color {
        stdout.execute(SetForegroundColor(color)).unwrap();
    }

    writeln!(stdout, "{}", big_text).unwrap();

    if color.is_some() {
        stdout.execute(ResetColor).unwrap();
    }

    stdout.flush().unwrap();
}

fn parse_fixed_time(fixed_time: &str) -> Option<Duration> {
    let parts: Vec<&str> = fixed_time.split(':').collect();
    if parts.len() == 2 {
        if let (Ok(hours), Ok(minutes)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            let now = Local::now();
            let current_time = now.time();
            let target_time = NaiveTime::from_hms_opt(hours, minutes, 0).unwrap();

            let remaining = if target_time >= current_time {
                target_time.signed_duration_since(current_time).num_seconds() as u64
            } else {
                24 * 3600 - current_time.signed_duration_since(target_time).num_seconds() as u64
            };
            return Some(Duration::from_secs(remaining));
        }
    }
    None
}

fn start_alarm(duration: Duration, color: Option<Color>) {
    let font = FIGfont::standard().unwrap();
    println!("Alarm clock set for {} seconds.", duration.as_secs());

    let remaining_time = Arc::new(AtomicBool::new(true));
    let running = Arc::clone(&remaining_time);

    ctrlc::set_handler({
        let running = Arc::clone(&running);
        move || {
            running.store(false, Ordering::SeqCst);
        }
    }).expect("Error setting Ctrl-C handler");

    let start = Instant::now();
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();

    while running.load(Ordering::SeqCst) {
        let elapsed = start.elapsed();
        if elapsed >= duration {
            break;
        }

        let remaining = duration - elapsed;
        let time_string = format!("{:02}:{:02}:{:02}", 
            remaining.as_secs() / 3600, 
            (remaining.as_secs() % 3600) / 60, 
            remaining.as_secs() % 60);

        display_big_text(&mut stdout, &time_string, color, &font);
        thread::sleep(Duration::from_millis(500));
    }

    display_big_text(&mut stdout, "ALARM UP!", color, &font);
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    println!("ALARM! Time's up!");
}

fn main() {
    let matches = Command::new("Rclock")
        .version("0.1")
        .author("lytexdev")
        .about("command-line stopwatch & alarm clock")
        .subcommand(
            Command::new("stopwatch")
                .about("Starts the stopwatch")
                .arg(
                    Arg::new("color")
                        .short('c')
                        .long("color")
                        .help("Sets the color for the output")
                        .value_parser(["red", "green", "blue", "yellow", "cyan", "magenta", "white"]),
                ),
        )
        .subcommand(
            Command::new("alarm")
                .about("Sets an alarm")
                .arg(
                    Arg::new("time")
                        .help("Time in seconds for the alarm or fixed format HH:MM")
                        .required(true),
                )
                .arg(
                    Arg::new("color")
                        .short('c')
                        .long("color")
                        .help("Sets the color for the output")
                        .value_parser(["red", "green", "blue", "yellow", "cyan", "magenta", "white"]),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("stopwatch") {
        let color = matches.get_one::<String>("color").map(|c| match c.as_str() {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            "yellow" => Color::Yellow,
            "cyan" => Color::Cyan,
            "magenta" => Color::Magenta,
            "white" => Color::White,
            _ => Color::White,
        });

        let font = FIGfont::standard().unwrap();
        let mut stdout = io::stdout();
        stdout.execute(terminal::EnterAlternateScreen).unwrap();
        display_big_text(&mut stdout, "STOPWATCH", color, &font);
        stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("alarm") {
        let color = matches.get_one::<String>("color").map(|c| match c.as_str() {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            "yellow" => Color::Yellow,
            "cyan" => Color::Cyan,
            "magenta" => Color::Magenta,
            "white" => Color::White,
            _ => Color::White,
        });

        if let Some(time) = matches.get_one::<String>("time") {
            if let Ok(seconds) = time.parse::<u64>() {
                start_alarm(Duration::from_secs(seconds), color);
            } else if let Some(duration) = parse_fixed_time(time) {
                start_alarm(duration, color);
            } else {
                eprintln!("Invalid time format. Use seconds or HH:MM.");
            }
        }
    }
}
