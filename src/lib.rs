#[derive(Debug)]
pub struct Processor {
    pub register_a: u8,                   //The first general purpose register
    pub register_b: u8,                   //The second general purpose register
    pub register_result: u8,              //The register the stores the result of most operations
    pub rip: u8,                          //The current instruction location
    pub stack: [u8; 256],                 //The memory of the program
    pub instructions: [Instruction; 256], //The program being executed
}

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

#[derive(Debug)]
pub enum ParserError<'a> {
    UnknownKeyWord(u8, u8, &'a str),
    InvalidArgument(u8, u8, &'a str),
    LengthError(usize),
    InstructionLengthError(u8, usize),
}

fn parse_register(line: u8, col: u8, s: &str) -> Result<Register, ParserError> {
    match s {
        "a" => Ok(Register::A),
        "b" => Ok(Register::B),
        "rip" => Ok(Register::Rip),
        "result" => Ok(Register::Res),
        _ => Err(ParserError::InvalidArgument(line, col, s)),
    }
}

fn parse_number(line: u8, col: u8, s: &str) -> Result<u8, ParserError> {
    match s.parse() {
        Err(_) => Err(ParserError::InvalidArgument(line, col, s)),
        Ok(x) => Ok(x),
    }
}

fn parse_move(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 3 {
        let register = parse_register(line, 2, s[1])?;
        let number: u8 = parse_number(line, 3, s[2])?;
        Ok(Instruction::Move(register, number))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}
fn parse_cpy(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 3 {
        let register1 = parse_register(line, 1, s[1])?;
        let register2 = parse_register(line, 2, s[2])?;
        Ok(Instruction::Cpy(register1, register2))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}
fn parse_sub(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 3 {
        let register1 = parse_register(line, 1, s[1])?;
        let register2 = parse_register(line, 2, s[2])?;
        Ok(Instruction::Sub(register1, register2))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}
fn parse_add(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 3 {
        let register1 = parse_register(line, 1, s[1])?;
        let register2 = parse_register(line, 2, s[2])?;
        Ok(Instruction::Add(register1, register2))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}
fn parse_load_from_address(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 3 {
        let register1 = parse_register(line, 1, s[1])?;
        let address = parse_register(line, 2, s[2])?;
        Ok(Instruction::LoadFromAddress(register1, address))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}
fn parse_load_to_address(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 3 {
        let register1 = parse_register(line, 1, s[1])?;
        let address = parse_register(line, 2, s[2])?;
        Ok(Instruction::LoadToAddress(address, register1))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}
fn parse_jump_less_than(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 2 {
        let address = parse_number(line, 1, s[1])?;
        Ok(Instruction::JumpLessThan(address))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}
fn parse_jump_greater_than(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 2 {
        let address = parse_number(line, 1, s[1])?;
        Ok(Instruction::JumpGreaterThan(address))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}

fn parse_jump_equal_to(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 2 {
        let address = parse_number(line, 1, s[1])?;
        Ok(Instruction::JumpEqual(address))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}

fn parse_jump(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 2 {
        let address = parse_number(line, 1, s[1])?;
        Ok(Instruction::Jump(address))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}
fn parse_dump(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 2 {
        Ok(Instruction::Dump(parse_register(line, 1, s[1])?))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}

fn parse_dump_mem(line: u8, s: Vec<&str>) -> Result<Instruction, ParserError> {
    if s.len() == 2 {
        Ok(Instruction::DumpMem(parse_register(line, 1, s[1])?))
    } else {
        Err(ParserError::InstructionLengthError(3, s.len()))
    }
}

pub fn parse_program<'a>(string: &'a str) -> Result<[Instruction; 256], ParserError<'a>> {
    let mut program = [Instruction::Nop; 256];
    let lines: Vec<&str> = string.trim().split("\n").collect();
    if lines.len() > 256 {
        return Err(ParserError::LengthError(lines.len()));
    }
    for (i, line) in lines.iter().enumerate() {
        let line = line.trim();
        if line.starts_with("#") {
            continue; //this is a comment
        }
        let cols: Vec<&str> = line.split(" ").collect();
        if cols.len() == 0 {
            continue;
        }
        program[i] = match cols[0] {
            "nop" => Instruction::Nop,
            "mov" => parse_move(i as u8, cols)?,
            "cpy" => parse_cpy(i as u8, cols)?,
            "sub" => parse_sub(i as u8, cols)?,
            "add" => parse_add(i as u8, cols)?,
            "lfa" => parse_load_from_address(i as u8, cols)?,
            "lta" => parse_load_to_address(i as u8, cols)?,
            "jlt" => parse_jump_less_than(i as u8, cols)?,
            "jgt" => parse_jump_greater_than(i as u8, cols)?,
            "jeq" => parse_jump_equal_to(i as u8, cols)?,
            "jmp" => parse_jump(i as u8, cols)?,
            "dump" => parse_dump(i as u8, cols)?,
            "dm" => parse_dump_mem(i as u8, cols)?,
            x => return Err(ParserError::UnknownKeyWord(i as u8, 0, x)),
        }
    }
    Ok(program)
}
