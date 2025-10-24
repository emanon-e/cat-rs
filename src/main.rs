use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

enum FileError {
    FileDoesntExit,
    FileReaderError,
}

fn print_file_content(path: &str) -> Result<(), FileError> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err(FileError::FileDoesntExit),
    };

    let file_reader = BufReader::new(file);

    for line in file_reader.lines() {
        let line_string = match line {
            Ok(s) => s,
            Err(_) => return Err(FileError::FileReaderError),
        };
        println!("{}", line_string);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        if !Path::new(arg).exists() {
            println!("File {} doesn't exist", arg);
            return;
        }
    }

    for path in args.iter().skip(1) {
        match print_file_content(path) {
            Err(FileError::FileDoesntExit) => {
                println!("File {} doesn't exist", path);
                return;
            }
            Err(FileError::FileReaderError) => {
                println!("Error when reading {}", path);
                return;
            }
            _ => (),
        };
    }
}
