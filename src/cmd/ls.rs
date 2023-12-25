use crate::domain::ls::Cli;
use clap::{Arg, App};

pub fn run() {
    let cmd = App::new("ls")
        .about("List directory contents")
        .version("1.0.0")
        .arg(Arg::new("dir")
            .short('d')
            .long("dir")
            .takes_value(true)
            .help("List directories and files from a path"))
        .arg(Arg::new("permissions")
            .short('p')
            .long("permissions")
            .takes_value(false)
            .help("Show permissions"))
        .get_matches();

    let dir = cmd.value_of("dir").unwrap_or("");
    let perm = cmd.is_present("permissions");

    let cli = Cli::new(dir, perm);
    cli.program();
}
