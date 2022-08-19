use text_io::read;

use crate::custom_error::UbungenError;
use crate::helper_functions::*;

pub fn words() -> Result<(), UbungenError> {
    let words_de = read_data("words_de.txt")?;
    let words_en = read_data("words_en.txt")?;    
    let number = random_number(words_en.len());

    
    println!("{}", words_en[number]);
    let mut line: String = read!("{}\n");
    let mut line_trim = line.trim().to_string(); 

    while line != words_de[number] {
        println!("{}", words_de[number]);
        line = read!("{}\n");
        line_trim = line.trim().to_string();
    }

    Ok(())
}

