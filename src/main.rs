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

    // TODO: Add version of this that does the same but keeps ordering from root - down
    fn path(&self, node_index: usize) -> String {
        let mut path = String::new();
        let mut index = node_index;

        loop {
            let node = &self.arena[index];
            path += &format!("_{}", node.name);

            if let Some(parent_index) = node.parent{
                index = parent_index;
            } else {
                break
            }
        }
        return path;
    }

    fn r_path(&self, node_index: usize) -> String {
        let node = &self.arena[node_index];

        // If we are not at the root -> get parent string,
        // add our name to it and return that to lower caller
        if let Some(parent_index) = node.parent{
            let mut path = self.r_path(parent_index);
            path += &format!("_{}", node.name);
            return path;
        } else {
            // We are at the root -> just return the current name
            return String::from(node.name);
        }
    }
}

fn main() {
    let mut tree: Tree = Tree::new();

    let child_1 = tree.add("Child1", tree.root);
    let sub_child_1 = tree.add("SubChild1", child_1);
    let _ = tree.add("SubChild2", child_1);
    let _ = tree.add("Child2", tree.root);
    let _ = tree.add("Child3", tree.root);

    // println!("{:#?}", tree);
    // TODO: add performance benchmark for these two functions
    let node_path = tree.path(tree.arena[sub_child_1].index);
    let r_node_path = tree.r_path(tree.arena[sub_child_1].index);
    println!("{}", node_path);
    println!("{}", r_node_path);
}
