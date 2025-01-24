use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use crossterm::style::Color;
use crate::utils::{clear_terminal, display_big_text, parse_fixed_time, play_beep};

pub fn run(args: &clap::ArgMatches) {
    let time = args.get_one::<String>("time").expect("Time is required");
    let color = args.get_one::<String>("color").map(|c| crate::utils::parse_color(c));

    if let Ok(seconds) = time.parse::<u64>() {
        start_alarm(Duration::from_secs(seconds), color);
    } else if let Some(duration) = parse_fixed_time(time) {
        start_alarm(duration, color);
    } else {
        eprintln!("Invalid time format. Use seconds or HH:MM.");
    }
}

fn start_alarm(duration: Duration, color: Option<Color>) {
    clear_terminal();
    let font = figlet_rs::FIGfont::standard().unwrap();
    let running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler({
        let running = Arc::clone(&running);
        move || {
            running.store(false, Ordering::SeqCst);
        }
    }).expect("Error setting Ctrl-C handler");

    let mut stdout = std::io::stdout();
    let start_time = std::time::Instant::now();

    while running.load(Ordering::SeqCst) {
        let elapsed = start_time.elapsed();
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
    play_beep();

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }
}
