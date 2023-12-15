use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display());
}