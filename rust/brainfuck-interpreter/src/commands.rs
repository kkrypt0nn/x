// The commands that brainfuck has
pub enum Command {
    // > 	Increment the data pointer by one (to point to the next cell to the right)
    IncrementPointer,
    // < 	Decrement the data pointer by one (to point to the next cell to the left)
    DecrementPointer,
    // + 	Increment the byte at the data pointer by one
    IncrementByte,
    // - 	Decrement the byte at the data pointer by one
    DecrementByte,
    // . 	Output the byte at the data pointer
    OutputByte,
    // , 	Accept one byte of input, storing its value in the byte at the data pointer
    InputByte,
    // [ 	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command,
    //      jump it forward to the command after the matching ] command
    BeginLoop,
    // ]    If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command,
    //      jump it back to the command after the matching [ command
    EndLoop,
    // Do nothing
    NoOp,
}

impl From<char> for Command {
    fn from(value: char) -> Self {
        match value {
            '>' => Command::IncrementPointer,
            '<' => Command::DecrementPointer,
            '+' => Command::IncrementByte,
            '-' => Command::DecrementByte,
            '.' => Command::OutputByte,
            ',' => Command::InputByte,
            '[' => Command::BeginLoop,
            ']' => Command::EndLoop,
            _ => Command::NoOp,
        }
    }
}
