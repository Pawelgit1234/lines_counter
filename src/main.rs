mod parse_config;
mod count;

use std::env;
use std::process;

use parse_config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if config.path.is_file() {
        println!("{}", count::count_lines_in_file(&config.path).unwrap_or_else(|err| {
            println!("Problelm by reading a file: {err}");
            process::exit(1);
        }));
    } else {
        println!("{}", count::count_line_in_dir(&config.path, config.is_recursive).unwrap_or_else(|err| {
            println!("Problelm by reading dir: {err}");
            process::exit(1);
        }));
    }
}
