#[derive(Clone, Copy, Debug)]
pub enum Register {
    A,   //The first general purpose register
    B,   //The second general purpose register
    Res, //The result register
    Rip, //the current instruction location
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Move(Register, u8),                  //Set the given register to the given value
    Cpy(Register, Register),             //Copy the value of the second register to the first
    Sub(Register, Register), // Subtracts the first register from the second and stores the result in the result register
    Add(Register, Register), // Add the 2 registers and stroes the result in the result register
    LoadFromAddress(Register, Register), //Loads the value of the stack adress stored into the second register into the given register
    LoadToAddress(Register, Register), //Loads the value of the seconds register into the stack adress stored in the first register
    JumpLessThan(u8), //Jumps to the given instruction location if register a is less than register b
    JumpGreaterThan(u8), //jumps to the given instruction location if register a is greater than register b
    JumpEqual(u8), //Jumps to the given instruction location if register a is equal to register b
    Jump(u8),      //Jumps to the given instruction location
    Dump(Register), //Shows the value of the given register
    DumpMem(Register), //Shows the value of the address stored in the register
    Nop,
}
