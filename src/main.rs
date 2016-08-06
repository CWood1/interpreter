pub mod parser;
pub mod value;
pub mod binopadd;
pub mod binopsub;
pub mod ast_executor;

use ast_executor::AstExecutor;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: String = match args.len() {
        1 => panic!("No input file specified!"),
        2 => {
            match args[1].parse() {
                Ok(s) => s,
                Err(_) => panic!("Could not read command line argument"),
            }
        },
        _ => panic!("Too many args!"),
    };
    
    let path = Path::new(&file_name[..]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read file {}: {}",
                           display,
                           Error::description(&why)),
        Ok(_) => println!("{}",
                          parser::parse_Stmts(&s[..]).unwrap().execute()),
    }
}
