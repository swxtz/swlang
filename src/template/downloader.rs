use std::error::Error;
use std::io::{self, BufRead};
use std::process;

use crate::template::errors;

#[tokio::main]
pub async fn get_template(url: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut url = url;

    if url.len() == 0 {
        url = default_url();
    }

    url = converte_url(url);

    let resp = reqwest::get(url).await?;

    if resp.status() != 200 {
        eprintln!(
            "{} {}",
            errors::status_code(resp.status()),
            errors::error(1)
        );
        process::exit(1);
    }

    let text = resp.text().await?;

    let content = converte_to_buffer(text);

    Ok(content)
}

fn converte_to_buffer(response: String) -> Vec<String> {
    #[warn(unused_assignments)]
    let file = response;
    let reader = io::BufReader::new(file.as_bytes());

    #[allow(unused_variables)]
    let mut lines = 0;
    let mut content: Vec<String> = Vec::new();

    for line in reader.lines() {
        // Desembrulha o Resultado de cada linha
        match line {
            Ok(line_content) => {
                // Faça algo com o conteúdo da linha aqui
                lines += 1;
                content.push(line_content);
            }
            Err(error) => {
                eprintln!("Erro ao ler a linha: {}", error);
            }
        }
    }

    return content;
}

pub fn default_url() -> String {
    return "https://raw.githubusercontent.com/swxtz/swlang/main/LICENSE".to_string();
}

fn converte_url(url: String) -> String {
    let mut url = url;

    if url.contains("github.com") {
        url = url.replace("github.com", "raw.githubusercontent.com");
    }

    if url.contains("blob") {
        url = url.replace("blob/", "");
    }

    return url;
}

mod tests {
    #[test]
    fn test_get_template() {
        let url = "https://raw.githubusercontent.com/swxtz/swlang/main/Cargo.toml".to_string();
        let content = super::get_template(url);
        assert_eq!(content.is_ok(), true);
    }

    #[test]
    fn test_converte_to_buffer() {
        let url = "https://raw.githubusercontent.com/swxtz/swlang/main/Cargo.toml".to_string();
        let content = super::get_template(url);
        let content = content.unwrap();
        let content = super::converte_to_buffer(content.join("\n"));
        assert_eq!(content.len() > 0, true);
    }

    #[test]
    fn test_converte_url() {
        let mut url = "https://github.com/swxtz/swlang/blob/main/src/main.rs".to_string();
        url = super::converte_url(url);

        assert_eq!(url.contains("raw.githubusercontent.com"), true);
        assert_eq!(url.contains("blob"), false);
    }
}
