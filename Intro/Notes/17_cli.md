

# cli 


# Lib 

```rs 
// lib 
use std::{error::Error, fs};






// --snip--

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
   pub  fn new(args: &[String]) -> Result<Self , &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // if args.len() < 3{
        //     panic!("not enough arguments");
        // }

       Ok( Self { query: args[1].clone(), file_path: args[2].clone() })
        
    }

    pub fn run(config: &Config) -> Result<() , Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
        // .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
    Ok(())
}
}

```








# Main 

```rs 
// main 

use std::{error::Error, fs};

use minigrep::Config;


fn main() {


 let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    println!("Searching for {}", &config.query);
    println!("In file {}", &config.file_path);

    Config::run(&config)
        .unwrap_or_else(|err| {
            eprintln!("Application error: {err}");
            std::process::exit(1);
        });
}


```