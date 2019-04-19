use replicatedu_lib::main_create;
use replicatedu_lib::daemon::main_daemon;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "--create" || args[1] == "-c" {
        main_create();
    }
    else if args[1] == "--daemon" || args[1] == "-d" {
        main_daemon();
    } else {
        panic!("specify a mode to run in with --create or --daemon");
    }

}
