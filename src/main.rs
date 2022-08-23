mod custom_error;
use custom_error::UbungenError;

mod helper_functions;
//use helper_functions::*;

mod phrases;
use phrases::*;

mod words;
use words::*;

use std::env;


//***********************************
fn main() -> Result<(), UbungenError> {
    let p = 'p'.to_string(); 
    let w = 'w'.to_string();
    let q = '?'.to_string();

    let args = env::args().collect::<Vec<String>>();
    
    match args.len() {
        2 => {
            if args[1] == p {
                phrases()?;
            } else if args[1] == w {
                words()?;
            } else if args[1] == q {
                println!("Arguments can only be either p for phrases or w for words.");
            } else {
                println!("Arguments can only be either p for phrases or w for words.");
            }
        },
        _ => println!("Please use one of the arguments: p for phrases, or w for words."),
    };

    Ok(())
}

