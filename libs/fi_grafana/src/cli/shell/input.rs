use std::io;
use std::io::Write;

use crate::error::GrafanaCliError;

pub enum UserInput {
    Number(u32),
    Text(String),
}

pub fn prompt_option<T>(prompt: &str, opt: &Option<T>) -> Option<T>
where
    T: Clone + From<UserInput>,
{
    if opt.is_none() {
        match user_input(prompt) {
            Ok(input) => {
                Some(T::from(input))
            }
            Err(error) => {
                eprintln!("{}", error);
                None
            }
        }
    } else {
        opt.clone()
    }
}

pub fn user_input(prompt: &str) -> Result<UserInput, GrafanaCliError> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = input.trim();
            if let Ok(number) = trimmed_input.parse::<u32>() {
                Ok(UserInput::Number(number))
            } else {
                Ok(UserInput::Text(trimmed_input.to_string()))
            }
        }
        Err(error) => Err(GrafanaCliError::IO(error))
    }
}

impl From<UserInput> for u32 {
    fn from(input: UserInput) -> Self {
        match input {
            UserInput::Number(n) => n,
            UserInput::Text(s) => {
                println!("{}", s);
                panic!("Expected a number, but got a string");
            }
        }
    }
}

impl From<UserInput> for String {
    fn from(input: UserInput) -> Self {
        match input {
            UserInput::Text(s) => s,
            UserInput::Number(_) => panic!("Expected a string, but got a number"),
        }
    }
}