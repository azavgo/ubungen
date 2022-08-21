use crate::custom_error::UbungenError;
use crate::helper_functions::*;

pub fn words() -> Result<(), UbungenError> {
    let mut config = Config::new();
    
    let words_de = read_data(&config.words_de)?;
    let words_en = read_data(&config.words_en)?;
        
    println!("Your current words score is {}", &config.score_words);
    config.score_words = lesson_logics(words_de, words_en, config.count, config.score_words);

    config.write_score_words();
    println!("Your current phrases score is {}", &config.score_words);

    Ok(())
}

