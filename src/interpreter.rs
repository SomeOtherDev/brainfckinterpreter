use std::path::PathBuf;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    IncDP,
    DecDP,
    IncByte,
    DecByte,
    OutByte,
    InpByte,
    LeftJMP,
    RightJMP,
    NOP
}

struct Lexer<'a> {
    stream : Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    fn new (source: &'a str) -> Lexer<'a> {
        Lexer {
            stream: source.chars().peekable()
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Instruction;
    
    fn next(&mut self) -> Option<Instruction> {
        let c = self.stream.next()?;

        match c {
            '>' => Some(Instruction::IncDP),
            '<' => Some(Instruction::DecDP),
            '+' => Some(Instruction::IncByte),
            '-' => Some(Instruction::DecByte),
            '.' => Some(Instruction::OutByte),
            ',' => Some(Instruction::InpByte),
            '[' => Some(Instruction::LeftJMP),
            ']' => Some(Instruction::RightJMP),

            other => Some(Instruction::NOP)

        }
    }
}

pub struct Memory {
    cells: [u8; 30_000],
    data_pointer: usize
}

impl Memory {
    pub fn new() -> Self {
        Memory { cells:[0u8; 30_000], data_pointer: 0 }
    }

    pub fn dp_move(&mut self, value: isize) {
        let move_result = self.data_pointer as isize + value;
        if move_result > 0 && move_result < self.cells.len() as isize {
            self.data_pointer = move_result as usize;
        }
    }

    pub fn increment_byte(&mut self) {
        self.cells[self.data_pointer] += 1;
    }

    pub fn decrement_byte(&mut self) {
        self.cells[self.data_pointer] -= 1;
    }

    pub fn get_byte(&self) -> u8 {
        self.cells[self.data_pointer]
    }

}


pub struct Program {
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    memory: Memory
}

impl Program {
    pub fn from_string(source_code: String) -> Self {
        let lexer = Lexer::new(source_code.as_str());
        let mut instructions: Vec<Instruction> = lexer.collect();
        instructions.retain(|&x| x != Instruction::NOP);

        Program { instructions: instructions, instruction_pointer: 0, memory: Memory::new() }
    }

    pub fn from_file(file_path: String) -> Result<Self, std::io::Error> {
        let source_code: String = std::fs::read_to_string(&file_path)?;
        let lexer = Lexer::new(source_code.as_str());
        let mut instructions: Vec<Instruction> = lexer.collect();
        instructions.retain(|&x| x != Instruction::NOP);

        Ok(Program {instructions: instructions, instruction_pointer: 0, memory: Memory::new()})
    }

}


