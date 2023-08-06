use std::error::Error;
use std::io::{self, BufRead};

#[tokio::main]
pub async fn get_template(url: String) -> Result<Vec<String>, Box<dyn Error>> {
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;

    let content = converte_to_buffer(resp);

    Ok(content)
}

pub fn converte_to_buffer(response: String) -> Vec<String> {
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
