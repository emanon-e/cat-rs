use std::{env, fs::File, io::{BufRead, BufReader}, path::Path};

fn print_file_content(path: &str) {
    let file = File::open(path)
        .expect(format!("File '{}' doesn't exist", path).as_str());

    let file_reader = BufReader::new(file);

    for line in file_reader.lines() {
        let line_string = line
            .expect(format!("An error occured when reading {}", path).as_str());
        println!("{}", line_string);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        if !Path::new(arg).exists() {
            println!("File '{}' doesn't exist", arg);
        }
    }

    for arg in args.iter().skip(1) {
        print_file_content(arg);
    }
}
