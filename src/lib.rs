use std::process::{self, Command};

extern crate  test_runner;
extern crate  skeleton_parser;

use skeleton_parser::{SkeletonCode,SkeletonDelimiters};
use test_runner::broker_test;

//returns a command setup ready to run the tests
fn setup_command(test_command: &str, command_directory: &str) -> Command {
    let mut command = if cfg!(target_os = "windows") {
        {
            let mut c = Command::new("cmd");
            c.args(&["/C", test_command]);
            c
        }
    } else {
        {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(test_command);
            c
        }
    };
    command.current_dir(command_directory);
    command
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn echo_hello() {
        let mut c = setup_command("ls", "..");
        dbg!(c.output());
    }
}
