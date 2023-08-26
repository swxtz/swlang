use std::fs::File;
use std::io::Write;
use std::io::stdin;
use std::path;
use std::path::PathBuf;

pub fn new_local() -> String {
    let content = "# Comments must be done with `#`, visit http://... for documentation";
    let mut file =
        File::create("config.sw").expect("could not create a file, please try again later");

    file.write_all(content.as_bytes()).expect("Could not write");
    println!("Configuration file created at root (.)");

    return "Created with success".to_string();
}

mod tests_new_local {
    #[allow(unused_imports)]
    use crate::compiler::flags::new_local;

    #[test]
    fn test_new_local() {
        let content = new_local();
        assert_eq!(content, "Created with success");
    }
}

pub fn new_file() {
    let mut path_stdin = String::new();
    let mut name_stdin = String::new();

    println!("Please enter the path to create the file, by default in root (.):");
     stdin()
        .read_line(&mut path_stdin)
        .expect("Failed to read line");

    println!("Please enter the name of the file to be created, by default config:");
        stdin()
            .read_line(&mut name_stdin)
            .expect("Failed to read line");

    let path = converte_string_to_path(path_stdin, name_stdin);

    
}

fn converte_string_to_path(path: String, name: String) -> PathBuf {
    let path = path.trim();
    let pathh = PathBuf::from(path);

    let name = name.trim();

    if pathh.is_dir() {
        let path = pathh.join(name);
        println!("{}", pathh.display());
        return pathh;
    }

    return pathh;
}