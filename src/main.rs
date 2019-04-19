use replicatedu_lib::grade_daemon;
use replicatedu_lib::register_daemon;
use replicatedu_lib::main_create;
use std::env;
use std::thread;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "--create" || args[1] == "-c" {
        main_create();
    } else if args[1] == "--grade_daemon" || args[1] == "-e" {
        grade_daemon::main();
    } else if args[1] == "--register_daemon" || args[1] == "-r" {
        register_daemon::main();
    } else if args[1] == "--all_daemons" || args[1] == "-a" {
        thread::spawn(move || {
            register_daemon::main();
        });
        thread::spawn(move || {
            grade_daemon::main();
        });
    } else {
        panic!("specify a mode to run in with --create, --grade_daemon or --register_daemon ");
    }
}
