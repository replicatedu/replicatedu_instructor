use issue_database::ClassIssues;
use std::env;
use std::fs;
pub fn main(){

    let username = &env::var("GITHUB_USERNAME").expect("set the GITHUB_USERNAME env");
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");

    let class_api = fs::read_to_string("api_addr".to_owned()).expect("error reading the api_addr file, start daemon in an initialized repo \
                folder or move the api_addr to your location");
    let issue = ClassIssues::new(class_api,
                                 username.to_string(),
                                 password.to_string());

    println!("entering register loop");
    loop {
        issue.get_all_registrations();
    }
}