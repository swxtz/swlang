

use crate::compiler::errors;

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, Write};
use std::error::Error;


pub fn verify_filetype(file: &str) -> bool {
    let path = Path::new(&file);

    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            if extension_str == "sw" {
                println!("{}", errors::error(1));

                return true;
            } else {
                println!("{}", errors::error(2));

                return false;
            }
        } else {
            println!("{}", errors::error(3));

            return false;
        }
    } else {
        println!("{}", errors::error(4));

        return false;
    }
}


pub fn verify_file_len(path:String) -> Vec<String> {
    let path = path.as_str();

    let file = File::open(path).expect("File not found");
    let reader = io::BufReader::new(file);

    #[allow(unused_variables)]
    let mut lines = 0; 
    let mut content:Vec<String> = Vec::new();

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

    if lines == 0 {
        println!("{}", errors::error(5));
    } 

    return content;
}

pub fn create_file(path: String, content: Vec<String>) -> Result<String, Box<dyn Error>> {

    let mut file = File::create(path)?;
    for line in content {
        writeln!(file, "{}", line)?;
    }

    Ok("File created".to_string())
}