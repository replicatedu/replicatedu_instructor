pub mod commands;
pub mod grade_daemon;
pub mod register_daemon;

use commands::{
    create_solution, create_student, gen_rsa_keys, pull_class_repo, should_ignore, write_file,
};

use git_wrapper;
use std::env;
use std::fs::create_dir;
use std::panic;

use std::fs;

use term_painter::Color::*;
use term_painter::ToStyle;

use class_crypto::ClassCrypto;
use std::fs::File;
use std::io::prelude::*;
use walkdir::{DirEntry, WalkDir};

use gag::Gag;

use test_runner::run_test_file;

fn run_tests(test_files: Vec<String>) {
    for file in test_files {
        println!("running test file {}", file);
        let print_gag = Gag::stdout().unwrap();
        let err_gag = Gag::stderr().unwrap();
        let scores = run_test_file(file);
        drop(print_gag);
        drop(err_gag);
        println!("scores: {:?}", scores);
        println!("{}", Green.paint("\tdone"));
    }
}

pub fn main_create() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        panic!("args: --create [reposity_url] [output_folder] [student_repo_name] [solution_repo_name]");
    }

    //setup the variables needed for repo creation
    let class_repo = &args[2];
    let output = &args[3];
    let student_repo_name = &args[4];
    let solution_repo_name = &args[5];
    let username = &env::var("GITHUB_USERNAME").expect("set the GITHUB_USERNAME env");
    let password = &env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");

    println!("{}", Yellow.paint("creating output directory: "));
    let print_gag = Gag::stdout().unwrap();
    create_dir(output).unwrap();
    drop(print_gag);
    println!("{}", Green.paint("\tdone"));

    println!("{}", Yellow.paint("pulling class repository: "));
    let print_gag = Gag::stdout().unwrap();
    pull_class_repo(class_repo, output);
    drop(print_gag);
    println!("{}", Green.paint("\tdone"));
    println!(
        "{}",
        Yellow.paint("creating student and solution directories")
    );
    let print_gag = Gag::stdout().unwrap();
    let mut walker = WalkDir::new(output).into_iter();
    //gets the new directory since it should be the only one in output
    let cloned_dir = walker.nth(1).unwrap();
    for entry in walker.filter_entry(|e| !should_ignore(e)) {
        let _entry = entry.unwrap();
    }
    let cloned_dir = cloned_dir.unwrap();
    let cloned_dir_s = cloned_dir.path().display().to_string();
    let student_dir = cloned_dir.path().display().to_string() + "_student";
    let solution_dir = cloned_dir.path().display().to_string() + "_solution";

    //create the student and solution directories then return the test manifests that are a part of them
    let solution_tests = create_solution(&cloned_dir_s, &solution_dir);
    let student_tests = create_student(&cloned_dir_s, &student_dir);
    drop(print_gag);
    println!("{}", Green.paint("\tdone"));

    println!("{}", Yellow.paint("running tests"));

    run_tests(solution_tests);
    run_tests(student_tests);

    println!("{}", Green.paint("\tdone"));

    let instructor_pair = ClassCrypto::new("instructor", true);
    let class_cord_pair = ClassCrypto::new("coordination", true);

    println!("{}", Yellow.paint("generating student deployment RSA keys"));
    let print_gag = Gag::stdout().unwrap();

    gen_rsa_keys(output, &class_cord_pair, &instructor_pair);

    let path = "/tmp/";

    let deploy_key_path: String = "".to_owned() + &output.to_string() + "/deploy_key.toml";
    let coord_key_path: String = "".to_owned() + &output.to_string() + "/coord_keys.toml";
    let inst_key_path: String = "".to_owned() + &output.to_string() + "/instructor_keys.toml";
    let pub_naked_deploy: String = "".to_owned() + &output.to_string() + "/deploy_key.pub";
    let priv_naked_deploy: String = "".to_owned() + &output.to_string() + "/deploy_key";

    let instructor_dir_coord: String = solution_dir.to_string() + "/coord_keys.toml";
    let instructor_dir_instr: String = solution_dir.to_string() + "/instructor_keys.toml";
    let instructor_dir_pub_naked_deploy: String = solution_dir.to_string() + "/deploy_key.pub";
    let instructor_dir_priv_naked_deploy: String = solution_dir.to_string() + "/deploy_key";

    let student_dir_deploy: String = student_dir.to_string() + "/deploy_key.toml";

    fs::copy(deploy_key_path, &student_dir_deploy).expect("file copy failed");
    fs::copy(coord_key_path, &instructor_dir_coord).expect("file copy failed");
    fs::copy(inst_key_path, &instructor_dir_instr).expect("file copy failed");
    fs::copy(pub_naked_deploy, &instructor_dir_pub_naked_deploy).expect("file copy failed");
    fs::copy(priv_naked_deploy, &instructor_dir_priv_naked_deploy).expect("file copy failed");

    //adding API address
    let mut url_str = String::new();
    //https://api.github.com/repos/hortinstein/fall2019student
    url_str.push_str(&format!(
        "https://api.github.com/repos/{}/{}",
        username, student_repo_name
    ));
    let api_addr_file: String = student_dir.to_string() + "/api_addr";
    write_file(&api_addr_file, &url_str);

    let api_addr_file: String = solution_dir.to_string() + "/api_addr";
    write_file(&api_addr_file, &url_str);

    drop(print_gag);
    println!("{}", Green.paint("\tdone"));

    println!(
        "{}",
        Yellow.paint("creating student solution and class repos on github")
    );
   let print_gag = Gag::stdout().unwrap();

    //created the student and solution repo
    git_wrapper::create_repo_pub(username, &password, student_repo_name, path);
    git_wrapper::create_repo(username, &password, solution_repo_name, path);
    git_wrapper::init_repo(username, &password, student_repo_name, &student_dir);
    git_wrapper::init_repo(username, &password, solution_repo_name, &solution_dir);
    //git_wrapper::add_files(&student_dir);
    //git_wrapper::add_files(&solution_dir);
    drop(print_gag);
    println!("{}", Green.paint("\tdone"));
}
