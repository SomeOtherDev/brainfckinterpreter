use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod interpreter;
use interpreter::Program;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path = &args[1];
        match fs::metadata(file_path) {
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
            let mut program: Program = Program::from_string(line.clone());
            program.run();
            println!();
        }
        
    }
}
