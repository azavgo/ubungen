use std::fs::File; 
use std::io::{BufRead, BufReader}; 
use rand::Rng;
use text_io::read;

use crate::custom_error::UbungenError;

//*****************************************
//function to generate a random number between 0 and number - 1
pub fn random_number(number: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..number)
}

//*****************************************
//function to read the content of the text file line by line
pub fn read_data(input: &str) -> Result<Vec<String>, UbungenError> {
    let f = File::open(input)?; 
    let br = BufReader::new(f); 
    let phrases_de: Vec<String> = br.lines().into_iter().map(|e| e.unwrap()).collect();
    Ok(phrases_de)
}
//*************************************
//function to drive the logics of the lesson
pub fn lesson_logics(deutsch: Vec<String>, english: Vec<String>) {
    let english_number = english.len();

    let mut number: usize; 
      
    let mut line = String::new(); 
    let mut line_trim = String::new();

    for _i in 0..2 {
        number = random_number(english_number);
        println!("{}", english[number]);    
  
        line = read!("{}\n");
        line_trim = line.trim().to_string();

        while &line_trim != &deutsch[number] {
            println!("{}", &deutsch[number]);
            line = read!("{}\n");
            line_trim = line.trim().to_string();
        }
    }
}
//****************************** 
