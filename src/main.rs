use colored::Colorize;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

mod compiler;
mod template;

use crate::compiler::compiler::{verify_file_len, verify_filetype};
use crate::template::downloader::{default_url, get_template};

mod errors;

fn main() {
    // let arg

    let args: Vec<String> = env::args().collect();

    let disclamer = "Project is in beta development phase.".yellow();

    if args.len() == 1 {
        println!(
            "
        {}
        {}
        {}
        ",
            disclamer,
            errors::error(1),
            errors::help()
        );
        return;
    }

    match args[1].as_str() {
        "new" => {
            if args[2].as_str() == "-l" {
                let content =
                    "# Comments must be done with `#`, visit http://... for documentation";

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

                let filetype = verify_filetype(path);

                if filetype == false {
                    return println!("{}", errors::error(2));
                }

                let lines = verify_file_len(path.to_string());

                for line in lines {
                    println!("{}", line);
                }
            }
        }

        "help" => {
            let disclamer = "Project is in beta development phase.".yellow();

            println!("{}", disclamer);
            println!("{}", errors::help());
            return;
        }

        "template" => {
            #[allow(unused_assignments)]
            let mut url = String::new();
            if args.len() <= 2 {
                url = default_url();

                let content = get_template(url).expect("Could not download template");

                let path = Path::new("config.sw").to_str().unwrap();

                compiler::compiler::create_file(path.to_string(), content)
                    .expect("Could not create file");

                return;
            }

            url = args[2].to_string();

            let content = get_template(url).expect("Could not download template");

            let path = Path::new("config.sw").to_str().unwrap();

            compiler::compiler::create_file(path.to_string(), content)
                .expect("Could not create file");

            return;
        }
        _ => {
            let disclamer = "Project is in beta development phase.".yellow();

            println!("{}", disclamer);
            println!("{}", errors::help());
            return;
        }
    }
}
