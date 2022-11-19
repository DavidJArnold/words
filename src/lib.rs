mod word_tree;

#[cfg(test)]
mod tests {
    #[test]
    fn it_runs() {
        use crate::word_tree::WordTree;
        let words = vec!["abc", "acc", "abb", "bad", "a", "dab"];
        WordTree::new(words);
        assert!(true);
    }

    #[test]
    fn build_tree() {
        use crate::word_tree::WordTree;
        let words = vec!["abc", "acc", "abb", "bad", "a", "dab"];
        let mut arena = WordTree::new(words);
        let test_node = arena.find_node(0, 'a').unwrap().id;
        assert!(arena.nodes[test_node].id.id == 0 as usize);
        let node_ids = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let depths = vec![0, 1, 2, 1, 2, 2, 0, 1, 2, 0, 1, 2];
        let data = vec!['a', 'b', 'c', 'c', 'c', 'b', 'b', 'a', 'd', 'd', 'a', 'b'];
        let terminal = vec![
            true, false, true, false, true, true, false, false, true, false, false, true,
        ];
        let parent: Vec<i32> = vec![-1, 0, 1, 0, 3, 1, -1, 6, 7, -1, 9, 10];
        for idx in node_ids {
            let node = &arena.nodes[idx];
            assert!(node.depth == depths[idx]);
            assert!(node.data == data[idx]);
            assert!(node.terminal == terminal[idx]);
            match node.parent {
                Some(node_id) => {
                    assert_eq!(node_id.id as i32, parent[idx])
                }
                None => {
                    assert!(parent[idx] == -1)
                }
            }
        }
    }

    #[test]
    fn empty_tree() {
        use crate::word_tree::WordTree;
        let words = vec![];
        let mut _arena = WordTree::new(words);
    }
}
