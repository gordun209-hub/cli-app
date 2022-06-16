use cli_app::Config;
use std::env;
use std::process;



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
