mod anagram;
use bincode;
mod word_tree;
use std::fs;
use std::io::Write;

fn word_search(tree: word_tree::WordTree, pattern: String) -> Vec<String> {
    let mut search_tree = tree.clone();
    if pattern.get(0..0) == String::from(".") {
        let mut node_list = search_tree.nodes;
    } else {
        let mut node_list = search_tree.find_node(0, pt[0]);
    };
    for (idx, char) in pt.chars().enumerate() {
        search_tree.disp();
        println!("Next...");
        if char != '.' {
            for node in search_tree.find_node(idx as i32, char) {
                println!(
                    "{} {:?} {}",
                    char,
                    search_tree.find_node(idx as i32, char),
                    search_tree.get_node(node.id).unwrap().data
                );
                if search_tree.get_node(node.id).unwrap().data != char {
                    search_tree.nodes.remove(node.id);
                }
            }
        }
    }
    search_tree.disp();
    // output_list.push(pt);
    let output_list = vec!["disappointing".to_string()];
    return output_list;
}

fn main() {
    // let words_file = "src/corncob_lowercase.txt";
    let words_file = "src/words.txt";
    let word_tree_file = "tree.hmm";

    if !std::path::Path::new(word_tree_file).exists() {
        let contents = fs::read_to_string(words_file).expect("");
        let words = contents.lines().collect::<Vec<&str>>();

        let arena = word_tree::WordTree::new(words);
        let encoded = bincode::serialize(&arena).unwrap();

        let mut file = fs::File::create(word_tree_file).unwrap();
        file.write_all(&encoded).unwrap();
    }

    let from_file = fs::read(word_tree_file).expect("");
    let arena: word_tree::WordTree = bincode::deserialize(&from_file[..]).unwrap();
    println!("Tree created!");

    let full_word = word_search(arena, "b.c".to_string());
    println!("{:?}", full_word);

    // arena.dbg();`
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
