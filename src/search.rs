use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use crate::store::{full_path, open_read, self};


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



pub fn search(query: Vec<String>, n: usize) -> Vec<String> {
    let stop_words = load_stopwords();
    let index = index::load();
    let result_indexes = index::search(query, &stop_words, &index);
    
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

pub mod index {
    use std::{collections::{HashMap, HashSet}, fs::{File, OpenOptions}, io::{BufReader, BufWriter, BufRead}, path::PathBuf};

    use bincode;

    use crate::helpers::split_line;
    use crate::store::{open_or_create, full_path};

    use super::load_stopwords;

    const INDEX_FILENAME: &str = "index.txt";

    pub fn load() -> HashMap<String, Vec<u16>> {
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

    pub fn save(index: &HashMap<String, Vec<u16>>) {
        let filepath = full_path(INDEX_FILENAME);
        let file = open_or_create(filepath, false).expect("Could not create index file");
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, index).expect("Could not serialize index file");
    }

    fn clean<S: ToString>(word: S) -> String {
        word
            .to_string()
            .trim()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }

    pub fn add_word<S: ToString>(word: S, line_number: u16, index: &mut HashMap<String, Vec<u16>>) {
        let word = clean(word);
        let line_numbers = index.entry(word).or_insert(Vec::new());
        line_numbers.push(line_number);
    }

    fn _add_line<'a>(line_number: u16, line: &str, stop_words: &HashSet<String>, index: &'a mut HashMap<String, Vec<u16>>) -> &'a HashMap<String, Vec<u16>> {
        let message = match split_line(line) {
            Some((_, message)) => message,
            None => return index,
        };

        for word in message.split_whitespace() {
            if stop_words.contains(word) {
                continue;
            }
            add_word(word, line_number, index);
        }
        index
    }

    pub fn add_line(line_number: u16, line: &str) {
        let mut index = load();
        let stop_words = load_stopwords();
        _add_line(line_number, line, &stop_words, &mut index);
        save(&index);
    }
    
    fn build_from_lines<T: IntoIterator<Item = String>>(lines: T) -> HashMap<String, Vec<u16>> {
        let mut index = HashMap::new();
        let stop_words = load_stopwords();

        for (line_number, line) in lines.into_iter().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            println!("Processing line {}", line);
            _add_line(line_number as u16, line, &stop_words, &mut index);
        }

        index
    }

    pub fn build<PB: Into<PathBuf>>(store_filename: PB) -> HashMap<String, Vec<u16>> {
        let store_filename = store_filename.into();
        let filepath = full_path(store_filename);
        let file = File::open(filepath).expect("Could not open store file");
        let reader = BufReader::new(file);
        let lines = reader.lines().filter_map(|l| l.ok());

        build_from_lines(lines)
    }

    fn _lookup_word(word: &str, index: &HashMap<String, Vec<u16>>) -> Vec<u16> {
        let word = clean(word);
        match index.get(&word) {
            Some(line_numbers) => line_numbers.clone(),
            None => Vec::new(),
        }
    }

    pub fn search(
            query: Vec<String>, 
            stop_words: &HashSet<String>, 
            index: &HashMap<String, Vec<u16>>,
        ) -> Vec<u16> {
    
    let mut ocurrences = Vec::new();

    for word in query {
        let word = clean(word);
        if stop_words.contains(&word) {
            continue;
        }

        let index_hits = _lookup_word(&word, index);
        ocurrences.extend(index_hits);
    }

    let mut counts: HashMap<u16, u16> = HashMap::new();
    for line_number in ocurrences {
        let count = counts.get(&line_number).unwrap_or(&0);

        counts.insert(line_number, count + 1);
    }


    let mut counts: Vec<(u16, u16)> = counts.into_iter().collect();
    
    // reverse sort
    fn negate(x: &u16) -> i32 {-(*x as i32)}
    counts.sort_by_key(|(_, count)| negate(count));
    counts.iter().map(|(line_number, _)| *line_number).collect()
        
    }
    
    #[cfg(test)]
    mod tests {
        use crate::search::{index};
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

            let idx = index::build_from_lines(lines);
            println!("{:?}", &idx);
            assert!(idx.contains_key("message"));
            assert!(idx.contains_key("three"));
            assert!(idx.contains_key("whatever"));
            assert!(idx.contains_key("nothing"));
            assert_eq!(idx.get("message").unwrap().len(), 6);
            assert_eq!(idx.len(), 11);
        }

        #[test]
        fn search_indexes_works() {
            let lines: Vec<String> = INDEX_FILE.lines().map(|l| l.to_string()).collect();

            let idx = index::build_from_lines(lines.clone());
            let query = vec!["message".to_string(), "three".to_string()];
            let mut stop_words = HashSet::new();
            stop_words.insert("the".to_string());

            let results = search(query, &stop_words,&idx);
            assert!(results.len() > 0);
            assert!(results.len() <= lines.len());
            println!("{:?}", &results);
            // assert!(false);

            let query = vec!["nothing".to_string()];
            let results = search(query, &stop_words,&idx);
            assert!(results.len() == 1);

            let query = vec!["three".to_string()];
            let results = search(query, &stop_words,&idx);
            assert!(results.len() == 2);


        }

    }

}


