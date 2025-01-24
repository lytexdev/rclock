use clap::{Arg, Command};

pub fn get_cli_matches() -> clap::ArgMatches {
    Command::new("Rclock")
        .version("0.1")
        .author("lytexdev")
        .about("Command-line stopwatch & alarm clock")
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
                    Arg::new("repeats")
                        .short('r')
                        .long("repeats")
                        .help("Number of Pomodoro cycles to repeat")
                        .value_parser(clap::value_parser!(u64))
                        .default_value("1"),
                )
                .arg(
                    Arg::new("color")
                        .short('c')
                        .long("color")
                        .help("Sets the color for the output")
                        .value_parser(["red", "green", "blue", "yellow", "cyan", "magenta", "white"]),
                )

        )
        .arg_required_else_help(true)
        .get_matches()
}
