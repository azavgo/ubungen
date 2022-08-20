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
pub fn lesson_logics(deutsch: Vec<String>, english: Vec<String>, count: u32, mut score: i32) {
    let english_number = english.len();

    let mut number: usize; 
      
    let mut line = String::new(); 
    let mut line_trim = String::new();

    for _i in 0..count {
        number = random_number(english_number);
        println!("{}", english[number]);    
  
        line = read!("{}\n");
        line_trim = line.trim().to_string();

        while &line_trim != &deutsch[number] {
            score -= 1; 
            println!("{}", &deutsch[number]);
            line = read!("{}\n");
            line_trim = line.trim().to_string();
        }
    }
}
//****************************** 
//config.json reads into Struct
//"count": 1 - means how many words or phrases to learn in one lesson
//"score": 0 - means +1 for success at the first attempt, -1 if the first attempt fails
pub struct Config<'a> {
    words_de: &'a str, 
    words_en: &'a str,
    phrases_de: &'a str, 
    phrases_en: &'a str, 
    count: u32, 
    score: i32,
}

impl Config<'static> {
  pub fn new() -> Self {
    Self {
        words_de: "words_de.txt", 
        words_en: "words_en.txt",
        phrases_de: "phrases_de.txt", 
        phrases_en: "phrases_en.txt", 
        count: 1, 
        score: 0,
    }
  }  
  
  pub fn words_de(self: &Self) -> &str {
    self.words_de
  }

  pub fn words_en(self: &Self) -> &str {
    self.words_en
  }

  pub fn phrases_de(self: &Self) -> &str {
    self.phrases_de
  }

  pub fn phrases_en(self: &Self) -> &str {
    self.phrases_en
  }

  pub fn count(self: &Self) -> u32 {
    self.count
  }

  pub fn score(self: &Self) -> i32 {
    self.score
  }
}

//******************************** 