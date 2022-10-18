use crate::names::*;
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
