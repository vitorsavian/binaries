extern crate colorful;

use std::{env, os::unix::fs::PermissionsExt};
use colorful::{Color, Colorful};

pub struct Cli {
    pub dir: String,
    pub permissions: bool
}

pub fn new(dir: &str, perm: bool) -> Cli {
    Cli {
        dir: dir.to_string(),
        permissions: perm,
    }
}

pub fn program(program: &self::Cli) {
    let path = env::current_dir().unwrap().join(&program.dir);

    path.read_dir().unwrap()
        .for_each(|entry| {
            let dir = entry.unwrap();
            if program.permissions {
                let permissions = dir.metadata().unwrap().permissions();
                let permissions_string = match permissions.mode() & 0o777 {
                    0o755 => "rwxr-xr-x",
                    0o644 => "rw-r--r--",
                    0o600 => "rw-------",
                    0o666 => "rw-rw-rw-",
                    0o777 => "rwxrwxrwx",
                    0o000 => "---------",
                    0o111 => "--x--x--x",
                    0o222 => "-w--w--w-",
                    0o333 => "-wx-wx-wx",
                    0o444 => "r--r--r--",
                    0o555 => "r-xr-xr-x",
                    _ => "---------",
                };
                println!("{}\t{}", permissions_string.color(Color::Yellow).bold(), dir.file_name().to_str().unwrap());
                return;
            } else {
                println!("{}", dir.file_name().to_str().unwrap());
                return;
            }
        });
}