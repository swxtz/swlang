use colored::Colorize;

pub fn error(code: i8) -> String {
    let code: i8 = code;

    match code {
        1 => "Configuration file found".yellow().to_string(),

        2 => "Configuration file not found".red().to_string(),

        3 => "Could not get the file extension".red().to_string(),

        4 => "The file does not have an extension".red().to_string(),

        _ => unreachable!(),
    }
}
