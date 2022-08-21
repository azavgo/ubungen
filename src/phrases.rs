use crate::custom_error::UbungenError;
use crate::helper_functions::*;

pub fn phrases() -> Result<(), UbungenError> {
    let mut config = Config::new();

    let phrases_de = read_data(&config.phrases_de)?;
    let phrases_en = read_data(&config.phrases_en)?;

    println!("Your current phrases score is {}", &config.score_phrases);
    config.score_phrases = lesson_logics(phrases_de, phrases_en, config.count, config.score_phrases); 
    
    config.write_score_phrases();
    println!("Your current phrases score is {}", &config.score_phrases);

    Ok(())
}

 
