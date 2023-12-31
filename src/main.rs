use colored::Colorize;
use std::env;

mod compiler;
mod template;
mod messages;

use crate::compiler::compiler::{verify_file_len, verify_filetype};
use crate::compiler::flags::{new_file, new_local};
use crate::messages::errors::{Error, ErrorType, Color};
use crate::template::downloader::{default_url, get_template};

fn main() {

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
            Error::new_error(1001, "You need to pass at least 1 argument".to_string(), ErrorType::Runtime, Color::Red),
            messages::help::print_help_message()
        );

        // print!("{}", Error::new_error(1, "teste".to_string(), ErrorType::Generic, Color::Red) );
        return;
    }

//     match args[1].as_str() {
//         "new" => {
//             if args[2].as_str() == "-l" {
//                 new_local();
//                 return;
//             }

//             new_file();

//             // println!("");
//             // print!("choose where the file will be saved, by default and in root (.): ");

//             // let mut path = String::new();

//             // io::stdin()
//             //     .read_line(&mut path)
//             //     .expect("Error reading file, try again!");

//             // println!("");
//             // print!("File being created at: {}", path)
//         }

//         "read" => {
//             if args[2].as_str() == "-l" {
//                 println!("{}", messages::warn(1));

//                 let path = Path::new("config.sw").to_str().unwrap();

//                 let filetype = verify_filetype(path);

//                 if filetype == false {
//                     return println!("{}", messages::error(2));
//                 }

//                 let lines = verify_file_len(path.to_string());

//                 for line in lines {
//                     println!("{}", line);
//                 }
//             }
//         }

//         "help" => {
//             let disclamer = "Project is in beta development phase.".yellow();

//             println!("{}", disclamer);
//             println!("{}", messages::help());
//             return;
//         }

//         "template" => {
//             #[allow(unused_assignments)]
//             let mut url = String::new();
//             if args.len() <= 2 {
//                 url = default_url();

//                 let content = get_template(url).expect("Could not download template");

//                 let path = Path::new("config.sw").to_str().unwrap();

//                 compiler::compiler::create_file(path.to_string(), content)
//                     .expect("Could not create file");

//                 return;
//             }

//             url = args[2].to_string();

//             let content = get_template(url).expect("Could not download template");

//             let path = Path::new("config.sw").to_str().unwrap();

//             compiler::compiler::create_file(path.to_string(), content)
//                 .expect("Could not create file");

//             return;
//         }
//         _ => {
//             let disclamer = "Project is in beta development phase.".yellow();

//             println!("{}", disclamer);
//             println!("{}", messages::help());
//             return;
//         }
//     }
}

mod tests {
    #[allow(unused_imports)]
    use crate::compiler::compiler::create_file_one_line;

    #[test]
    fn test_get_template() {
        let url = "https://raw.githubusercontent.com/swxtz/swlang/main/Cargo.toml".to_string();
        let content = super::get_template(url);
        assert_eq!(content.is_ok(), true);
    }

    #[test]
    fn test_get_template_and_convert_url() {
        let url = "https://github.com/swxtz/swlang/blob/main/Cargo.toml".to_string();
        let content = super::get_template(url);
        assert_eq!(content.is_ok(), true);
    }

    #[test]
    fn test_verify_filetype() {
        let path = "config.sw";
        let filetype = super::verify_filetype(path);
        assert_eq!(filetype, true);
    }

    #[test]
    fn test_verify_file_len() {
        let content: String =
            "# Comments must be done with `#`, visit http://... for documentation".to_string();

        let path = "config.sw";
        create_file_one_line(path.to_string(), content).expect("Error creating file");
        let lines = super::verify_file_len(path.to_string());
        assert_eq!(lines.len(), 1);
    }
}
