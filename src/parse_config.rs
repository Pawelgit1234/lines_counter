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

        let path: PathBuf;
        let mut is_recursive = false;

        if args.contains(&"-r".to_string()) {
            is_recursive = true;
            path = args.iter()
                .find(|&arg| arg != "-r")
                .cloned()
                .map(PathBuf::from)
                .ok_or("No path provided after -r")?;
        } else {
            path = PathBuf::from(args[1].clone());
        }

        if !path.exists() {
            return Err("Provided path does not exist");
        }

        if is_recursive == true && path.is_file() {
            return Err("Only Paths can be recursive")
        }

        Ok(Config { path, is_recursive })
    }
}