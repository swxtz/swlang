use colored::Colorize;

pub fn error(error_code: i8) -> String {
    let code = error_code;

    match code {
        1 => "Voce precisa passar pelo menos 1 argumento"
            .red()
            .to_string(),

        2 => "O arquivo não tem a extensão .sw".red().to_string(),

        _ => unreachable!(),
    }
}

pub fn warn(warn_code: i8) -> String {
    let code = warn_code;

    match code {
        1 => "Lendo o arquivo no diretorio local (.)"
            .yellow()
            .to_string(),

        _ => unreachable!(),
    }
}
