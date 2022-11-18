#[derive(Debug, Clone)]
pub struct WordTree {
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct NodeId {
    id: usize,
}

#[derive(Debug, Clone)]
pub struct Node {
    data: char,
    id: NodeId,
    depth: i32,
    parent: Option<NodeId>,
}

impl WordTree {
    pub fn new(words: Vec<&str>) -> WordTree {
        let mut arena = WordTree { nodes: Vec::new() };
        arena.build_word_tree(words);
        return arena;
    }

    pub fn find_node(&mut self, depth: i32, data: char) -> Option<NodeId> {
        // find node by data
        for nodes in &mut self.nodes {
            if nodes.data == data && nodes.depth == depth {
                return Some(nodes.id);
            };
        }
        return None;
    }

    pub fn find_node_parent(
        &mut self,
        depth: i32,
        data: char,
        parent: &mut NodeId,
    ) -> Option<NodeId> {
        // find node by data
        for nodes in &mut self.nodes {
            if nodes.data == data && nodes.depth == depth - 1 && depth > 0 {
                println!("{:?} {} {}", nodes.parent, nodes.depth, nodes.data);
                let temp = match nodes.parent {
                    Some(_) => nodes.parent.unwrap(),
                    None => {
                        return None;
                    }
                };
                if temp == *parent {
                    return Some(nodes.id);
                };
            };
        }
        return None;
    }

    pub fn add_node(&mut self, data: char, depth: i32, parent: Option<NodeId>) -> usize {
        let index = self.nodes.len();
        let node_id = NodeId { id: index };

        let node = Node {
            data: data,
            id: node_id,
            depth: depth,
            parent: parent,
        };

        self.nodes.push(node);

        return index;
    }

    fn get_node(&self, id: usize) -> Option<Node> {
        for node in &self.nodes {
            if node.id.id == id {
                return Some(node.to_owned());
            };
        }
        return None;
    }

    pub fn get_children(&self, id: usize) -> Vec<NodeId> {
        let mut children: Vec<NodeId> = vec![];
        for node in &self.nodes {
            if node.depth > 0 {
                if node.parent.unwrap().id == id {
                    children.push(node.id);
                }
            }
        }
        return children;
    }

    pub fn nodes_at_depth(&self, depth: i32) -> Vec<NodeId> {
        let mut nodes: Vec<NodeId> = vec![];
        for node in &self.nodes {
            if node.depth == depth {
                nodes.push(node.id);
            }
        }
        return nodes;
    }

    pub fn max_depth(&self) -> i32 {
        let mut max_depth = 0;
        for node in &self.nodes {
            if node.depth > max_depth {
                max_depth = node.depth;
            }
        }
        return max_depth;
    }

    pub fn print_children(&self, node: NodeId, level: i32) {
        let kids = self.get_children(node.id);
        if kids.len() > 0 {
            for child in kids {
                println!(
                    "{}|-{}",
                    "|".repeat((level - 1).try_into().unwrap()),
                    self.get_node(child.id).unwrap().data
                );
                self.print_children(child, level + 1)
            }
        }
    }

    pub fn disp(&self) {
        let root_nodes = self.nodes_at_depth(0);

        for node in root_nodes {
            println!("{}", self.get_node(node.id).unwrap().data);
            let level: i32 = 1;
            self.print_children(node, level);
        }
    }

    pub fn build_word_tree(&mut self, words: Vec<&str>) {
        for word in words {
            println!("{}", word);

            let mut word_iterator = word.chars();
            let ltr = word_iterator.next().unwrap();

            println!("First letter {}", ltr);
            // first node
            let first_node = match self.find_node(0, ltr) {
                Some(node_id) => node_id.id,
                None => {
                    let new_node = self.add_node(ltr, 0, None);
                    self.get_node(new_node).unwrap().id.id
                }
            };
            println!("Just created {}", first_node);
            println!("Now the arena looks like: {:?}", self);
            // thereafter...
            let mut prev_node = self.get_node(first_node).unwrap().id;

            for (idx, ltr) in word_iterator.enumerate() {
                println!("next letter {} at depth {}", ltr, idx as i32 + 1);
                let next_node = match self.find_node_parent(
                    idx as i32,
                    self.get_node(prev_node.id).unwrap().data,
                    &mut prev_node,
                ) {
                    Some(node_id) => node_id.id,
                    None => self.add_node(ltr, idx as i32 + 1, Some(prev_node)),
                };
                println!("Just created {}", next_node);
                println!("Now the arena looks like: {:?}", self);
                println!("Get the NodeId {:?}", self.get_node(next_node));
                prev_node = self.get_node(next_node).unwrap().id;
            }
            println!("Done that one");
        }
    }
}
