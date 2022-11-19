mod anagram;
mod word_tree;
use crate::word_tree::WordTree;
use std::fs;

fn main() {
    let file_path = "src/words.txt";

    let contents = fs::read_to_string(file_path).expect("");
    let words = contents.lines().collect::<Vec<&str>>();
    let words2 = contents.lines().collect::<Vec<&str>>();

    let arena = WordTree::new(words);

    println!("Tree created!");
    arena.dbg();
    arena.disp();

    let an_map = anagram::anagram(words2);
    let anagrist = "cba";
    println!(
        "{:?}",
        anagram::solve_anagram(&an_map, anagrist.to_string())
    )
}
