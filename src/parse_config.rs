use std::path::PathBuf;

pub struct Config {
    pub path: PathBuf,
    pub is_recursive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 3 || args.len() < 2 {
            return Err("Wrong count of arguments");
        }

        let mut path: Option<PathBuf> = None;
        let mut is_recursive = false;

        for arg in args.iter().skip(1) {
            if arg == "-r" {
                is_recursive = true;
            } else if path.is_none() {
                path = Some(PathBuf::from(arg));
            } else {
                return Err("Too many arguments");
            }
        }

        let path = path.ok_or("No path provided")?;
        if !path.exists() {
            return Err("Provided path does not exist");
        }

        if is_recursive && path.is_file() {
            return Err("Only Paths can be recursive")
        }

        Ok(Config { path, is_recursive })
    }
}