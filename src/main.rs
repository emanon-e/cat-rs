use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use termion::color::{self, Color};

use rand::rng;
use rand::seq::SliceRandom;

const ALL_COLORS: [&'static dyn Color; 5] = [
    &color::Blue,
    &color::Red,
    &color::Magenta,
    &color::Green,
    &color::Yellow,
];

enum FileError {
    FileDoesntExit,
    FileReaderError,
}

fn print_file_content(path: &str, output_color: &'static dyn Color) -> Result<(), FileError> {
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
        println!("{}{}", color::Fg(output_color), line_string);
    }

    Ok(())
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("Expected at least one file as an argument");
        return;
    }

    let mut output_colors: Vec<&'static dyn Color> = vec![&color::White];
    if args[0] == "--colorize" {
        output_colors = ALL_COLORS.to_vec();
        output_colors.shuffle(&mut rng());
        args = (&args[1..]).to_vec();
    }

    if args.len() == 0 {
        println!("Expected at least one file as an argument");
        return;
    }

    for arg in args.iter() {
        if !Path::new(arg).exists() {
            println!("File {} doesn't exist", arg);
            return;
        }
    }

    for (index, path) in args.iter().enumerate() {
        let output_color = output_colors[index % output_colors.len()];
        match print_file_content(path, output_color) {
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
