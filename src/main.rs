use cli_app::Config;
use std::env;
use std::process;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        assert_eq!(vec["safe, fast, productive."], search(query, contents));
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = cli_app::run(config) {
        // --snip--
        println!("Application error: {}", e);

        process::exit(1);
    }
}
