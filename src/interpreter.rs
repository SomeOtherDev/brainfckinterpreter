use std::iter::Peekable;
use std::str::Chars;
use std::io::{self, Read, Write};

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

            _ => Some(Instruction::NOP)

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
        if move_result >= 0 && move_result < self.cells.len() as isize {
            self.data_pointer = move_result as usize;
        }
    }

    pub fn increment_byte(&mut self) {
        if (self.cells[self.data_pointer] as i16 + 1) <= 255 {
            self.cells[self.data_pointer] += 1;
        }
        else{
            println!("Attempt to increment byte {} above u8 max. Check brainfck code.", self.data_pointer);
        }
        
    }

    pub fn decrement_byte(&mut self) {
        if (self.cells[self.data_pointer] as i8 - 1) >= 0{
            self.cells[self.data_pointer] -= 1;
        }
        else {
            println!("Attempt to decrement byte {} below 0. Check brainfck code.", self.data_pointer);
        }
    }

    pub fn get_byte(&self) -> u8 {
        self.cells[self.data_pointer]
    }

    pub fn set_byte(&mut self, byte: u8) {
        self.cells[self.data_pointer] = byte;
    }

    
    pub fn arbitrary_read(&self, dp: usize) -> u8 {
        self.cells[dp]
    }

}


pub struct Program {
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    memory: Memory
}

impl Program {
    pub fn from_string(source_code: String) -> Self {
        let lexer = Lexer::new(source_code.as_str())
        .filter(|&t| t != Instruction::NOP);
        let instructions = lexer.collect();

        Program { instructions: instructions, instruction_pointer: 0, memory: Memory::new() }
    }

    pub fn from_file(file_path: String) -> Result<Self, std::io::Error> {
        let source_code: String = std::fs::read_to_string(&file_path)?;
        let lexer = Lexer::new(source_code.as_str())
        .filter(|&t| t != Instruction::NOP);
        let instructions: Vec<Instruction> = lexer.collect();

        Ok(Program {instructions: instructions, instruction_pointer: 0, memory: Memory::new()})
    }

    pub fn read_memory(&self, dp: usize) -> u8 {
        self.memory.arbitrary_read(dp)
    }

    pub fn input_byte(&self) -> u8 {
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        buffer[0]
    }

    pub fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            let instruction = self.instructions[self.instruction_pointer];
            match instruction {
                Instruction::IncDP => self.memory.dp_move(1),
                Instruction::DecDP => self.memory.dp_move(-1),
                Instruction::IncByte => self.memory.increment_byte(),
                Instruction::DecByte => self.memory.decrement_byte(),
                Instruction::OutByte => {
                    print!("{}", self.memory.get_byte());
                    io::stdout().flush().unwrap();
                },
                Instruction::InpByte => self.memory.set_byte(self.input_byte()),
                Instruction::LeftJMP => {
                    if self.memory.get_byte() == 0u8 {
                        for i in self.instruction_pointer..self.instructions.len()-1 {
                            if self.instructions[i] == Instruction::RightJMP {
                                self.instruction_pointer = i;
                                break;
                            }
                        }
                    }
                },
                Instruction::RightJMP => {
                    if self.memory.get_byte() != 0u8 {
                        for i in (0..self.instruction_pointer).rev() {
                            if self.instructions[i] == Instruction::LeftJMP {
                                self.instruction_pointer = i;
                                break;
                            }
                        }
                    }
                },
                Instruction::NOP => (),
            }

            self.instruction_pointer += 1;
            println!("Instruction {:?} ran. IP: {}. DP: {}. Data: {}", instruction, self.instruction_pointer, self.memory.data_pointer, self.memory.get_byte());
        }
    }

}


