use std::io::Read;

use commands::Command;

mod commands;

const DEBUG: bool = false;
const PRINT_MEMORY: bool = false;

// The machine that keeps track of the data
pub struct Machine {
    // The instruction pointer is which instruction of the code is currently being executed
    // Pretty much an index to the character that is being interpreted
    instruction_pointer: usize,
    // The data pointer is the pointer that points to the element of the array to manipulate
    data_pointer: usize,
    // This will keep the max length of the vector, so the maximum number the data pointer has been set to
    max_len: usize,
    // How many times the data pointer was moved into a negative value
    negative: usize,
    // This data is the memory that Brainfuck can edit, so this will help to print characters
    memory: Vec<u8>,
    // The code to be executed
    code: String,
}

impl Machine {
    // Creates a new machine with its starting values
    pub fn new() -> Self {
        Machine {
            instruction_pointer: 0,
            data_pointer: 0,
            max_len: 0,
            negative: 0,
            // As per Wikipedia, the array must have "at least" 30,000 element
            memory: vec![0; 30000],
            code: String::new(),
        }
    }

    // Try to find the next ending loop command, and also check for newly opened loops when encountering a begin loop command
    fn find_end_loop(&self) -> Option<usize> {
        let mut opened_loops = 1;
        for idx in (self.instruction_pointer + 1)..self.code.len() {
            match Command::from(self.code.chars().nth(idx).unwrap()) {
                Command::BeginLoop => opened_loops += 1,
                Command::EndLoop => opened_loops -= 1,
                _ => {}
            }

            if opened_loops == 0 {
                return Some(idx + 1);
            }
        }
        None
    }

    // Try to find the next begin loop command, and also check for newly opened loops when encountering an end loop command
    fn find_begin_loop(&self) -> Option<usize> {
        let mut opened_loops = 1;
        for idx in (0..(self.instruction_pointer)).rev() {
            match Command::from(self.code.chars().nth(idx).unwrap()) {
                Command::BeginLoop => opened_loops -= 1,
                Command::EndLoop => opened_loops += 1,
                _ => {}
            }

            if opened_loops == 0 {
                return Some(idx + 1);
            }
        }
        None
    }

    // This pretty prints the memory in a similar format as memory dumps
    // address | values in decimal | values in ascii representation, NUL is '.'
    fn pretty_print_memory(&self) {
        println!("");
        let remainder = (self.max_len) % 10;
        for (i, chunk) in self.memory[0..(10 - remainder) + self.max_len]
            .chunks(10)
            .map(|chunk| chunk.to_vec())
            .enumerate()
        {
            print!("{:05}  ", i * 10);

            let mut decimal_view = String::new();
            let mut ascii_view = String::new();
            for value in chunk {
                decimal_view.push_str(format!("{:03} ", value).as_str());
                ascii_view
                    .push_str(format!("{}", if value == 0 { '.' } else { value as char }).as_str());
            }
            println!("{:<40} {:<10}", decimal_view, ascii_view);
        }
    }

    // Will evaluate Brainfuck code character by character
    pub fn evaluate(&mut self, bf: String) {
        self.code = bf.chars().filter(|&c| "+-<>[].,".contains(c)).collect();
        if DEBUG {
            println!("Interpreting '{}'", self.code);
        }
        while self.instruction_pointer < self.code.len() {
            let command = self.code.chars().nth(self.instruction_pointer).unwrap();
            if DEBUG {
                println!(
                    "Command: {} // Memory Layout: {:?} // Data Pointer: {} // Instruction Pointer: {} // Negative: {}",
                    command,
                    &self.memory[0..20],
                    self.data_pointer,
                    self.instruction_pointer,
                    self.negative,
                );
            }

            // Ignore all commands other than increment and decrement pointer if the data pointer is already negative
            if !matches!(
                Command::from(command),
                Command::IncrementPointer | Command::DecrementPointer
            ) && (self.negative != 0)
            {
                self.instruction_pointer += 1;
                continue;
            }

            let mut jumped = false;
            match Command::from(command) {
                Command::IncrementPointer => {
                    if self.negative > 0 {
                        self.negative -= 1;
                    } else {
                        self.data_pointer += 1;
                        // Make sure the len of the memory will be at least the value of the data pointer
                        // The vector has 30,000 initial values, so won't happen all the time, ideally it never even happens..
                        if self.data_pointer <= self.memory.len() {
                            self.memory.push(0);
                        }
                        if self.max_len < self.data_pointer {
                            self.max_len = self.data_pointer + 1;
                        }
                    }
                }
                Command::DecrementPointer => {
                    if self.data_pointer == 0 {
                        self.negative += 1;
                    } else {
                        self.data_pointer -= 1;
                    }
                }
                Command::IncrementByte => {
                    self.memory[self.data_pointer] = if self.memory[self.data_pointer] < u8::MAX {
                        self.memory[self.data_pointer] + 1
                    } else {
                        0
                    };
                }
                Command::DecrementByte => {
                    self.memory[self.data_pointer] = if self.memory[self.data_pointer] > 0 {
                        self.memory[self.data_pointer] - 1
                    } else {
                        u8::MAX
                    };
                }
                Command::OutputByte => {
                    print!("{}", self.memory[self.data_pointer] as char);
                }
                Command::InputByte => {
                    let input = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok());

                    if let Some(input_byte) = input {
                        if input_byte == 10 {
                            self.memory[self.data_pointer] = 0;
                            continue;
                        }
                        self.memory[self.data_pointer] = input_byte;
                    } else {
                        self.instruction_pointer += 1;
                        continue;
                    }
                }
                Command::BeginLoop => {
                    if self.memory[self.data_pointer] == 0 {
                        if let Some(idx) = self.find_end_loop() {
                            self.instruction_pointer = idx;
                            jumped = true;
                        }
                    }
                }
                Command::EndLoop => {
                    if self.memory[self.data_pointer] != 0 {
                        if let Some(idx) = self.find_begin_loop() {
                            self.instruction_pointer = idx;
                            jumped = true;
                        }
                    }
                }
                _ => {}
            }

            if !jumped {
                self.instruction_pointer += 1;
            }
        }

        if PRINT_MEMORY {
            self.pretty_print_memory();
        }
    }
}
