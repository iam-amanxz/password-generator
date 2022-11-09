use inquire::{MultiSelect, Text};
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::process;

const CAPITAL_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SIMPLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";
const SPECIAL_CHARS: &str = "!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*!@#$%^&*";
const NUMBERS: &str = "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789";

const TEXT_INCLUDE_CAPITAL: &str = "include capital letters";
const TEXT_INCLUDE_SIMPLE: &str = "include simple letters";
const TEXT_INCLUDE_SPECIAL: &str = "include special characters";
const TEXT_INCLUDE_NUMBERS: &str = "include numbers";

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
            use_simple,
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

fn main() {
    let mut use_capital = false;
    let mut use_simple = false;
    let mut use_special = false;
    let mut use_number = false;

    let pw_length = Text::new("Enter the password length: (5-127), Default: 12").prompt();

    let pw_length: i8 = match pw_length {
        Ok(length) => {
            let mut length: i32 = length.parse().unwrap_or_else(|_| 12);
            if length < 5 {
                length = 5;
            }
            if length > 127 {
                length = 127;
            }
            length as i8
        }
        Err(_) => {
            println!("An error happened when asking for password length, try again later.");
            process::exit(1);
        }
    };

    let options = vec![
        TEXT_INCLUDE_CAPITAL,
        TEXT_INCLUDE_SIMPLE,
        TEXT_INCLUDE_SPECIAL,
        TEXT_INCLUDE_NUMBERS,
    ];

    let options = MultiSelect::new("Select one more options:", options).prompt();

    match options {
        Ok(options) => {
            if options.contains(&TEXT_INCLUDE_CAPITAL) {
                use_capital = true
            }
            if options.contains(&TEXT_INCLUDE_SIMPLE) {
                use_simple = true
            }
            if options.contains(&TEXT_INCLUDE_SPECIAL) {
                use_special = true
            }
            if options.contains(&TEXT_INCLUDE_NUMBERS) {
                use_number = true
            }
            if options.len() == 0 {
                use_simple = true;
            }
        }
        Err(_) => {
            println!("The options could not be processed");
            process::exit(1);
        }
    }

    let config = Config::new(use_capital, use_simple, use_special, use_number, pw_length);
    let password = generate(&config);
    println!("{}", "=".repeat(password.len() + 4));
    println!("| {} |", password);
    println!("{}", "=".repeat(password.len() + 4));
}
