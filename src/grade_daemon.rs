use class_crypto;
use class_crypto::serialization::{Message, Participant};
use class_crypto::ClassCrypto;
use issue_database::ClassIssues;
use std::env;
use std::fs;
use std::panic;
use std::str;
use std::{thread, time};
use term_painter::Color::*;
use term_painter::ToStyle;
use toml;

pub fn main() {
    let username = &env::var("GITHUB_USERNAME").expect("set the GITHUB_USERNAME env");
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");

    let class_api = fs::read_to_string("api_addr".to_owned()).expect(
        "error reading the api_addr file, start daemon in an initialized repo \
         folder or move the api_addr to your location",
    );

    let instructor_cryto_file_string = fs::read_to_string("coord_keys.toml".to_owned()).expect(
        "error reading the instructor_keys.toml file, start daemon in an initialized repo \
         folder or move the instructor_keys.toml to your location",
    );

    let instructor_cryto_obj: Participant =
        toml::from_str(&instructor_cryto_file_string).expect("error parsing instructor crypto");
    let instructor_cryto = ClassCrypto::new_from_sk("instructor", instructor_cryto_obj.sk, true)
        .expect("error creating cryto obj");
    let issue = ClassIssues::new(class_api, username.to_string(), password.to_string());
    let thirty_seconds = time::Duration::from_secs(30);
    println!("{}", Green.paint("entering register checking loop"));
    loop {
        let did_panic = panic::catch_unwind(|| {
            let open_regs = issue.get_open_registrations().expect("error getting api");
            for reg in &open_regs {
                //dbg!(reg);

            }
        });
        if did_panic.is_err() {
            println!(
                "{}",
                Red.paint("panic: api url incorrect, make sure running this from student dir ")
            );
        }
        println!("{}", Green.paint("sleeping 30 seconds before next check "));

        thread::sleep(thirty_seconds);
    }
}
