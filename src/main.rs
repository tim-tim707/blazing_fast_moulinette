use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{prelude::*, BufReader},
    path::Path,
    process::Command,
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "BFM: The Blazing Fast Moulinette",
    about = "Tool for managing git and directory profiling of practicals, as well as running tests"
)]
struct Opt {
    /// Toggle practical cloning (-g, --git)
    #[structopt(short = "g", long = "git")]
    clone: bool,

    /// Toggle git commit message collection (-m, --messages)
    #[structopt(short = "m", long = "messages")]
    commit_messages: bool,

    /// Practical number, two digits
    #[structopt(name = "practical_nb", required = true)]
    practical_nb: String,

    /// File Path to student list
    #[structopt(name = "student_list_file", default_value = "")]
    student_list_file: String,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buffer = BufReader::new(file);
    buffer
        .lines()
        .map(|l| l.expect("Couldnt parse line"))
        .collect()
}

struct PracticalInfo {
    practical_path: String,
    student_list: Vec<StudentInfo>,
}
struct StudentInfo {
    login: String,
    practical_dir: String,
}

impl PracticalInfo {
    fn create_practical_data(options: &Opt, practical_number: &String) -> PracticalInfo {
        let mut student_list_path = "".to_string();

        if !Path::new(&options.student_list_file).exists() {
            if Path::new("students.txt").exists() {
                println!("Found students.txt file");
                student_list_path = "students.txt".to_string();
            } else if Path::new("../students.txt").exists() {
                println!("Found ../students.txt file");
                student_list_path = "../students.txt".to_string();
            } else {
                if options.student_list_file == "" {
                    panic!("Student list not found but required, please add this file to your directory or give its path as an argument");
                } else {
                    panic!("Invalid path: {}", options.student_list_file);
                }
            }
        } else {
        }

        let infos = PracticalInfo {
            practical_path: format!("../TP{}", practical_number),
            student_list: lines_from_file(student_list_path)
                .iter()
                .map(|login| StudentInfo {
                    login: login.to_string(),
                    practical_dir: format!("tp{}-{}", practical_number, login),
                })
                .collect(),
        };

        infos
    }
}

fn clone_all(infos: &PracticalInfo) {
    println!("Cloning...");
    for (i, student) in infos.student_list.iter().enumerate() {
        create_dir_all(&format!(
            "{}/{}",
            infos.practical_path, student.practical_dir
        ))
        .expect("Directory already exist");

        let command_result = Command::new("git")
            .current_dir(&format!("{}/", infos.practical_path))
            .args([
                "clone",
                &format!(
                    "git@git.cri.epita.fr:p/2025-s3-tp/{}",
                    student.practical_dir
                ),
            ])
            .output()
            .expect("Error while cloning");

        // status 128: DENIED by fallthru : never cloned
        if command_result.status.code() == Some(128) {
            let failed_clone_filename = &format!("{}/failed_clone.txt", infos.practical_path);
            let mut failed_clone_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(failed_clone_filename)
                .expect("Couldnt create failed clone file");
            File::write_all(
                &mut failed_clone_file,
                format!("{}\n", student.login).as_bytes(),
            )
            .expect("Couldnt write in file");
        }

        println!("{}/{}", i, infos.student_list.len());
    }

    println!("Cloning done");
}

fn commit_messages(infos: &PracticalInfo) {
    let commit_message = &format!("{}/commit_message.txt", infos.practical_path);
    let commit_message_path = Path::new(commit_message);

    if Path::exists(commit_message_path) {
        eprintln!("Warning: commit file already exists, no modification will occur");
    } else {
        let mut commit_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(commit_message_path)
            .expect("Couldnt create commit file");

        for student in &infos.student_list {
            let command_result = Command::new("git")
                .current_dir(&format!(
                    "{}/{}",
                    infos.practical_path, student.practical_dir
                ))
                .args(["log", "--pretty=format:%s"])
                .output()
                .expect("Couldnt get commit messages");

            let commit_messages = String::from_utf8_lossy(&command_result.stdout);

            File::write_all(
                &mut commit_file,
                format!("\n\n{}\n", student.login).as_bytes(),
            )
            .expect("Couldnt write student name");
            //File::write_all(&mut commit_file, "\n\n====\n".as_bytes()).expect("Couldnt write student separator");
            File::write_all(&mut commit_file, commit_messages.as_bytes())
                .expect("Couldnt write commit messages");
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let infos: PracticalInfo = PracticalInfo::create_practical_data(&opt, &opt.practical_nb);

    if opt.clone {
        clone_all(&infos);
    }
    if opt.commit_messages {
        commit_messages(&infos);
    }
}
