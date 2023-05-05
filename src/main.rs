use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod interpreter;
use interpreter::Program;


fn main() {

    let mut cmdline = env::args().skip(1); //Skips executable path
    if let Some(file_path) = cmdline.next() {
        match fs::metadata(&file_path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    let mut program: Program = Program::from_file(file_path.to_string()).unwrap();
                    program.run();
                    println!();
                }
            },
            Err(_) => {
                println!("File {} does not exist.", file_path);
            }
        }
    }
    else {
        let mut line = String::new();
        let stdin = io::stdin();
        loop {
            print!("brainf*ck: ");
            io::stdout().flush().unwrap();

            stdin.lock().read_line(&mut line).unwrap();
            line = line.trim().to_string();
            let mut program: Program = Program::from_string(&line);
            program.run();
            println!();
        }
        
    }
}
