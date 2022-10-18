use crate::names::*;
#[derive(Debug)]
pub struct Processor {
    pub register_a: u8,                   //The first general purpose register
    pub register_b: u8,                   //The second general purpose register
    pub register_result: u8,              //The register the stores the result of most operations
    pub rip: u8,                          //The current instruction location
    pub stack: [u8; 256],                 //The memory of the program
    pub instructions: [Instruction; 256], //The program being executed
}

#[derive(Debug)]
pub enum ExecutionError {
    IntegerOverflow,
}

impl Processor {
    pub fn new(program: [Instruction; 256]) -> Self {
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
    fn load_from_addr(&mut self, r: Register, addr: Register) {
        self.set_register(r, self.stack[self.get_register(addr) as usize])
    }
    fn load_to_addr(&mut self, addr: Register, r: Register) {
        //println!("the address: {}", self.get_register(addr) as usize);
        //println!("the value: {}", self.get_register(r));
        self.stack[self.get_register(addr) as usize] = self.get_register(r);
        //println!("{}", self.stack[self.get_register(addr) as usize]);
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
    fn dump(&self, r: Register) {
        println!("{}", self.get_register(r));
    }
    fn dump_mem(&self, r: Register) {
        println!("{}", self.stack[self.get_register(r) as usize]);
    }
    pub fn execute(&mut self) -> Result<(), ExecutionError> {
        while self.rip < 255 {
            let instruction = self.instructions[self.rip as usize];
            self.rip += 1;
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
                Instruction::Dump(r) => self.dump(r),
                Instruction::DumpMem(r) => self.dump_mem(r),
            }
        }
        Ok(())
    }
}
