mod lib;
fn main() {
    let program = lib::parse_program("mov a 2\nmov b 2\nadd a b").unwrap();
    let mut proc = lib::Processor::new(program);
    proc.execute().unwrap();
    println!("{:#?}", proc);
}
