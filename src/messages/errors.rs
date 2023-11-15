
use colored::Colorize;

pub enum ErrorType {
    Generic,
    File,
    Compiler,
    Runtime,
    Syntax,
}

pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

pub struct ErrorBuilder {
    code: i32,
    message: String,
}

pub struct Error;

impl Error {
    pub fn new_error(code: i32, message: String, error_type: ErrorType, color: Color) -> String {
        let error = Error::error_builder(code, message);

        return Error::new_error_message(error, color, error_type);
    }

    pub fn new_error_message(error: ErrorBuilder, color: Color, error_type: ErrorType) -> String {
        let message = format!("{}: {} {}", convert_enum_to_string(error_type), error.message,  error.code);

        return add_color(message, color);
    }

    fn error_builder(code: i32, message: String) -> ErrorBuilder {
        ErrorBuilder {
            code,
            message,
        }
    }
}

fn convert_enum_to_string(error: ErrorType) -> String {
    match error {
        ErrorType::Generic => "Generic".to_string(),
        ErrorType::File => "File".to_string(),
        ErrorType::Compiler => "Compiler".to_string(),
        ErrorType::Runtime => "Runtime".to_string(),
        ErrorType::Syntax => "Syntax".to_string(),
    }
}

fn add_color(error: String, color: Color) -> String {
    match color {
        Color::Red => error.red().to_string(),
        Color::Yellow => error.yellow().to_string(),
        Color::Green => error.green().to_string(),
        Color::Blue => error.blue().to_string(),
    }
}


