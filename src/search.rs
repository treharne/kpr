use std::collections::{HashSet, HashMap};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;

use bincode;
extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};

use crate::store::{full_path, open_read, self};
use crate::helpers::split_line;
use crate::store::open_or_create;

const INDEX_FILENAME: &str = "index.txt";
const STOPS_FILENAME: &str = "stopwords.txt";

pub fn load_stopwords() -> HashSet<String> {
    let filepath = full_path(STOPS_FILENAME);
    let file = open_read(filepath).expect("Could not open stopwords file");
    let reader = BufReader::new(file);
    let mut stopwords = HashSet::new();

    for line in reader.lines() {
        let line = line.expect("Could not parse line");
        stopwords.insert(line.trim().to_string());
    }

    stopwords
}


pub fn search(query: &[String], n: usize) -> Vec<String> {
    let index = Index::load();
    let result_indexes = index.search(query);
    
    let lines = store::load_lines(None);

    let get_line = |line_number: u16| -> Option<String> {
        let line = lines.get(line_number as usize);
        Some(line?.to_string())
    };

    result_indexes
        .into_iter()
        .take(n)
        .filter_map(get_line)
        .collect()

}

pub struct Index {
    index: HashMap<String, Vec<u16>>,
    stop_words: HashSet<String>,
    stemmer: Stemmer,
}

impl Index {

    fn default() -> Self {
        Self::load()
    }

    pub fn load() -> Self {
        Index {
            index: Self::load_index(),
            stop_words : load_stopwords(),
            stemmer: Stemmer::create(Algorithm::English),
        }
    }

    fn from_lines(lines: impl IntoIterator<Item=String>) -> Self {
        let mut index = Self {
            index: HashMap::new(),
            stop_words : load_stopwords(),
            stemmer: Stemmer::create(Algorithm::English),
        };

        for (line_number, line) in lines.into_iter().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            index.add_line(line_number as u16, line);
        }
        index
    }

    pub fn from_store_path(store_filename: impl Into<PathBuf>) -> Self {
        let store_filename = store_filename.into();
        let filepath = full_path(store_filename);
        let file = File::open(filepath).expect("Could not open store file");
        let reader = BufReader::new(file);
        let lines = reader.lines().filter_map(Result::ok);

        Self::from_lines(lines)
    }

    fn clean(&self, word: &str) -> String {
        let lower = 
            word
            .trim()
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        self.stemmer.stem(&lower).to_string()
    }   

    fn add_word(&mut self, word: &str, line_number: u16) {

        let word = self.clean(word);
        let line_numbers = self.index
            .entry(word)
            .or_default();
        line_numbers.push(line_number);
    }

    pub fn add_line(&mut self, line_number: u16, line: &str) {
        let Some((_, message)) = split_line(line) else { return };
        for word in message.split_whitespace() {
            if self.stop_words.contains(word) {
                continue;
            }
            self.add_word(word, line_number);
        }
    }

    fn lookup_word(&self, word: &str) -> Vec<u16> {
        let word = self.clean(word);
        println!("Looking up word: {:?}", &word);
        println!("Index: {:?}", &self.index);
        match self.index.get(&word) {
            Some(line_numbers) => line_numbers.clone(),
            None => Vec::new(),
        }
    }

    pub fn save(&self) {
        let filepath = full_path(INDEX_FILENAME);
        let file = open_or_create(filepath, false).expect("Could not create index file");
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self.index).expect("Could not serialize index file");

    }

    pub fn load_index() -> HashMap<String, Vec<u16>> {
        let filepath = full_path(INDEX_FILENAME);

        let file = OpenOptions::new()
            .read(true)
            .open(filepath)
            .expect("Could not open index file");

        if file.metadata().expect("Could not get metadata").len() == 0 {
            return HashMap::new()
        };

        let reader = BufReader::new(file);
        bincode::deserialize_from(reader).expect("Could not deserialize index file")
    }

    pub fn search(&self, query: &[String]) -> Vec<u16> {

        let mut ocurrences = Vec::new();

        for word in query {
            let word = self.clean(word);
            if self.stop_words.contains(&word) {
                continue;
            }

            let index_hits = self.lookup_word(&word);
            ocurrences.extend(index_hits);
        }

        let mut counts: HashMap<u16, u16> = HashMap::new();
        for line_number in ocurrences {
            let count = counts.get(&line_number).unwrap_or(&0);

            counts.insert(line_number, count + 1);
        }

        let mut counts: Vec<(u16, u16)> = counts.into_iter().collect();
        
        // reverse sort
        counts.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        counts.into_iter().map(|(line_number, _)| line_number).collect()
        
    }
}
    

// }

#[cfg(test)]
mod tests {
    use super::*;
    const INDEX_FILE: &'static str = "
    1680917693908: message two
    1680917698382: message 3
    1680917701962: message three
    1680917704320: message
    1680917706282: three
    1680917709913: whatever
    1680917717166: Nothing... I think
    1680917722186: Another message
    1680917729297: one more message
    1680917733553: one more time
";

    #[test]
    fn build_index_works() {
        let lines = INDEX_FILE.lines().map(|l| l.to_string());

        let index = Index::from_lines(lines);
        assert_ne!(index.lookup_word("message").len(), 0);
        assert_ne!(index.lookup_word("three").len(), 0);
        assert_ne!(index.lookup_word("whatever").len(), 0);
        assert_ne!(index.lookup_word("nothing").len(), 0);
        assert_eq!(index.lookup_word("message").len(), 6);
        assert_eq!(index.index.len(), 11)
    }

    #[test]
    fn search_indexes_works() {
        let lines: Vec<String> = INDEX_FILE.lines().map(|l| l.to_string()).collect();

        let index = Index::from_lines(lines.clone());
        let query = vec!["message".to_string(), "three".to_string()];
        let mut stop_words = HashSet::new();
        stop_words.insert("the".to_string());

        let results = index.search(&query);
        assert!(results.len() > 0);
        assert!(results.len() <= lines.len());
        println!("{:?}", &results);
        // assert!(false);
        
        let query = vec!["nothing".to_string()];
        let results = index.search(&query);
        println!("{:?}", &results);
        println!("{:?}", &index.index);

        assert!(results.len() == 1);
        
        let query = vec!["three".to_string()];
        let results = index.search(&query);
        assert!(results.len() == 2);


    }

}


