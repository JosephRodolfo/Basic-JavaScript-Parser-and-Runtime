mod helper_funcs;

use std::collections::HashMap;

use helper_funcs::{read_file_line_by_line, string_array_to_vec};
mod interpreter_types;
mod traits;
use interpreter_types::{Interpreter::Interpreter};

mod types;
use types::Program::Program;

fn main() {
    let file_string = read_file_line_by_line("src/test/test.txt");
    let length = file_string.len();
    let file_vec = string_array_to_vec(file_string);

    let mut program = Program {
        end: length,
        ..Default::default()
    };

    let new_interpreter = Interpreter::default();

    program.loop_to_parse_program(file_vec);
    // println!("{:#?}", program);

    new_interpreter.loop_through_body_types(program);
}
