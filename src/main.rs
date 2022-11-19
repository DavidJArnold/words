mod word_tree;
use crate::word_tree::WordTree;
use std::fs;

fn main() {
    let file_path = "src/words.txt";

    let contents = fs::read_to_string(file_path).expect("");
    let words = contents.lines().collect::<Vec<&str>>();

    let arena = WordTree::new(words);

    println!("Tree created!");
    arena.dbg();
    arena.disp();
}
