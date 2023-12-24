use crate::domain::ls;
use clap::{Arg, App};

pub fn run() {
    let _cmd = App::new("ls")
        .about("List directory contents")
        .version("1.0.0")
        .arg(Arg::new("dir")
            .short('d')
            .long("dir")
            .help("List directories and files from a path"))
        .get_matches();
    
    ls::program()
}
