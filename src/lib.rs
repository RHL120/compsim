struct Processor {
    register_a: u8,
    register_b: u8,
    register_result: u8,
    rip: u8,
    stack: [u8; 1024],
}

enum Instruction {}
