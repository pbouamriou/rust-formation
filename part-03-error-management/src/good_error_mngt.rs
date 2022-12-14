use part_03_error_management::AppError;
use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, AppError> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(AppError::MissingQuery),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(AppError::MissingFilename),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    pub fn run(&self) -> Result<(), AppError> {
        let contents = fs::read_to_string(&self.filename)?;

        let results = if self.case_sensitive {
            search(&self.query, &contents)
        } else {
            search_case_insensitive(&self.query, &contents)
        };

        for line in results {
            println!("{}", line);
        }

        Ok(())
    }
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let results = contents.lines().filter(|x| x.contains(query)).collect();

    results
}

fn run() -> Result<(), AppError> {
    Config::new(env::args())?.run()
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            println!("Error : {}", err);
        }
    }
}
