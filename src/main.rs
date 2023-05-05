mod interpreter;

use interpreter::Program;

fn main() {
    let simple_addition_code = String::from("++>+++[<+>-]<");
    let mut simple_addition = Program::from_string(simple_addition_code);
    simple_addition.run();
    println!("{}", simple_addition.read_memory(0));
}
