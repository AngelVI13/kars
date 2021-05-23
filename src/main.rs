use std::fmt;


#[derive(Debug)]
struct Node<'a> {
	name: &'a str,
    index: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}


impl<'a> fmt::Display for Node<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "N[{}]", self.name);
	}
}

impl<'a> Node<'a> {
    fn new_root(name: &'a str, index: usize) -> Node {
        return Node {
            name,
            index,
            parent: None,
            children: Vec::new(),
        }
    }

    fn new(name: &'a str, index: usize, parent: usize) -> Node {
        return Node {
            name,
            index,
            parent: Some(parent),
            children: Vec::new(),
        }
    }
}


#[derive(Debug)]
struct Tree<'a> {
    arena: Vec<Node<'a>>,
    root: usize
}

impl<'a> Tree<'a> {
    fn new() -> Tree<'a> {
        let root_node = Node::new_root("Root", 0);
        let mut tree: Vec<Node> = Vec::new();
        tree.push(root_node);
        
        return Tree {
            arena: tree,
            root: 0,
        };
    }

    fn add(&mut self, name: &'a str, parent: usize) -> usize {
        let new_element_index = self.arena.len();
        let new_node = Node::new(name, new_element_index, parent);

        self.arena.push(new_node);
        // register child to parent node
        self.arena[parent].children.push(new_element_index);

        return new_element_index;
    }
}

fn main() {
    let mut tree: Tree = Tree::new();

    let child_1 = tree.add("Child1", tree.root);
    let _ = tree.add("SubChild1", child_1);
    let _ = tree.add("SubChild2", child_1);
    let _ = tree.add("Child2", tree.root);
    let _ = tree.add("Child3", tree.root);

    println!("{:#?}", tree);
}
