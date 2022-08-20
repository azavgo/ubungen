use text_io::read;

use crate::custom_error::UbungenError;
use crate::helper_functions::*;

pub fn words() -> Result<(), UbungenError> {
    let config = Config::new();
    
    let words_de = read_data(config.words_de())?;
    let words_en = read_data(config.words_en())?;
        
    lesson_logics(words_de, words_en, config.count(), config.score());

    Ok(())
}

