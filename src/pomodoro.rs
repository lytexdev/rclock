use std::time::{Duration, Instant};
use std::thread;
use crossterm::style::Color;
use crate::utils::{clear_terminal, display_big_text, play_beep};

pub fn run(args: &clap::ArgMatches) {
    let work = *args.get_one::<u64>("work").unwrap();
    let break_time = *args.get_one::<u64>("break").unwrap();
    let color = args.get_one::<String>("color").map(|c| crate::utils::parse_color(c));

    start_pomodoro(work, break_time, color);
}

fn start_pomodoro(work_minutes: u64, break_minutes: u64, color: Option<Color>) {
    clear_terminal();
    let font = figlet_rs::FIGfont::standard().unwrap();
    let work_duration = Duration::from_secs(work_minutes * 60);
    let break_duration = Duration::from_secs(break_minutes * 60);

    let mut stdout = std::io::stdout();
    let start_time = Instant::now();

    while start_time.elapsed() < work_duration {
        let remaining = work_duration - start_time.elapsed();
        let time_string = format!("Work: {:02}:{:02}",
            remaining.as_secs() / 60,
            remaining.as_secs() % 60);

        display_big_text(&mut stdout, &time_string, color, &font);
        thread::sleep(Duration::from_millis(500));
    }

    play_beep();
    display_big_text(&mut stdout, "BREAK TIME!", color, &font);

    thread::sleep(break_duration);
    display_big_text(&mut stdout, "POMODORO DONE!", color, &font);
}
