use std::fs::File; 
use std::io::{BufRead, BufReader}; 
use rand::Rng;
use text_io::read;

//***********************************
fn main() -> Result<(), UbungenError> {
    let phrases_de = read_data("phrases_de.txt")?;
    let phrases_en = read_data("phrases_en.txt")?;    
    let number = random_number(phrases_en.len());

    println!("{}", phrases_en[number]);
    let mut line: String = read!("{}\n");
    while line != phrases_de[number] {
        println!("{}", phrases_de[number]);
        line = read!("{}\n");
    }
    println!("{}", phrases_de[number]);
    
    Ok(())
}

//*****************************************
//function to generate a random number between 0 and length of a vector - 1
fn random_number(number: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..number)
}

//*****************************************
//function to read the content of the text file line by line
fn read_data(input: &str) -> Result<Vec<String>, UbungenError> {
    let f = File::open(input)?; 
    let br = BufReader::new(f); 
    let phrases_de: Vec<String> = br.lines().into_iter().map(|e| e.unwrap()).collect();
    Ok(phrases_de)
}
//*************************************
//Custom error
#[derive(Debug)]
pub enum UbungenError { 
    IOError(std::io::Error),  
}

impl From<std::io::Error> for UbungenError {
    fn from(error: std::io::Error) -> Self {
        UbungenError::IOError(error)
    }
}
