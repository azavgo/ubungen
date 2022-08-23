use std::fs::File; 
use std::io::{BufRead, BufReader}; 
use rand::Rng;
use text_io::read;
use serde_derive::{Deserialize, Serialize};

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
pub fn lesson_logics(deutsch: Vec<String>, english: Vec<String>, count: u32, mut score: i32) -> i32 {
    let english_number = english.len();

    let mut number: usize; 
      
    let mut line = String::new(); 
    let mut line_trim = String::new();

    for _i in 0..count {
        number = random_number(english_number);
        println!("{}", english[number]);    
  
        line = read!("{}\n");
        line_trim = line.trim().to_string();

        //the score logics
        if &line_trim != &deutsch[number] {
            score -= 1;
        } else {
            score += 1;
        }

        while &line_trim != &deutsch[number] {
            println!("{}", &deutsch[number]);
            line = read!("{}\n");
            line_trim = line.trim().to_string();
        }
    }
    score
}
//****************************** 
//config.json reads into Struct
//"count": 1 - means how many words or phrases to learn in one lesson
//"score": 0 - means +1 for success at the first attempt, -1 if the first attempt fails
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub words_de: String, 
    pub words_en: String,
    pub phrases_de: String, 
    pub phrases_en: String, 
    pub count: u32, 
    pub score_phrases: i32,
    pub score_words: i32,
    pub session_count: usize,
}

impl Config {
  pub fn new() -> Self {
    let json_config = std::fs::read_to_string("config.json").unwrap(); 
    serde_json::from_str::<Self>(&json_config).unwrap()
  }
  
  pub fn write_score_phrases(self: &Self) {
    std::fs::write("config.json", serde_json::to_string_pretty(self).unwrap()).unwrap();
  }

  pub fn write_score_words(self: &Self) {
    std::fs::write("config.json", serde_json::to_string_pretty(self).unwrap()).unwrap();
  }
}
//******************************** 
//ConfigPhrases struct keeps records for each phrase ever used
pub struct ConfigPharses {
  pub phrase_number: usize, 
  pub phrase_score: u8,
  pub phrase_session: usize,
}

impl ConfigPharses {
  pub fn new(phrase_number: usize, phrase_score: u8, phrase_session: usize) -> Self {
    Self {
      phrase_number: phrase_number, 
      phrase_score: phrase_score, 
      phrase_session: phrase_session,
    }
  }
}

//**************************