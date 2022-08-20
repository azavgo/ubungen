use text_io::read;

use crate::custom_error::UbungenError;
use crate::helper_functions::*;

pub fn phrases() -> Result<(), UbungenError> {
    let config = Config::new();

    let phrases_de = read_data(config.phrases_de())?;
    let phrases_en = read_data(config.phrases_en())?;

    lesson_logics(phrases_de, phrases_en, config.count(), config.score()); 

    Ok(())
}

 
