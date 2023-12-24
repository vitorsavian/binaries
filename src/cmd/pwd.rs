use crate::domain::pwd;

pub fn run() {
    let _cmd = clap::Command::new("pwd")
        .about("Print the current working directory")
        .version("1.0.0")
        .get_matches();

    pwd::program()
}
