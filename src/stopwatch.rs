use std::time::Instant;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use crossterm::style::Color;
use crate::utils::{clear_terminal, display_big_text};

pub fn run(args: &clap::ArgMatches) {
    let color = args.get_one::<String>("color").map(|c| crate::utils::parse_color(c));
    start_stopwatch(color);
}

fn start_stopwatch(color: Option<Color>) {
    clear_terminal();
    let font = figlet_rs::FIGfont::standard().unwrap();
    let start_time = Instant::now();
    let running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler({
        let running = Arc::clone(&running);
        move || {
            running.store(false, Ordering::SeqCst);
        }
    }).expect("Error setting Ctrl-C handler");

    let mut stdout = std::io::stdout();

    while running.load(Ordering::SeqCst) {
        let elapsed = start_time.elapsed();
        let time_string = format!("{:02}:{:02}:{:02}.{:03}",
            elapsed.as_secs() / 3600,
            (elapsed.as_secs() % 3600) / 60,
            elapsed.as_secs() % 60,
            elapsed.subsec_millis());

        display_big_text(&mut stdout, &time_string, color, &font);
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
