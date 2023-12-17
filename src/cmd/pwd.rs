use std::env;

pub fn run() {
    let _cmd = clap::Command::new("pwd")
        .about("Print the current working directory")
        .version("0.1.0")
        .get_matches();

    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("{}", e),
    }
}
