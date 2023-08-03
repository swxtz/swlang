use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use colored::Colorize;

mod compiler;
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
                let content = "# Comments must be done with `#`, visit http://... for documentation";

                let mut file = File::create("config.sw")
                    .expect("could not create a file, please try again later");

                file.write_all(content.as_bytes()).expect("Could not write");

                println!("Configuration file created at root (.)");

                return;
            }

            println!("");
            print!("choose where the file will be saved, by default and in root (.): ");

            let mut path = String::new();

            io::stdin()
                .read_line(&mut path)
                .expect("Error reading file, try again!");

            println!("");
            print!("File being created at: {}", path)
        }

        "read" => {
            if args[2].as_str() == "-l" {
                println!("{}", errors::warn(1));

                let path = Path::new("config.sw").to_str().unwrap();

                let filetype = compiler::verify_filetype(path);

                if filetype == false {
                    return println!("{}", errors::error(2));
                }

                
            }
        }

        "teste" => {
            let disclamer = "Project is in beta development phase.".yellow();

            println!("{}", disclamer);
            println!("{}", errors::help());
            return;
        }

        _ => unreachable!(),
    }
}
