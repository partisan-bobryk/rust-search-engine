use std::{
    collections::HashMap,
    fs::{self, File},
    io::Result,
    path::PathBuf,
};

use rust_search_engine::{reader::read_content, tokenizer::Lexer};

type TokenFrequency = HashMap<String, usize>;
type TokenFrequencyIndex = HashMap<PathBuf, TokenFrequency>;

fn main() -> Result<()> {
    // Load data in
    let index_path = "index.json";
    let index_file = File::create(index_path)?;

    let dir_path = "sample";
    let dir = fs::read_dir(dir_path)?;

    // Create an index
    let mut global_index: TokenFrequencyIndex = HashMap::new();

    for file in dir {
        let file_path = file?.path();
        println!("Indexing {:?} ...", &file_path);
        let content = read_content(&file_path)?.chars().collect::<Vec<_>>();

        let mut tf: TokenFrequency = HashMap::new();
        let lexer = Lexer::new(&content);

        for token in lexer {
            let term = token
                .iter()
                .map(|t| t.to_ascii_uppercase())
                .collect::<String>();

            if let Some(frequency) = tf.get_mut(&term) {
                *frequency += 1;
            } else {
                tf.insert(term, 0);
            }
        }

        global_index.insert(file_path, tf);
    }

    println!("Saving to index.json");
    serde_json::to_writer(index_file, &global_index)?;

    Ok(())
}
