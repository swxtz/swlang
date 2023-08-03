
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

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

pub fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
