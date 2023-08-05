use std::path::Path;

use crate::compiler::errors;

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
