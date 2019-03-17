use std::fs;
use std::process::{self, Command};

extern crate fs_extra;
extern crate skeleton_parser;
extern crate test_runner;
extern crate hubcaps;

use fs_extra::copy_items;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use skeleton_parser::{SkeletonCode, SkeletonDelimiters};
use test_runner::broker_test;
use hubcaps::{Credentials, Github};

//returns a command setup ready to run the tests
fn command_wrapper(test_command: &str, command_directory: &str) -> Command {
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

pub fn duplicate_directory(src: &str, dest: &str) {
    let options = CopyOptions::new(); //Initialize default values for CopyOptions

    // copy dir1 and file1.txt to target/dir1 and target/file1.txt
    let mut from_paths = Vec::new();
    from_paths.push(src);
    copy_items(&from_paths, dest, &options).unwrap();
}

pub struct GithubCommand {
    token: String
}

impl GithubCommand{
    pub fn set_github_token(&self, name:&str) {
        let github = Github::new(
            "myreplicatedu/0.0.1",
            Credentials::Token(self.token.clone()),
        );
        

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn echo_hello() {
        let mut c = command_wrapper("ls", "..");
        dbg!(c.output());
    }
}
