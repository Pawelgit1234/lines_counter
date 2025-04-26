pub struct Config {
    path: String,
    is_recursive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let path: String;
        let mut is_recursive = false;

        if args.len() > 3 || args.len() < 2 {
            return Err("Wrong count of arguments");
        }

        if args.contains(&"-r".to_string()) {
            is_recursive = true;
            path = args.iter().find(|&arg| arg != "-r").unwrap().clone();
        } else {
            path = args[1].clone();
        }

        Ok(Config { path, is_recursive })
    }
}