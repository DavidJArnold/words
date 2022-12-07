use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordTree {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub data: char,
    pub id: usize,
    pub depth: i32,
    pub parent: Option<usize>,
    pub terminal: bool,
}

impl WordTree {
    pub fn new(words: Vec<&str>, max_length: Option<i32>) -> WordTree {
        let mut arena = WordTree { nodes: Vec::new() };
        arena.build_word_tree(words, max_length);
        return arena;
    }

    pub fn find_node(&mut self, depth: i32, data: char) -> Vec<usize> {
        // find node by data
        let mut matching_nodes: Vec<usize> = vec![];
        for nodes in &mut self.nodes {
            if nodes.data == data && nodes.depth == depth {
                matching_nodes.push(nodes.id);
            };
        }
        return matching_nodes;
    }

    fn find_node_parent(&mut self, depth: i32, data: char, parent: &usize) -> Option<usize> {
        // find node by data
        for nodes in &mut self.nodes {
            if nodes.data == data && nodes.depth == depth && depth > 0 {
                // println!(
                //     "Found matching node: {:?} {} {}",
                //     nodes.parent, nodes.depth, nodes.data
                // );
                let temp = match nodes.parent {
                    Some(_) => nodes.parent.unwrap(),
                    None => {
                        return None;
                    }
                };
                // println!("Found a candidate node");
                if temp == *parent {
                    return Some(nodes.id);
                };
            };
        }
        return None;
    }

    fn add_node(&mut self, data: char, depth: i32, parent: Option<usize>, terminal: bool) -> usize {
        let id = self.nodes.len();

        let node = Node {
            data,
            id,
            depth,
            parent,
            terminal,
        };

        self.nodes.push(node);

        return id;
    }

    pub fn get_node(&self, id: usize) -> Option<Node> {
        for node in &self.nodes {
            if node.id == id {
                return Some(node.to_owned());
            };
        }
        return None;
    }

    fn make_terminal(&mut self, node_id: usize) {
        let node = &mut self.nodes[node_id];
        node.terminal = true;
    }

    fn get_children(&self, id: usize) -> Vec<usize> {
        let mut children: Vec<usize> = vec![];
        for node in &self.nodes {
            if node.depth > 0 {
                if node.parent.unwrap() == id {
                    children.push(node.id);
                }
            }
        }
        return children;
    }

    pub fn remove_node(&mut self, id: usize) {
        let children = self.get_children(self.get_node(id).unwrap().id);
        let node_idx = self.nodes.iter().position(|x| x.id == id).unwrap();
        self.nodes.remove(node_idx);
        for child in children {
            self.remove_node(child);
        }
    }

    pub fn nodes_at_depth(&self, depth: i32) -> Vec<usize> {
        let mut nodes: Vec<usize> = vec![];
        for node in &self.nodes {
            if node.depth == depth {
                nodes.push(node.id);
            }
        }
        return nodes;
    }

    fn max_depth(&self) -> i32 {
        let mut max_depth = 0;
        for node in &self.nodes {
            if node.depth > max_depth {
                max_depth = node.depth;
            }
        }
        return max_depth;
    }

    fn print_children(&self, node_id: usize, level: i32) {
        let kids = self.get_children(node_id);
        if kids.len() > 0 {
            for child in kids {
                // println!("{:?}", self.get_node(child.id).unwrap());
                print!(
                    "{}{}",
                    "|".repeat((level).try_into().unwrap()),
                    self.get_node(child).unwrap().data
                );
                if self.get_node(child).unwrap().terminal {
                    println!("*");
                } else {
                    println!("");
                }
                self.print_children(child, level + 1);
            }
        }
    }

    pub fn get_words(&self, length: i32) -> Vec<String> {
        // if length is None, get words of all length, otherwise just words of particular length
        let node_list = self.nodes_at_depth(length - 1);
        let mut words: Vec<String> = vec![];
        for node in node_list {
            let mut current_word: String = String::from("");
            if self.get_node(node).unwrap().terminal {
                let mut current_node = self.get_node(node).unwrap();
                for idx in 0..length {
                    current_word.push(current_node.data);
                    if idx + 1 < length {
                        current_node = match current_node.parent {
                            Some(node_id) => self.get_node(node_id).unwrap(),
                            None => break,
                        };
                    }
                    // current_node = self.get_node(.id).unwrap();
                }
            }
            if current_word.len() as i32 == length {
                words.push(current_word);
            }
        }
        let mut rev_words: Vec<String> = vec![];
        for word in &words {
            rev_words.push(word.chars().rev().collect::<String>());
        }
        return rev_words;
    }

    pub fn dbg(&self) {
        for node in &self.nodes {
            println!("{:?}", node);
        }
    }

    pub fn disp(&self) -> &Self {
        let root_nodes = self.nodes_at_depth(0);

        for node in root_nodes {
            if self.get_node(node).unwrap().terminal {
                println!("{}*", self.get_node(node).unwrap().data);
            } else {
                println!("{}", self.get_node(node).unwrap().data);
            }

            let level: i32 = 1;
            self.print_children(node, level);
        }
        return self;
    }

    fn build_word_tree(&mut self, words: Vec<&str>, max_length: Option<i32>) {
        for word in words {
            let mut word_iterator = word.chars();
            match max_length {
                Some(length) => {
                    if word.chars().count() as i32 > length {
                        continue;
                    }
                }
                None => (),
            };
            //if word.chars().count() > max_length {
            //    continue;
            //};
            let ltr = word_iterator.next().unwrap();
            let matched_nodes = self.find_node(0, ltr);

            let first_node: usize = if matched_nodes.len() != 0 {
                matched_nodes[0]
            } else {
                let new_node: usize = self.add_node(ltr, 0, None, false);
                self.get_node(new_node).unwrap().id
            };
            let mut prev_node_id: usize = self.get_node(first_node).unwrap().id;

            for (idx, ltr) in word_iterator.enumerate() {
                let next_node = match self.find_node_parent(idx as i32 + 1, ltr, &mut prev_node_id)
                {
                    Some(node_id) => node_id,
                    None => self.add_node(ltr, idx as i32 + 1, Some(prev_node_id), false),
                };
                prev_node_id = self.get_node(next_node).unwrap().id
            }
            let prev_node = self.get_node(prev_node_id).unwrap();
            self.make_terminal(prev_node.id);
        }
    }

    pub fn word_search(&mut self, pattern: String) -> Vec<String> {
        let mut search_tree = self.clone();
        for (idx, char) in pattern.chars().enumerate() {
            if char != '.' {
                for node in search_tree.nodes_at_depth(idx as i32) {
                    if search_tree.get_node(node).unwrap().data != char {
                        search_tree.remove_node(node);
                    }
                }
            }
        }
        let words = search_tree.get_words(pattern.len() as i32);
        return words;
    }
}
