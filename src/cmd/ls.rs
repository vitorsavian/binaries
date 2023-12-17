use std::{env, os::unix::fs::PermissionsExt};

pub fn run() {
    let _cmd = clap::Command::new("ls")
        .about("List directory contents")
        .version("1.0.0")
        .get_matches();

    let path = env::current_dir().unwrap();

    path.read_dir().unwrap()
        .for_each(|entry| {
            let dir = entry.unwrap();
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
            println!("{}\t{}", permissions_string, dir.file_name().to_str().unwrap());
        });
}