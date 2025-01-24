use std::io::Write;
use crossterm::{
    ExecutableCommand, cursor,
    style::{Color, SetForegroundColor, ResetColor},
    terminal::{Clear, ClearType},
};
use chrono::{Local, NaiveTime};
use std::time::Duration;

pub fn parse_color(color: &str) -> Color {
    match color {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "white" => Color::White,
        _ => Color::White,
    }
}

pub fn parse_fixed_time(fixed_time: &str) -> Option<Duration> {
    let parts: Vec<&str> = fixed_time.split(':').collect();
    if parts.len() == 2 {
        if let (Ok(hours), Ok(minutes)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            let now = Local::now().time();
            let target_time = NaiveTime::from_hms_opt(hours, minutes, 0).unwrap();

            let remaining = if target_time >= now {
                target_time.signed_duration_since(now).num_seconds() as u64
            } else {
                24 * 3600 - now.signed_duration_since(target_time).num_seconds() as u64
            };
            return Some(Duration::from_secs(remaining));
        }
    }
    None
}

pub fn play_beep() {
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "/min", "powershell", "[console]::beep(800,800)"])
            .status();
    } else {
        let _ = std::process::Command::new("sh").arg("-c").arg("echo -e '\\a'").status();
    }
}

pub fn display_big_text<W: Write>(stdout: &mut W, text: &str, color: Option<Color>, font: &figlet_rs::FIGfont) {
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

pub fn clear_terminal() {
    let mut stdout = std::io::stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();
}
