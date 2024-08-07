use std::io;
use std::io::Write;

pub fn input_dialog(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let name = input.trim();
            Some(name.to_string())
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}