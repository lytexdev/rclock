use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use clap::{Arg, Command};
use crossterm::{ExecutableCommand, cursor, terminal, style::{Color, SetForegroundColor, ResetColor},};
use figlet_rs::FIGfont;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn display_time<W: Write>(stdout: &mut W, duration: Duration, color: Option<Color>, font: &FIGfont) {
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;
    let millis = duration.subsec_millis();

    let time_string = format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis);
    let big_text = font.convert(&time_string).unwrap();

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

fn start_stopwatch(color: Option<Color>) {
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let font = FIGfont::standard().unwrap();
    let start_time = Instant::now();
    let elapsed_time = Arc::new(AtomicBool::new(true));
    let running = Arc::clone(&elapsed_time);

    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while elapsed_time.load(Ordering::SeqCst) {
        let current_time = start_time.elapsed();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        display_time(&mut stdout, current_time, color, &font);
        thread::sleep(Duration::from_millis(100));
    }

    stdout.execute(cursor::Show).unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    display_time(&mut io::stdout(), start_time.elapsed(), color, &font);
}

fn main() {
    let matches = Command::new("Rclock")
        .version("0.1")
        .author("lytexdev")
        .about("Rust clock cli")
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .help("Sets the color of the stopwatch")
                .value_parser(["red", "green", "blue", "yellow", "cyan", "magenta", "white"]),
        )
        .arg(
            Arg::new("stopwatch")
                .required(true)
                .help("Starts the stopwatch"),
        )
        .get_matches();

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

    if matches.contains_id("stopwatch") {
        start_stopwatch(color);
    }
}
