struct Processor {
    register_a: u8,                   //The first general purpose register
    register_b: u8,                   //The second general purpose register
    register_result: u8,              //The register the stores the result of most operations
    rip: u8,                          //The current instruction location
    stack: [u8; 256],                 //The memory of the program
    instructions: [Instruction; 256], //The program being executed
}

enum Register {
    A,   //The first general purpose register
    B,   //The second general purpose register
    Res, //The result register
    Rip, //the current instruction location
}

enum Instruction {
    Move(Register, u8),            //Set the given register to the given value
    Cpy(Register, Register),       //Copy the value of the second register to the first
    Sub(Register, Register), // Subtracts the first register from the second and stores the result in the result register
    Add(Register, Register), // Add the 2 registers and stroes the result in the result register
    LoadFromAddress(Register, u8), //Loads the value of the given stack adress into the given register
    LoadToAddress(u8, Register),   //Loads the value in the given register to the given address
    JumpLessThan(u8), //jumps to the given instruction location if register a is less than register b
    JumpGreaterThan(u8), //jumps to the given instruction location if register a is greater than register b
    JumpEqualTo(u8), //Jumps to the given instruction location if register a is equal to register b
    Nop,
}

enum ExecutionError {
    IntegerOverflow,
    OutOfBoundMem,
}

impl Processor {
    fn new(program: [Instruction; 256]) -> Self {
        Processor {
            register_a: 0,
            register_b: 0,
            register_result: 0,
            rip: 0,
            stack: [0; 256],
            instructions: program,
        }
    }
    fn execute() -> Result<(), ExecutionError> {
        Ok(())
    }
}
