use text_io::read;

use crate::custom_error::UbungenError;
use crate::helper_functions::*;

pub fn phrases() -> Result<(), UbungenError> {
    let phrases_de = read_data("phrases_de.txt")?;
    let phrases_en = read_data("phrases_en.txt")?;

    lesson_logics(phrases_de, phrases_en); 

    Ok(())
}

 
