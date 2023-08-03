
use std::path::Path;

pub fn verify_filetype(file: &str) -> bool {
    let path = Path::new(&file);

    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            if extension_str == "sw" {
                println!("Arquivo de configuração encontrado");

                return true;
            } else {
                println!("Arquivo de configuração não encontrado");

                return false;
            }
        } else {
            println!("Não foi possível obter a extensão do arquivo.");

            return false;
        }
    } else {
        println!("O arquivo não possui uma extensão.");

        return false;
    }
}

