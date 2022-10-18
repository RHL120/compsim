pub mod names;
pub mod parser;
pub mod proc;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprint!("Usage: {} <file path>", args[0]);
        return;
    }
    let file = match std::fs::read_to_string(&args[1]) {
        Err(e) => {
            eprint!("Failed to open the file {}, error: {}", args[1], e);
            return;
        }
        Ok(x) => x,
    };
    let script = match parser::parse_program(&file) {
        Err(e) => {
            eprint!("Failed to parse the file {}, error: {:#?}", args[1], e);
            return;
        }
        Ok(x) => x,
    };
    let mut processor = proc::Processor::new(script);
    match processor.execute() {
        Err(e) => {
            println!("Failed to execute the script: error: {:#?}", e);
            return;
        }
        _ => (),
    };
    //println!("{:#?}", processor.stack);
}
