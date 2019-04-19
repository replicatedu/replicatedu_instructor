use replicatedu_lib::error_daemon;
use replicatedu_lib::register_daemon;
use replicatedu_lib::main_create;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "--create" || args[1] == "-c" {
        main_create();
    } else if args[1] == "--error_daemon" || args[1] == "-e" {
        error_daemon::main();
    } else if args[1] == "--register_daemon" || args[1] == "-r" {
        register_daemon::main();
    } else {
        panic!("specify a mode to run in with --create, --error_daemon or --register_daemon ");
    }
}
