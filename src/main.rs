use replicatedu_lib::{
    duplicate_directory, pull_class_repo, replace_with_skeleton, replace_with_solution,
};
use test_runner::{run_test_file};
use std::env;
use std::fs::create_dir;
use std::panic;
use walkdir::{DirEntry, WalkDir};

use term_painter::Color::*;
use term_painter::ToStyle;

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
    let mut walker = WalkDir::new("test").into_iter();

    let cloned_dir = walker.nth(1).unwrap();
    for entry in walker.filter_entry(|e| !should_ignore(e)) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
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
    
}
