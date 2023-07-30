use colored::Colorize;

pub fn error(error_code: i8) -> String {
    let code = error_code;

    match code {
        1 => "Voce precisa passar pelo menos 1 argumento"
            .red()
            .to_string(),
        _ => unreachable!(),
    }
}
