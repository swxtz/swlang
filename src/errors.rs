use colored::Colorize;

pub fn error(error_code: i8) -> String {
    let code = error_code;

    match code {
        1 => "You need to pass at least 1 argument"
            .red()
            .to_string(),

        2 => "The file does not have the .sw extension".red().to_string(),

        _ => unreachable!(),
    }
}

pub fn warn(warn_code: i8) -> String {
    let code = warn_code;

    match code {
        1 => "Reading the file in the local directory (.)"
            .yellow()
            .to_string(),

        2 => "Project is in beta development phase."
            .yellow()
            .to_string(),

        _ => unreachable!(),
    }
}


pub fn help() -> String {
    let help = "
    swlang [command] [options]

     Commands:
         new [options] - creates a new file
             -l - creates a configuration file at the root (.)

         read [options] - reads a file
             -l - reads the configuration file at the root (.)

         --help | -h - show this message
    ";

    help.to_string()
}