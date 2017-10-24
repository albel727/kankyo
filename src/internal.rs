use std::io::Read;
use ::Result;

pub fn read_to_string<R: Read>(reader: &mut R) -> Result<String> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;

    Ok(s)
}
