use std::env;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub linelength: usize,
    pub tab_size: usize,
}

const DEFAULT_LENGTH: usize = 80;

// Last attempt is just make it a constructor, also add in error
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Ignore the file name

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let linelength = match args.next() {
            Some(arg) => {
                if let Ok(i) = arg.parse::<usize>() {
                    i
                } else {
                    return Err("That isn't a valid number!");
                }
            }
            None => DEFAULT_LENGTH,
        };

        let tab_size = env::var("TAB_SIZE")
            .unwrap_or(String::from("8"))
            .parse::<usize>()
            .expect("TAB_SIZE environment var is not a valid value");
        if tab_size == 0 {return Err("Tab size can not be 0")}

        Ok(Config {
            filename,
            linelength,
            tab_size,
        })
    }
}