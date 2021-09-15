# BFM: The Blazing Fast Moulinette
## ASM moulinette written in Rust(🚀)
Build with `cargo build`, run with `cargo run -- <flags> <args>`

# Flags:
- `-h` or `--help`: Print help message
- `-g` or `--git`: git clone all repository from a student list file (see arguments)
- `-m` or `--message`: Create a `commit_message.txt` in the practical directory listing all the commit messages of the students in a single file

# Arguments:
- Optional: `<path to student list file>`. If none is provided, the program will try to find a `students.txt` file in the current or parent directory. The file must have one login per line.
- Mandatory: `<practical number>`. Two digit number that will be used in order to make git commands and profile the practicals