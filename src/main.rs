use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::{env, process};

const CAPITAL_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SIMPLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";
const SPECIAL_CHARS: &str = "!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*";
const NUMBERS: &str = "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789";

#[derive(Debug)]
struct Config {
    use_capital: bool,
    use_simple: bool,
    use_special: bool,
    use_number: bool,
    length: i8,
}

impl Config {
    pub fn new(
        use_capital: bool,
        use_simple: bool,
        use_special: bool,
        use_number: bool,
        length: i8,
    ) -> Self {
        Config {
            use_capital,
            use_simple: if !use_capital && !use_simple && !use_special && !use_number {
                true
            } else {
                use_simple
            },
            use_special,
            use_number,
            length,
        }
    }
}

fn generate(config: &Config) -> String {
    let mut password = String::new();
    let mut reg_pattern = String::new();

    loop {
        password.clear();
        reg_pattern.clear();

        if config.use_capital {
            password.push_str(CAPITAL_CHARS);
            reg_pattern.push_str("[A-Z]+")
        }
        if config.use_simple {
            password.push_str(SIMPLE_CHARS);
            reg_pattern.push_str("[a-z]+")
        }
        if config.use_special {
            password.push_str(SPECIAL_CHARS);
            reg_pattern.push_str("[!@#$%^&*]+")
        }
        if config.use_number {
            password.push_str(NUMBERS);
            reg_pattern.push_str("[0-9]+")
        }

        let re = Regex::new(&reg_pattern[..]).unwrap();

        unsafe {
            password.as_mut_vec().shuffle(&mut thread_rng());
        }

        password = password[0..(config.length as usize)].to_string();

        if re.is_match(&password) {
            break;
        }
    }

    password
}

fn print_suggest_help() {
    println!("See 'password_generator --help'.");
}

fn print_help() {
    println!("Usage: password_generator [--length=length] [--no-capital] [--no-simple] [--no-special] [--no-numbers]");
    println!("");
    println!("These are the common arguments supported by the password generator:");
    println!("--length        Length of the password (5-127), Eg: --length=32, Default: 12");
    println!("--no-capital    Generate password without capital letters");
    println!("--no-simple     Generate password without simple letters");
    println!("--no-special    Generate password without special characters");
    println!("--no-numbers    Generate password without numbers");
    println!("");
    println!("Note: If you provide all arguments, password generator will default to generate password with only simple letters");
}

fn main() {
    let args = env::args();
    let mut use_capital = true;
    let mut use_simple = true;
    let mut use_special = true;
    let mut use_number = true;
    let mut length: i8 = 12;

    for arg in args {
        if arg.eq("--help") {
            print_help();
            process::exit(0);
        }
        if arg.eq("--no-capital") {
            use_capital = false;
        }
        if arg.eq("--no-simple") {
            use_simple = false;
        }
        if arg.eq("--no-special") {
            use_special = false;
        }
        if arg.eq("--no-number") {
            use_number = false;
        }
        if arg.starts_with("--length") {
            let (_, len) = arg.split_once("=").unwrap_or_else(|| {
                eprintln!(
                    "Problem parsing arguments. Please try again with the correct arguments."
                );
                print_suggest_help();
                process::exit(1);
            });

            length = len.parse().unwrap_or_else(|error| {
                eprintln!("Problem parsing arguments: {error}");
                print_suggest_help();
                process::exit(1);
            });

            if length < 5 {
                println!("Length must be between 5-127 characters. Defaulting to 5");
                print_suggest_help();
                length = 5;
            }
        }
    }

    let config = Config::new(use_capital, use_simple, use_special, use_number, length);
    let password = generate(&config);
    println!("{}", password);
}
