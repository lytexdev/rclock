use std::time::{Duration, Instant};
use std::thread;
use crossterm::style::Color;
use crate::utils::{clear_terminal, display_big_text, play_beep};

pub fn run(args: &clap::ArgMatches) {
    let work = args
        .get_one::<u64>("work")
        .expect("Work duration is required");
    let break_time = args
        .get_one::<u64>("break")
        .expect("Break duration is required");
    let repeats = args
        .get_one::<u64>("repeats")
        .unwrap_or(&1);
    let color = args
        .get_one::<String>("color")
        .map(|c| crate::utils::parse_color(c));

    start_pomodoro(*work, *break_time, *repeats, color);
}

fn start_pomodoro(work_minutes: u64, break_minutes: u64, repeats: u64, color: Option<Color>) {
    clear_terminal();
    let font = figlet_rs::FIGfont::standard().unwrap();
    let work_duration = Duration::from_secs(work_minutes * 60);
    let break_duration = Duration::from_secs(break_minutes * 60);

    for cycle in 1..=repeats {
        println!("Pomodoro Cycle {}/{}", cycle, repeats);
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

        let break_start_time = Instant::now();
        while break_start_time.elapsed() < break_duration {
            let remaining = break_duration - break_start_time.elapsed();
            let time_string = format!("Break: {:02}:{:02}",
                remaining.as_secs() / 60,
                remaining.as_secs() % 60);

            display_big_text(&mut stdout, &time_string, color, &font);
            thread::sleep(Duration::from_millis(500));
        }

        play_beep();

        if cycle < repeats {
            display_big_text(&mut stdout, "NEXT CYCLE STARTING!", color, &font);
            thread::sleep(Duration::from_secs(3));
            clear_terminal();
        }
    }

    let mut stdout = std::io::stdout();
    display_big_text(&mut stdout, "POMODORO DONE!", color, &font);
    println!("Pomodoro session complete!");
}
