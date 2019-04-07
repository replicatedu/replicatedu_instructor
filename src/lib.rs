use std::fs;
use std::fs::OpenOptions;
use std::io::Write;


use std::process::{Command};

extern crate fs_extra;
extern crate skeleton_parser;
extern crate test_runner;
extern crate class_crypto;
use skeleton_parser::{return_default_delim, SkeletonCode, SkeletonDelimiters};
use class_crypto::ClassCrypto;
use class_crypto::convert_me_to_serializable;
use class_crypto::participant_to_str;
use class_crypto::serialization::Participant;

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
//Copy-Item C:\Logfiles -Destination C:\Drawings\Logs -Recurse
pub fn duplicate_directory(src: &str, dest: &str) {
    if cfg!(target_os = "windows") {
        panic!("need to support windows");
    }

    let owned_string: String = "cp -r ".to_owned();
    let command = owned_string + src + " " + dest;
    let mut c = command_wrapper(&command, ".");
    c.output();
}

fn write_file(filepath: &str, contents: &str) {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filepath)
    {
        Ok(ref mut file) => {
            file.set_len(0);
            writeln!(file, "{}",contents).unwrap();
        }
        Err(err) => {
            panic!("Failed to open log file: {}", err);
        }
    }
}

pub fn replace_with_skeleton(filepath: &str) {
    let contents = match fs::read_to_string(&filepath) {
        Ok(contents) => contents,
        Err(_) => return,
    };
    let delims = return_default_delim();
    let parsed_code = SkeletonCode::new(delims, contents).unwrap();
    write_file(filepath, &parsed_code.skeleton_code);
}

pub fn replace_with_solution(filepath: &str) {
    let contents = match fs::read_to_string(&filepath) {
        Ok(contents) => contents,
        Err(_) => return,
    };

    let delims = SkeletonDelimiters {
        skeleton_tag: "!_SKELETON".to_string(),
        skeleton_delimiter: "#//!_ ".to_string(),
        solution_tag: "!_SOLUTION".to_string(),
    };
    let parsed_code = SkeletonCode::new(delims, contents).unwrap();
    write_file(filepath, &parsed_code.solution_code);
}

pub fn pull_class_repo(repopath: &str, folder: &str) {
    let owned_string: String = "git clone ".to_owned();
    let mut command = owned_string + repopath;
    command += " template && rm -rf template/.git"; 
    let mut c = command_wrapper(&command, folder);
    c.output();
}

//rsa key generation
//ssh-keygen -f /etc/ssh/ssh_host_rsa_key -N '' -t rsa
pub fn gen_rsa_keys(outdir:&str, coord_crypto:&ClassCrypto, instructor_crypto:&ClassCrypto){
    let command = "rm -rf ./deploy_key* && \
                   ssh-keygen -f ./deploy_key -N '' -t rsa && \
                   echo \"paste the following into deploy keys\" && \
                   cat deploy_key.pub &&
                   ssh-add -y -K ./deploy_key";
    let mut c = command_wrapper(&command, outdir);
    println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    let command = "rm -rf ./database_key* && \
                   ssh-keygen -f ./database_key -N '' -t rsa && \
                   echo \"paste the following into deploy keys\" && \
                   cat database_key.pub &&
                   ssh-add -y -K ./database_key";
    let mut c = command_wrapper(&command, outdir);
    println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    
    //read the contents of the key and 
    let deploy_key = match fs::read_to_string(outdir.to_string()+&"/deploy_key.pub".to_owned()) {
        Ok(contents) => contents,
        Err(_) => panic!("cannot read the deploy public key"),
    };

    let deploy_key_toml = coord_crypto.encrypt_to_toml( deploy_key.as_bytes().to_vec(), 
                                                        instructor_crypto.return_pk());
    write_file(&(outdir.to_string()+&"/deploy_key.toml".to_owned()),
                 &deploy_key_toml);


    //read the contents of the key and 
    let database_key = match fs::read_to_string(outdir.to_string()+&"/database_key.pub".to_owned()) {
        Ok(contents) => contents,
        Err(_) => panic!("cannot read the database public key"),
    };

    let database_key_toml = coord_crypto.encrypt_to_toml( database_key.as_bytes().to_vec(), 
                                                        instructor_crypto.return_pk());
    write_file(&(outdir.to_string()+&"/database_keys.toml".to_owned()),
                 &database_key_toml);

    let coord_toml = participant_to_str( convert_me_to_serializable(coord_crypto));
    let instructor_toml = participant_to_str( convert_me_to_serializable(instructor_crypto));
    write_file(&(outdir.to_string()+&"/coord_keys.toml".to_owned()), &coord_toml);
    write_file(&(outdir.to_string()+&"/instructor_keys.toml".to_owned()), &instructor_toml);
    

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
