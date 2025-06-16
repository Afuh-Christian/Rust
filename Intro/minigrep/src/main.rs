

use minigrep::Config;


fn main() {


 let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    // println!("Searching for {}", &config.query);
    // println!("In file {}", &config.file_path);

    Config::run(config)
        .unwrap_or_else(|err| {
            eprintln!("Application error: {err}");
            std::process::exit(1);
        });
}
