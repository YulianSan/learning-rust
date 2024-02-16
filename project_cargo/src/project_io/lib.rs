use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Lack of parameters");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn run(config: &Config) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(&config.file_path)?;
    println!("content: {}", content);

    Ok(content)
}
