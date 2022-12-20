
mod brisk_repl;
mod error_handler;
// mod scanner;

use std::env;
use brisk_repl::brisk_repl;

fn main() {
    let args: Vec<String> =  env::args().collect();
    println!("{:?}", args);
    if args.len()  > 1 {
        println!("brisk [file]");
    }

    if args.len() == 2 {
        println!("Main execution block");
    }
    else  {
        brisk_repl()
    }
}
