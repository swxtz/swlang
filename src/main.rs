use std::env;
use std::fs::File;
use std::io;
use std::io::Write;

mod errors;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", errors::error(1));
        return;
    }

    match args[1].as_str() {
        "new" => {
            if args[2].as_str() == "-l" {
                let content = "# Comenterios deve ser feitos com `#`, visite http://... para ver a documentação";

                let mut file = File::create("config.sw")
                    .expect("não foi possivel criar um arquivo, tente novamente mais tarde");

                file.write_all(content.as_bytes()).expect("Could not write");

                println!("Arquivo de configuração criado na raiz (.)");
                return;
            }

            println!("");
            print!("escolha onde o arquivo vai ser salvo, por padrão e no (.): ");

            let mut path = String::new();

            io::stdin()
                .read_line(&mut path)
                .expect("error ao ler arquivo, tente novamente!");

            println!("");
            print!("arquivo sendo criado em: {}", path)
        }
        _ => unreachable!(),
    }
}