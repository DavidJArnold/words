mod word_tree;

use crate::word_tree::WordTree;
// pub mod word_tree;
fn main() {
    let words = vec!["abc", "acc", "abb", "bad"];
    let arena = WordTree::new(words);

    arena.disp();
}
