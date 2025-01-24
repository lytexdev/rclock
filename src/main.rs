mod cli;
mod alarm;
mod stopwatch;
mod pomodoro;
mod utils;

fn main() {
    let matches = cli::get_cli_matches();

    match matches.subcommand() {
        Some(("stopwatch", args)) => stopwatch::run(args),
        Some(("alarm", args)) => alarm::run(args),
        Some(("pomodoro", args)) => pomodoro::run(args),
        _ => eprintln!("Invalid command. Use --help for usage."),
    }
}
