use binaries::cmd::ls::run;
fn main() {
    run();
    // let args = env::args().collect::<Vec<String>>();

    // args.iter().for_each(|arg| {
    //     println!("{}", arg.to_string());
    // });

    // let flag = match args[1].as_str() {
    //     _ => env::set_current_dir(args[1].as_str()),
    // };


    // if flag == OK() {
    //     dir.unwrap().read_dir().unwrap()
    //         .for_each(|entry| {
    //             let dir = entry.unwrap();
    //             let permissions = dir.metadata().unwrap().permissions();
    //             let permissions_string = match permissions.mode() & 0o777 {
    //                 0o755 => "rwxr-xr-x",
    //                 0o644 => "rw-r--r--",
    //                 0o600 => "rw-------",
    //                 0o666 => "rw-rw-rw-",
    //                 0o777 => "rwxrwxrwx",
    //                 0o000 => "---------",
    //                 0o111 => "--x--x--x",
    //                 0o222 => "-w--w--w-",
    //                 0o333 => "-wx-wx-wx",
    //                 0o444 => "r--r--r--",
    //                 0o555 => "r-xr-xr-x",
    //                 _ => "---------",
    //             };
    //             println!("{}\t{}", permissions_string, dir.file_name().to_str().unwrap());
    //         });
    // } else {
    //     println!("Error");
    // }
}