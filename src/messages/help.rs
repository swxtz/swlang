pub fn print_help_message() -> String {
    let help = "
    swlang [command] [options]

     Commands:
         new [options] - creates a new file
             -l - creates a configuration file at the root (.)

         read [options] - reads a file
             -l - reads the configuration file at the root (.)

         --help | -h - show this message
    ";

    help.to_string()
}