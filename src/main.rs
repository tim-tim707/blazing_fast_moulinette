use std::{
    env,
    fs::{create_dir_all, File},
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
    for line in student_list {
        let student_dir = format!("tp{}-{}", practical_number, line);
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Please provide the student list and then the number of the practical");
    }

    clone_all(&args[2], &args[1])
}
