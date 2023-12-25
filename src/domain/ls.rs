extern crate colorful;

use colorful::{Color, Colorful, core::color_string::CString};
use std::{env, os::unix::fs::PermissionsExt};

pub struct Cli {
    pub dir: String,
    pub permissions: bool,
}

impl Cli {
    pub fn new(dir: &str, perm: bool) -> Cli {
        Cli {
            dir: dir.to_string(),
            permissions: perm,
        }
    }

    pub fn program(&self) {
        let path = env::current_dir().unwrap().join(&self.dir);

        path.read_dir().unwrap().for_each(|entry| {
            let dir = entry.unwrap();
            if self.permissions {
                println!(
                    "{}\t{}",
                    self.permissions(&dir),
                    dir.file_name().to_str().unwrap()
                );
                return;
            } else {
                println!("{}", dir.file_name().to_str().unwrap());
                return;
            }
        });
    }

    fn permissions(&self, entry: &std::fs::DirEntry) -> CString {
        let permissions = entry.metadata().unwrap().permissions();
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

        let result = permissions_string.color(Color::Yellow).bold();
        result
    }
}
