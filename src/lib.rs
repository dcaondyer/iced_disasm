use colored::{ColoredString, Colorize};
use iced_x86::{
    Decoder, DecoderOptions, Formatter, FormatterOutput, FormatterTextKind, IntelFormatter,
};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;

const DEFAULT_CODE_RIP: u64 = 0x0000_7FFA_C46A_CDA4;

pub struct Config {
    pub code_bitness: u32,
    pub file_path: PathBuf,
    pub code_rip: u64,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            if args[1] != "help" {
                return Err("not enough arguments!");
            } else {
                menu();
                process::exit(0);
            }
        }

        let code_bitness = match args[1].clone().trim().parse() {
            Ok(num) => {
                if num != 64 && num != 32 && num != 16 {
                    return Err("the code bitness value must be one of 16, 32, or 64!");
                } else {
                    num
                }
            }
            Err(_) => {
                menu();
                process::exit(0);
            }
        };
        let file_path = PathBuf::from(args[2].clone());
        let code_rip;
        if args.len() > 3 {
            code_rip = match args[3].clone().trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    return Err("wrong code_rip!");
                }
            };
        } else {
            code_rip = DEFAULT_CODE_RIP;
        }

        Ok(Config {
            code_bitness,
            file_path,
            code_rip,
        })
    }
}

fn menu() {
    println!("This is a disassembler based on iced-x86.");
    println!("Typical command:");
    println!("  ./iced_disasm [16|32|64] file_path");
    println!("With code_rip:");
    println!("  ./iced_disasm [16|32|64] file_path code_rip");
    println!();
    println!("Meaning:");
    println!("  [16|32|64] is the arch of the executable");
    println!("  file_path is the path to the file");
    println!("  code_rip is the instruction pointer register");
    println!();
    println!("To show this message:");
    println!("  ./iced_disasm help");
    println!();
}

struct MyFormatterOutput {
    vec: Vec<(String, FormatterTextKind)>,
}

impl MyFormatterOutput {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
}

impl FormatterOutput for MyFormatterOutput {
    fn write(&mut self, text: &str, kind: FormatterTextKind) {
        self.vec.push((String::from(text), kind));
    }
}

#[allow(dead_code)]
pub fn disasm(code_bitness: u32, bytes: &[u8], code_rip: u64) {
    let mut decoder = Decoder::with_ip(code_bitness, bytes, code_rip, DecoderOptions::NONE);
    let mut formatter = IntelFormatter::new();
    formatter.options_mut().set_first_operand_char_index(8);
    let mut output = MyFormatterOutput::new();

    for instruction in &mut decoder {
        output.vec.clear();
        formatter.format(&instruction, &mut output);
        for (text, kind) in output.vec.iter() {
            print!("{}", get_color(text.as_str(), *kind));
        }
        println!();
    }
}

fn get_color(s: &str, kind: FormatterTextKind) -> ColoredString {
    match kind {
        FormatterTextKind::Directive | FormatterTextKind::Keyword => s.bright_yellow(),
        FormatterTextKind::Prefix | FormatterTextKind::Mnemonic => s.bright_red(),
        FormatterTextKind::Register => s.bright_blue(),
        FormatterTextKind::Number => s.bright_cyan(),
        _ => s.white(),
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let code = fs::read(&config.file_path)?;

    println!("Starting disassembler... for x{}", config.code_bitness);
    println!();
    disasm(config.code_bitness, code.leak(), config.code_rip);
    println!();
    println!("Priting finished!");

    Ok(())
}
