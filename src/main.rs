mod anagram;
mod word_tree;
use crate::word_tree::WordTree;
use bincode;
use std::fs;
use std::io::Write;

fn main() {
    let words_file = "src/corncob_lowercase.txt";
    // let words_file = "src/words.txt";
    let word_tree_file = "tree.hmm";

    if !std::path::Path::new(word_tree_file).exists() {
        let contents = fs::read_to_string(words_file).expect("");
        let words = contents.lines().collect::<Vec<&str>>();

        let arena = WordTree::new(words);
        let encoded = bincode::serialize(&arena).unwrap();

        let mut file = fs::File::create(word_tree_file).unwrap();
        file.write_all(&encoded).unwrap();
    }

    let from_file = fs::read(word_tree_file).expect("");
    let arena: WordTree = bincode::deserialize(&from_file[..]).unwrap();
    println!("Tree created!");
    // arena.dbg();
    // arena.disp();

    let anagram_filename = "anagram_map.hmm";
    if !std::path::Path::new(anagram_filename).exists() {
        let contents = fs::read_to_string(words_file).expect("");
        let words = contents.lines().collect::<Vec<&str>>();
        let an_map = anagram::anagram(words);
        let encoded = bincode::serialize(&an_map).unwrap();

        let mut file = fs::File::create(anagram_filename).unwrap();
        file.write_all(&encoded).unwrap();
    }

    let from_file = fs::read(anagram_filename).expect("");
    let decoded: std::collections::HashMap<String, String> =
        bincode::deserialize(&from_file[..]).unwrap();

    let anagrist = "complciated";
    println!(
        "{:?}",
        anagram::solve_anagram(&decoded, anagrist.to_string())
    )
}
