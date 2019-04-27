
use class_crypto;
use class_crypto::serialization::{Message, Participant};
use class_crypto::ClassCrypto;
use issue_database::ClassIssues;
use std::process::Command;
use std::env;
use std::fs;
use std::panic;
use std::str;
use std::{thread, time};
use term_painter::Color::*;
use term_painter::ToStyle;
use toml;
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

pub fn download_and_grade(repopath:String,) -> String{
    let owned_string: String = "mkdir -p /tmp/grading && rm -rf /tmp/grading && git clone ".to_owned();
    let mut command = owned_string + &repopath;
    command += " grading && mkdir output && docker run ";
    command += "--mount type=bind,source=\"$(pwd)\"/output,target=/tmp ";
    command += "--mount type=bind,source=\"$(pwd)\"/grading,target=/grading ";
    command += "replicatedu_tester:demo_time bash -c \"cd /grading && test_runner manifest.replicatedu /tmp/test_results\"";
    dbg!(&command);
    let mut c = command_wrapper(&command, "/tmp");
    //docker run --mount type=bind,source="$(pwd)"/output,target=/tmp --mount type=bind,source="$(pwd)"/grading,target=/grading replicatedu_tester:demo_time bash -c "cd /grading && test_runner manifest.replicatedu /tmp/test_results"
    //mkdir output && docker run \
    //  --mount type=bind,source="$(pwd)"/output,target=/tmp
    //  --mount type=bind,source="$(pwd)"/grading,target=/grading \
    //   replicatedu_tester:demo_time \
    //  bash -c "cd /home/ && test_runner test.toml /tmp/test_results"
    let result = &c.output().expect("failed");
    dbg!(&result);
    str::from_utf8(&result.stdout).unwrap().to_string()
}

pub fn main() {
    let username = &env::var("GITHUB_USERNAME").expect("set the GITHUB_USERNAME env");
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");

    let class_api = fs::read_to_string("api_addr".to_owned()).expect(
        "error reading the api_addr file, start daemon in an initialized repo \
         folder or move the api_addr to your location",
    );

    //loading it to send back to the student to confirm
    let instructor_cryto_file_string = fs::read_to_string("instructor_keys.toml".to_owned())
        .expect(
            "error reading the instructor_keys.toml file, start daemon in an initialized repo \
             folder or move the instructor_keys.toml to your location",
        );

    let instructor_cryto_obj: Participant =
        toml::from_str(&instructor_cryto_file_string).expect("error parsing coord crypto");

    //this is used to decrypt the students url
    let coord_cryto_file_string = fs::read_to_string("coord_keys.toml".to_owned()).expect(
        "error reading the coord_keys.toml file, start daemon in an initialized repo \
         folder or move the coord_keys.toml to your location",
    );

    let coord_cryto_obj: Participant =
        toml::from_str(&coord_cryto_file_string).expect("error parsing coord crypto");

    let coord_cryto = ClassCrypto::new_from_sk("coord", coord_cryto_obj.sk, true)
        .expect("error creating cryto obj");
    let instructor_cryto = ClassCrypto::new_from_sk("instructor", instructor_cryto_obj.sk, true)
        .expect("error creating cryto obj");

    let issue = ClassIssues::new(class_api, username.to_string(), password.to_string());

    let thirty_seconds = time::Duration::from_secs(30);
    println!("{}", Green.paint("entering grading loop"));
    loop {
        let did_panic = panic::catch_unwind(|| {
            let open_regs = issue.get_open_registrations().expect("error getting api");
            for reg in &open_regs {
                let reg_panic = panic::catch_unwind(|| {
                    //dbg!(reg);

                    //get the students reponse and attempt to decrypt using the coord crypto
                    let student_message: Message =
                        toml::from_str(&reg.body).expect("error reading toml");
                    let plain_message = coord_cryto
                        .decrypt_from_toml(&reg.body)
                        .expect("error decrypting");
                    println!(
                        "{}",
                        Green.paint("grading decrypted using coord keys added:")
                    );
                    println!(
                        "username: {} repo: {}",
                        reg.title,
                        str::from_utf8(&plain_message).expect("err parsing utf8")
                    );
                    let grade = download_and_grade(str::from_utf8(&plain_message).expect("err parsing utf8").to_string());
                    let enc_grade =
                        instructor_cryto.encrypt_to_toml(grade.as_bytes().to_vec(), student_message.pk);
                    
                    issue.post_grade(reg, &enc_grade);
                    println!("confirmed registration");
                });
                if reg_panic.is_err() {
                    println!(
                        "{}",
                        Red.paint("panic: invalid reg...trying next")
                    );
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn echo_hello() {
       dbg!(download_and_grade("https://github.com/replicatedu/test_class.git".to_string()));
    }
}
