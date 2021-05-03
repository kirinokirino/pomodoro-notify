const HELP: &str = "\
pomodoro-notify
USAGE:
  pomodoro-notify [POMODORO_DURATION [BREAK_DURATION [POMODOROS]]]
FLAGS:
  -h, --help            Prints help information
OPTIONS:
   POMODORO_DURATION: the number of minutes for the pomodoro
   BREAK_DURATION: the number of minutes for the break
   POMODOROS: the number of pomodoros to launch one after the other
";

#[derive(Debug, Clone, Copy)]
pub struct AppArgs {
    pub pomodoro_duration: u32,
    pub break_duration: u32,
    pub number_of_pomodoros: u32,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        pomodoro_duration: pargs.opt_free_from_str()?.unwrap_or(25),
        break_duration: pargs.opt_free_from_str()?.unwrap_or(5),
        number_of_pomodoros: pargs.opt_free_from_str()?.unwrap_or(4),
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}
