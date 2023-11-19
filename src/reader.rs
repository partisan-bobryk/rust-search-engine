use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

pub fn read_content<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
