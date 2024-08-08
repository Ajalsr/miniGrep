use std::env;
use std::process;
//use std::error::Error;

use minigrep::Config;


fn main() {
    //let args : Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });



    //let file_path = &args[2];

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

   if let Err(e) =  minigrep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
   };

    
}






