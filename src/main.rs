use std::{
    env,
    fs::{create_dir_all, File, OpenOptions},
    io::{prelude::*, BufReader},
    path::Path,
    process::Command,
};

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
    fn create_practical_data(student_list: &String, practical_number: &String) -> PracticalInfo {
        PracticalInfo {
            practical_path: format!("../TP{}", practical_number),
            student_list: lines_from_file(student_list)
                .iter()
                .map(|login| StudentInfo {
                    login: login.to_string(),
                    practical_dir: format!("tp{}-{}", practical_number, login),
                })
                .collect(),
        }
    }
}

fn clone_all(infos: &PracticalInfo) {
    println!("Cloning...");
    for student in &infos.student_list {
        create_dir_all(&format!(
            "{}/{}",
            infos.practical_path, student.practical_dir
        ))
        .expect("Directory already exist");

        Command::new("git")
            .current_dir(&format!(
                "{}/{}",
                infos.practical_path, student.practical_dir
            ))
            .args([
                "clone",
                &format!(
                    "git@git.cri.epita.fr:p/2025-s3-tp/{}",
                    student.practical_dir
                ),
            ])
            .output()
            .expect("Couldnt clone");
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
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Please provide the student list and then the number of the practical");
    }

    let infos: PracticalInfo = PracticalInfo::create_practical_data(&args[1], &args[2]);
    clone_all(&infos);
    commit_messages(&infos);
}
