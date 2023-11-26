use std::fs;
use std::env;
use std::io;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
    match env::current_dir() {
        Ok(path) => {
            let _entries = ls(path).unwrap();
        }
        Err(e) => {
            println!("Unable to get path: {}", e);
        }
    }
}

fn ls(path: PathBuf) -> io::Result<()> {
    let mut entries = fs::read_dir(path)?
    .map(|res| res.map(|entry| entry.file_name()))
    .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(())
}
