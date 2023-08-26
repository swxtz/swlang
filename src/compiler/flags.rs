use std::fs::File;
use std::io::Write;

pub fn new_local() -> String{
  let content = "# Comments must be done with `#`, visit http://... for documentation";
  let mut file =
      File::create("config.sw").expect("could not create a file, please try again later");

  file.write_all(content.as_bytes()).expect("Could not write");
  println!("Configuration file created at root (.)");

  return "Created with success".to_string();
}
