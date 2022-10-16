struct Processor {
    register_a: u8,                   //The first general purpose register
    register_b: u8,                   //The second general purpose register
    register_result: u8,              //The register the stores the result of most operations
    rip: u8,                          //The current instruction location
    stack: [u8; 256],                 //The memory of the program
    instructions: [Instruction; 256], //The program being executed
}

#[derive(Clone, Copy)]
enum Register {
    A,   //The first general purpose register
    B,   //The second general purpose register
    Res, //The result register
    Rip, //the current instruction location
}

#[derive(Clone, Copy)]
enum Instruction {
    Move(Register, u8),            //Set the given register to the given value
    Cpy(Register, Register),       //Copy the value of the second register to the first
    Sub(Register, Register), // Subtracts the first register from the second and stores the result in the result register
    Add(Register, Register), // Add the 2 registers and stroes the result in the result register
    LoadFromAddress(Register, u8), //Loads the value of the given stack adress into the given register
    LoadToAddress(u8, Register),   //Loads the value in the given register to the given address
    JumpLessThan(u8), //jumps to the given instruction location if register a is less than register b
    JumpGreaterThan(u8), //jumps to the given instruction location if register a is greater than register b
    JumpEqual(u8), //Jumps to the given instruction location if register a is equal to register b
    Jump(u8),      //Jumps to the given instruction location
    Nop,
}

enum ExecutionError {
    IntegerOverflow,
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
    fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => self.register_a,
            Register::B => self.register_b,
            Register::Res => self.register_result,
            Register::Rip => self.rip,
        }
    }
    fn set_register(&mut self, register: Register, val: u8) {
        let rf = match register {
            Register::A => &mut self.register_a,
            Register::B => &mut self.register_b,
            Register::Res => &mut self.register_result,
            Register::Rip => &mut self.rip,
        };
        *rf = val;
    }
    fn mov(&mut self, r: Register, v: u8) {
        self.set_register(r, v);
    }
    fn cpy(&mut self, r1: Register, r2: Register) {
        self.set_register(r1, self.get_register(r2));
    }
    fn sub(&mut self, r1: Register, r2: Register) -> Result<(), ExecutionError> {
        let val1 = self.get_register(r1);
        let val2 = self.get_register(r2);
        if let Some(x) = val1.checked_sub(val2) {
            self.register_result = x;
            Ok(())
        } else {
            Err(ExecutionError::IntegerOverflow)
        }
    }
    fn add(&mut self, r1: Register, r2: Register) -> Result<(), ExecutionError> {
        let val1 = self.get_register(r1);
        let val2 = self.get_register(r2);
        if let Some(x) = val1.checked_add(val2) {
            self.register_result = x;
            Ok(())
        } else {
            Err(ExecutionError::IntegerOverflow)
        }
    }
    fn load_from_addr(&mut self, r: Register, addr: u8) {
        self.set_register(r, self.stack[addr as usize])
    }
    fn load_to_addr(&mut self, addr: u8, r: Register) {
        self.stack[addr as usize] = self.get_register(r);
    }
    fn jump_less_than(&mut self, addr: u8) {
        if self.register_a < self.register_b {
            self.rip = addr;
        }
    }
    fn jump_more_than(&mut self, addr: u8) {
        if self.register_a > self.register_b {
            self.rip = addr;
        }
    }
    fn jump_equal(&mut self, addr: u8) {
        if self.register_a == self.register_b {
            self.rip = addr;
        }
    }
    fn jump(&mut self, addr: u8) {
        self.rip = addr;
    }
    fn execute(&mut self) -> Result<(), ExecutionError> {
        while self.rip < 255 {
            let instruction = self.instructions[self.rip as usize];
            match instruction {
                Instruction::Nop => (),
                Instruction::Jump(addr) => self.jump(addr),
                Instruction::JumpEqual(addr) => self.jump_equal(addr),
                Instruction::JumpGreaterThan(addr) => self.jump_more_than(addr),
                Instruction::JumpLessThan(addr) => self.jump_less_than(addr),
                Instruction::LoadToAddress(addr, r) => self.load_to_addr(addr, r),
                Instruction::LoadFromAddress(r, addr) => self.load_from_addr(r, addr),
                Instruction::Add(r1, r2) => self.add(r1, r2)?,
                Instruction::Sub(r1, r2) => self.sub(r1, r2)?,
                Instruction::Move(r, v) => self.mov(r, v),
                Instruction::Cpy(r1, r2) => self.cpy(r1, r2),
            }
        }
        Ok(())
    }
}
