use colored::Colorize;
use reqwest::StatusCode;

pub fn error(code: i8) -> String {
    match code {
        1 => "Template not found".red().to_string(),

        _ => unreachable!(),
    }
}

pub fn status_code(msg: StatusCode) -> String {
    format!("Status: {},", msg).red().to_string()
}