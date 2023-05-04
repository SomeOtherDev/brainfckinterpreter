use std::path::PathBuf;

pub struct Memory {
    cells: [u8; 30_000],
    data_pointer: usize
}

impl Memory {
    pub fn new() -> Self {
        Memory { cells:[0u8; 30_000], data_pointer: 0 }
    }

    pub fn increment_pointer(&mut self) -> () {
        if (self.data_pointer + 1) < 30_000 {
            self.data_pointer += 1;
        }
    }

    pub fn decrement_pointer(&mut self) -> () {
        if (self.data_pointer - 1) > 0{
            self.data_pointer -= 1;
        }
    }

    pub fn increment_byte(&mut self) -> () {
        self.cells[self.data_pointer] += 1;
    }

    pub fn decrement_byte(&mut self) -> () {
        self.cells[self.data_pointer] -= 1;
    }

    pub fn get_byte(&self) -> u8 {
        self.cells[self.data_pointer]
    }

}


pub struct Program {
    instructions: Vec<char>,
    instruction_pointer: usize,
    memory: Memory
}

impl Program {
    pub fn from_string(source_code: String) -> Self {
        Program { instructions: source_code.chars().collect(), instruction_pointer: 0, memory: Memory::new() }
    }

    pub fn from_file(file_path: String) -> Result<Self, std::io::Error> {
        let source_code: String = std::fs::read_to_string(&file_path)?;
        Ok(Program {instructions: source_code.chars().collect(), instruction_pointer: 0, memory: Memory::new()})
    }

}