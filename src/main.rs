use replicatedu_lib::{
    duplicate_directory, pull_class_repo, replace_with_skeleton, replace_with_solution,gen_rsa_keys
};

use test_runner::{run_test_file};
use std::env;
use std::fs::create_dir;
use std::panic;
use walkdir::{DirEntry, WalkDir};
use git_wrapper;

use std::fs;

use term_painter::Color::*;
use term_painter::ToStyle;

use class_crypto::ClassCrypto;
use std::fs::File;
use std::io::prelude::*;

fn should_ignore(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".git"))
        .unwrap_or(false)
}
fn create_student(cloned_dir: &str, student_dir: &str) -> Vec<String> {
    let mut tests = Vec::new();
    duplicate_directory(&cloned_dir, &student_dir);
    let walker = WalkDir::new(student_dir).into_iter();
    for entry in walker.filter_entry(|e| !should_ignore(e)) {
        let entry = entry.unwrap().path().display().to_string();
        println!("{}", entry);
        if entry != student_dir {
            //let s = format!("writing student: {}",entry);
            //println!("{}", Yellow.paint(s));
            replace_with_skeleton(&entry);
            //println!("{}", Green.paint("\tdone"));
        }
        if entry.contains("manifest.replicatedu") {
            let s = entry.to_string();
            tests.push(s)
        }
    }
    tests
}

fn create_solution(cloned_dir: &str, solution_dir: &str) -> Vec<String> {
    let mut tests = Vec::new();
    duplicate_directory(&cloned_dir, &solution_dir);
    let walker = WalkDir::new(solution_dir).into_iter();

    for entry in walker.filter_entry(|e| !should_ignore(e)) {
        let entry = entry.unwrap().path().display().to_string();
        println!("{}", entry);
        if entry != solution_dir {
            //let s = format!("writing solution: {}",entry);
            //println!("{}", Yellow.paint(s));
            replace_with_solution(&entry);
            //println!("{}", Green.paint("\tdone"));
        }
        if entry.contains("manifest.replicatedu") {
            let s = entry.to_string();
            tests.push(s)
        }
    }
    tests
}


fn run_tests(test_files:Vec<String>){
    for file in test_files{
        println!("{} {}", Yellow.paint("Running Test Files: "),Blue.paint(&file));
        let scores = run_test_file(file);
        dbg!(scores);
        println!("{}", Green.paint("\tdone"));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("args: reposity_url output_folder");
    }
    let output = &args[2];
    let class_repo = &args[1];

    println!("{}", Yellow.paint("creating output directory: "));
    create_dir(output).unwrap();
    println!("{}", Green.paint("\tdone"));

    println!("{}", Yellow.paint("pulling class repository: "));
    pull_class_repo(class_repo, output);
    println!("{}", Green.paint("\tdone"));

    println!(
        "{}",
        Yellow.paint("creating student and solution directories")
    );
    let mut walker = WalkDir::new(output).into_iter();

    let cloned_dir = walker.nth(1).unwrap();
    for entry in walker.filter_entry(|e| !should_ignore(e)) {
        let entry = entry.unwrap();
        //println!("{}", entry.path().display());
    }
    let cloned_dir = cloned_dir.unwrap();
    let cloned_dir_s = cloned_dir.path().display().to_string();
    let student_dir = cloned_dir.path().display().to_string() + "_student";
    let solution_dir = cloned_dir.path().display().to_string() + "_solution";
    
    //create the student and solution directories then return the test manifests that are a part of them
    let solution_tests = create_solution(&cloned_dir_s, &solution_dir);
    let student_tests = create_student(&cloned_dir_s, &student_dir);
    println!("{}", Green.paint("\tdone"));
    run_tests(solution_tests);
    run_tests(student_tests);
    
    let instructor_pair = ClassCrypto::new("instructor", true);
    let class_cord_pair = ClassCrypto::new("coordination", true);

    println!(
        "{}",
        Yellow.paint("generating student deployment RSA keys")
    );
    gen_rsa_keys(output,&class_cord_pair, &instructor_pair);

    let username = "hortinstein";
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
    let student_repo_name = "student_example";
    let solution_repo_name = "solution_example";
    let path = "/tmp/";
    
    let database_key_path:String = "".to_owned() + &output.to_string() +"/database_keys.toml";
    let database_key_priv_path:String = "".to_owned() + &output.to_string() +"/database_keys_priv.toml";
    
    let deploy_key_path:String ="".to_owned() + &output.to_string() +"/deploy_key.toml";
    let coord_key_path:String = "".to_owned() +&output.to_string() +"/coord_keys.toml";
    let inst_key_path:String = "".to_owned() +&output.to_string() +"/instructor_keys.toml";
   
    let student_dir_database:String = student_dir.to_string() + "/database_keys.toml";
    let student_dir_database_priv:String = student_dir.to_string() + "/database_keys_priv.toml";
    let student_dir_deploy:String = student_dir.to_string() + "/deploy_key.toml";
    let instructor_dir_coord:String = solution_dir.to_string() + "/coord_keys.toml"; 
    let instructor_dir_instr:String = solution_dir.to_string() + "/instructor_keys.toml"; 
    dbg!(&database_key_priv_path);
    fs::copy(database_key_priv_path, &student_dir_database_priv).expect("file copy failed");
    dbg!(&database_key_path);
    fs::copy(database_key_path, &student_dir_database).expect("file copy failed");
    dbg!(&deploy_key_path);
    fs::copy(deploy_key_path, &student_dir_deploy).expect("file copy failed");
    dbg!(&coord_key_path);
    fs::copy(coord_key_path, &instructor_dir_coord).expect("file copy failed");
    dbg!(&inst_key_path);
    fs::copy(inst_key_path, &instructor_dir_instr).expect("file copy failed");
   

    //created the student and solution repo
    git_wrapper::create_repo_pub(username, &password, student_repo_name, path);
    git_wrapper::create_repo(username, &password, solution_repo_name, path);
    git_wrapper::init_repo( username, &password, student_repo_name,&student_dir);
    git_wrapper::init_repo( username, &password, solution_repo_name,&solution_dir);

    git_wrapper::create_repo(username, &password, "class_database", path);
    let deploy_path:String = output.to_string() +"/database_key.pub";
    let mut file = File::open(deploy_path).expect("key not there");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("error reading key");
    git_wrapper::add_deploy_key(username, &password, "class_database", path, &contents);
 
}
