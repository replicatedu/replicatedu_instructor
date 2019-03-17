use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::{self, Command};

extern crate fs_extra;
extern crate skeleton_parser;
extern crate test_runner;
extern crate hubcaps;

use fs_extra::copy_items;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use skeleton_parser::{SkeletonCode, SkeletonDelimiters, return_default_delim};
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

fn write_file(filepath:&str, contents: &str) -> std::io::Result<()> {
    let mut file = File::open(filepath)?;
    file.write_all(contents.as_bytes()).expect("Unable to write data");
    Ok(())
}

pub fn replace_with_skeleton(filepath:&str){
    let contents = match fs::read_to_string(&filepath) {
        Ok(contents) => contents,
        Err(_) => panic!("file does not exist"),
    };
    let delims = return_default_delim();
    let parsed_code = SkeletonCode::new(delims, contents).unwrap();
    write_file(filepath, &parsed_code.skeleton_code).unwrap();
}

pub fn replace_with_solution(filepath:&str){
    let contents = match fs::read_to_string(&filepath) {
        Ok(contents) => contents,
        Err(_) => panic!("file does not exist"),
    };
    let delims = return_default_delim();
    let parsed_code = SkeletonCode::new(delims, contents).unwrap();
    write_file(filepath, &parsed_code.solution_code).unwrap();
}

pub struct GithubCommand {
    token: String
}

impl GithubCommand{
    pub fn new(token: &str) -> GithubCommand {
        let gh = GithubCommand{
            token: token.to_string(),
        };
        gh
    }
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
