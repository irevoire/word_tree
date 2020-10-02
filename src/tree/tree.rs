use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    c: char,
    is_word: bool,
    childs: HashMap<char, Node>,
}

impl Node {
    // Creates a new Node, with the given character
    fn new(c: char) -> Node {
        Node {
            c: c,
            is_word: false,
            childs: HashMap::new(),
        }
    }

    // Recursively generates the dot labels for the node
    fn generate_labels(&self, depth: u8) {
        let mut color = "grey";
        if self.is_word {
            color = "green";
        }
        println!("{}{} [label={}, color={}]", self.c, depth, self.c, color);
        for (_, v) in &self.childs {
            v.generate_labels(depth+1);
        }
    }

    // Serializes the Node as a dot link
    fn to_graph(&self, depth: u8) {
        if self.childs.is_empty() {
            println!("{}{};", self.c, depth);
        } else {
            for (_, v) in &self.childs {
                print!("{}{} -> ", self.c, depth);
                &v.to_graph(depth+1);
            }
        }
    }
}

#[derive(Debug)]
pub struct TreeWord {
    title: String,
    childs: HashMap<char, Node>,
}


impl TreeWord {
    // Creates a new TreeWord with the given title
    pub fn new(title: String) -> TreeWord {
        TreeWord {
            title: title,
            childs: HashMap::new(),
        }
    }

    // Adds a word to the TreeWord
    pub fn add_word(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }
        let first_char: char = word.chars().nth(0).unwrap();
        let mut node: &mut Node = self.childs.entry(first_char).or_insert(Node::new(first_char));
        for c in word.chars().skip(1) {
            node = node.childs.entry(c).or_insert(Node::new(c));
        }
        node.is_word = true;
    }

    // Prints the TreeWord
    pub fn print(self) {
        println!("{:?}", self);
    }

    // Generates the labels for the entire TreeWord
    fn generate_labels(&self) {
        for (_, v) in &self.childs {
            &v.generate_labels(0);
        }
    }

    // Serializes the entires TreeWord as a dot file
    pub fn to_graph(&self) {
        println!("digraph {} {{", self.title);
        self.generate_labels();
        for (_, v) in &self.childs {
            v.to_graph(0);
        }
        println!("}}");
    }
}
