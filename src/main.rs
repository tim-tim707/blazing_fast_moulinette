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

fn clone_all(student_list_filename: &String, practical_number: &String) {
    let student_list = lines_from_file(student_list_filename);

    println!("Cloning...");
    for student in student_list {
        let student_dir = format!("tp{}-{}", practical_number, student);
        let cloning_dir = format!("../TP{}/{}", practical_number, student_dir);
        create_dir_all(&cloning_dir).expect("Directory already exist");
        let repo = format!("git@git.cri.epita.fr:p/2025-s3-tp/{}", student_dir);

        Command::new("git")
            .arg("clone")
            .arg(repo)
            .arg(cloning_dir)
            .output()
            .expect("Couldnt clone");
    }

    println!("Cloning done");
}

fn commit_messages(student_list_filename: &String, practical_number: &String) {
    let commit_message_file = format!("../TP{}/commit_message.txt", practical_number);
    let commit_message_path = Path::new(&commit_message_file);

    if Path::exists(commit_message_path) {
        eprintln!("Warning: commit file already exists, no modification will occur");
    } else {
        let mut commit_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(commit_message_path)
            .expect("Couldnt create commit file");

        let student_list = lines_from_file(student_list_filename);
        for student in student_list {
            let student_dir = format!("tp{}-{}", practical_number, student);
            let cloning_dir = format!("../TP{}/{}", practical_number, student_dir);

            let command_result = Command::new("git")
                .current_dir(cloning_dir)
                .args(["log", "--pretty=format:%s"])
                .output()
                .expect("Couldnt get commit messages");

            let commit_messages = String::from_utf8_lossy(&command_result.stdout);

            //File::write_all(&mut commit_file, format!("\n\n{}\n", student).as_bytes()).expect("Couldnt write student name");
            File::write_all(&mut commit_file, "\n\n====\n".as_bytes())
                .expect("Couldnt write student separator");
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

    //clone_all(&args[2], &args[1])
    commit_messages(&args[1], &args[2]);
}
