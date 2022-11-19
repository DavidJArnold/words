mod word_tree;
use crate::word_tree::WordTree;

fn main() {
    let words = vec!["abc", "acc", "abb", "bad", "a", "dab"];
    let arena = WordTree::new(words);

    println!("Tree created!");
    arena.dbg();
    arena.disp();
}
