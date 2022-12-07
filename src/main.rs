mod anagram;
use bincode;
mod word_tree;
use std::fs;
use std::io::Write;

fn main() {
    // corncob list from http://www.mieliestronk.com/wordlist.html
    // let words_file = "src/corncob_lowercase.txt";
    // let word_tree_file = "corncob_tree.hmm";
    // let anagram_filename = "corncob_map.hmm";
    // let max_tree_depth = Some(12);
    let words_file = "src/words.txt";
    let word_tree_file = "words_tree.hmm";
    let anagram_filename = "words_map.hmm";
    let max_tree_depth = Some(4);

    // create the tree if it doesn't exist
    if !std::path::Path::new(word_tree_file).exists() {
        let contents = fs::read_to_string(words_file).expect("");
        let words = contents.lines().collect::<Vec<&str>>();

        let word_tree = word_tree::WordTree::new(words, max_tree_depth);

        let encoded = bincode::serialize(&word_tree).unwrap();
        let mut file = fs::File::create(word_tree_file).unwrap();
        file.write_all(&encoded).unwrap();
        println!("Tree created!");
    }

    // load tree from file
    let from_file = fs::read(word_tree_file).expect("");
    let mut word_tree: word_tree::WordTree = bincode::deserialize(&from_file[..]).unwrap();

    // get all possible 3 letter words
    // let words = arena.get_words(3);
    // println!("3 letter words: {:?}", words);

    // find words fitting pattern
    let pattern = "c.mplic.t.".to_string();
    let full_word = word_tree.word_search(pattern);
    println!("Matched words: {:?}", full_word);

    // create anagram hashmap if the file doesn't exist
    if !std::path::Path::new(anagram_filename).exists() {
        let contents = fs::read_to_string(words_file).expect("");
        let words = contents.lines().collect::<Vec<&str>>();

        let an_map = anagram::anagram(words);

        let encoded = bincode::serialize(&an_map).unwrap();
        let mut file = fs::File::create(anagram_filename).unwrap();
        file.write_all(&encoded).unwrap();
        println!("Anamgram map created!");
    }

    // read anagram map
    let from_file = fs::read(anagram_filename).expect("");
    let decoded: std::collections::HashMap<String, String> =
        bincode::deserialize(&from_file[..]).unwrap();

    // solve an anagram
    let anagrist = "complciated";
    println!(
        "{:?}",
        anagram::solve_anagram(&decoded, anagrist.to_string())
    )
}
