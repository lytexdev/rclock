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

    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "/min", "powershell", "[console]::beep(800,800)"])
            .status();
    } else {
        let _ = std::process::Command::new("sh").arg("-c").arg("echo -e '\\a'").status();
    }

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    println!("ALARM! Time's up!");
}

fn start_stopwatch(color: Option<Color>) {
    let font = FIGfont::standard().unwrap();
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let start_time = Instant::now();
    let running = Arc::new(AtomicBool::new(true));
    let handler_running = Arc::clone(&running);

    ctrlc::set_handler(move || {
        handler_running.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        let elapsed = start_time.elapsed();
        let time_string = format!("{:02}:{:02}:{:02}.{:03}", 
            elapsed.as_secs() / 3600, 
            (elapsed.as_secs() % 3600) / 60, 
            elapsed.as_secs() % 60, 
            elapsed.subsec_millis());

        display_big_text(&mut stdout, &time_string, color, &font);
        thread::sleep(Duration::from_millis(100));
    }

    stdout.execute(cursor::Show).unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
}

fn start_pomodoro(work_minutes: u64, break_minutes: u64, color: Option<Color>) {
    let font = FIGfont::standard().unwrap();
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();

    let work_duration = Duration::from_secs(work_minutes * 60);
    let break_duration = Duration::from_secs(break_minutes * 60);

    println!("Starting Pomodoro timer: {} minutes work, {} minutes break.", work_minutes, break_minutes);

    let start_time = Instant::now();
    while start_time.elapsed() < work_duration {
        let remaining = work_duration - start_time.elapsed();
        let time_string = format!("Work: {:02}:{:02}", 
            remaining.as_secs() / 60, 
            remaining.as_secs() % 60);

        display_big_text(&mut stdout, &time_string, color, &font);
        thread::sleep(Duration::from_millis(500));
    }

    display_big_text(&mut stdout, "BREAK TIME!", color, &font);

    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "/min", "powershell", "[console]::beep(800,800)"])
            .status();
    } else {
        let _ = std::process::Command::new("sh").arg("-c").arg("echo -e '\\a'").status();
    }

    thread::sleep(Duration::from_secs(3)); // wait for 3 seconds before break countdown

    let break_start_time = Instant::now();
    while break_start_time.elapsed() < break_duration {
        let remaining = break_duration - break_start_time.elapsed();
        let time_string = format!("Break: {:02}:{:02}", 
            remaining.as_secs() / 60, 
            remaining.as_secs() % 60);

        display_big_text(&mut stdout, &time_string, color, &font);
        thread::sleep(Duration::from_millis(500));
    }

    display_big_text(&mut stdout, "POMODORO DONE!", color, &font);

    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Pomodoro session complete!");
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
        .subcommand(
            Command::new("pomodoro")
                .about("Starts a Pomodoro timer with work and break intervals")
                .arg(
                    Arg::new("work")
                        .short('w')
                        .long("work")
                        .help("Work duration in minutes")
                        .required(true)
                        .value_parser(clap::value_parser!(u64)),
                )
                .arg(
                    Arg::new("break")
                        .short('b')
                        .long("break")
                        .help("Break duration in minutes")
                        .required(true)
                        .value_parser(clap::value_parser!(u64)),
                )
                .arg(
                    Arg::new("color")
                        .short('c')
                        .long("color")
                        .help("Sets the color for the output")
                        .value_parser(["red", "green", "blue", "yellow", "cyan", "magenta", "white"]),
                ),
        )
        .arg_required_else_help(true)
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
        start_stopwatch(color);
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

    if let Some(matches) = matches.subcommand_matches("pomodoro") {
        let work = *matches.get_one::<u64>("work").unwrap();
        let break_time = *matches.get_one::<u64>("break").unwrap();
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

        start_pomodoro(work, break_time, color);
    }
}
