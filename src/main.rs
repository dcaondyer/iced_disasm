use iced_disasm::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = iced_disasm::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    println!("This is a disassembler based on iced-x86.");
    println!("Typical command:");
    println!("  ./iced_disasm [16|32|64] file_path");
    println!();
}
