
use std::path::Path;

pub fn verify_filetype(file: &str) -> bool {
    let path = Path::new(&file);

    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            if extension_str == "sw" {
                println!("Configuration file found");

                return true;
            } else {
                println!("Configuration file not found");

                return false;
            }
        } else {
            println!("Could not get the file extension.");

            return false;
        }
    } else {
        println!("The file does not have an extension.");

        return false;
    }
}

